-- Ticket lots ("lotes") per event.
-- Supports N+1 lots: lote 1,2,3... where each lot can sell up to max_tickets.
--
-- Important: "sell_enabled" is the manual toggle.
-- The *effective* sellability can be computed at SQL level (see view + functions below):
-- if all previous lots are sold out, the next lot becomes sellable automatically.

CREATE TABLE IF NOT EXISTS ticket_lots (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),

  event_id uuid NOT NULL REFERENCES events (id) ON DELETE CASCADE,

  -- ordering: 1,2,3...
  position int NOT NULL CHECK (position >= 1),

  name text,

  max_tickets int NOT NULL CHECK (max_tickets >= 0),

  -- manual toggle (user-controlled)
  sell_enabled boolean NOT NULL DEFAULT true,

  created_at timestamptz NOT NULL DEFAULT now (),
  updated_at timestamptz NOT NULL DEFAULT now (),

  UNIQUE (event_id, position)
);

CREATE INDEX IF NOT EXISTS ticket_lots_event_id_idx ON ticket_lots (event_id);
CREATE INDEX IF NOT EXISTS ticket_lots_event_pos_idx ON ticket_lots (event_id, position);

-- Helper view: sold counts per lot (counts rows in tickets).
-- Note: for now we count ALL tickets as sold. If later you add "refunded/cancelled",
-- change the WHERE clause here + in the trigger.
CREATE OR REPLACE VIEW ticket_lot_sales AS
SELECT
  l.id AS lot_id,
  l.event_id,
  l.position,
  l.max_tickets,
  l.sell_enabled,
  COUNT(t.id)::int AS sold
FROM ticket_lots l
LEFT JOIN tickets t ON t.lot_id = l.id
GROUP BY l.id;

-- Compute whether a lot is sold out.
CREATE OR REPLACE FUNCTION ticket_lot_is_sold_out(p_lot_id uuid)
RETURNS boolean
LANGUAGE sql
STABLE
AS $$
  SELECT (s.sold >= s.max_tickets) FROM ticket_lot_sales s WHERE s.lot_id = p_lot_id;
$$;

-- Compute whether a lot is effectively sellable.
-- Rule requested:
-- - If all previous lots are sold out, this lot becomes sellable automatically,
--   even if sell_enabled=false.
CREATE OR REPLACE FUNCTION ticket_lot_is_sellable(p_lot_id uuid)
RETURNS boolean
LANGUAGE sql
STABLE
AS $$
  WITH cur AS (
    SELECT event_id, position, sell_enabled
    FROM ticket_lots
    WHERE id = p_lot_id
  )
  SELECT
    (
      -- either manually enabled...
      cur.sell_enabled
      OR
      -- ...or all previous lots are sold out
      COALESCE((
        SELECT bool_and(ticket_lot_is_sold_out(l2.id))
        FROM ticket_lots l2
        WHERE l2.event_id = cur.event_id
          AND l2.position < cur.position
      ), true)
    )
    AND (NOT ticket_lot_is_sold_out(p_lot_id));
$$;

-- Returns the current sellable lot for an event.
-- Picks the lowest position lot that is sellable.
CREATE OR REPLACE FUNCTION ticket_lot_current_for_event(p_event_id uuid)
RETURNS uuid
LANGUAGE sql
STABLE
AS $$
  SELECT l.id
  FROM ticket_lots l
  WHERE l.event_id = p_event_id
    AND ticket_lot_is_sellable(l.id)
  ORDER BY l.position ASC
  LIMIT 1;
$$;
