-- Add lot_id to tickets and enforce lot capacity + auto-progress sellability at SQL level.

ALTER TABLE tickets
  ADD COLUMN IF NOT EXISTS lot_id uuid;

ALTER TABLE tickets
  ADD CONSTRAINT IF NOT EXISTS tickets_lot_id_fk
  FOREIGN KEY (lot_id) REFERENCES ticket_lots (id) ON DELETE RESTRICT;

CREATE INDEX IF NOT EXISTS tickets_lot_id_idx ON tickets (lot_id);

-- Guard: ensure the ticket's event_id matches the lot.event_id,
-- and ensure the lot is currently sellable and not over max_tickets.
CREATE OR REPLACE FUNCTION tickets_before_insert_guard()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  lot_event_id uuid;
  sold int;
  max_tickets int;
BEGIN
  IF NEW.lot_id IS NULL THEN
    RAISE EXCEPTION 'lot_id is required';
  END IF;

  SELECT event_id INTO lot_event_id
  FROM ticket_lots
  WHERE id = NEW.lot_id;

  IF lot_event_id IS NULL THEN
    RAISE EXCEPTION 'invalid lot_id %', NEW.lot_id;
  END IF;

  IF lot_event_id <> NEW.event_id THEN
    RAISE EXCEPTION 'ticket event_id % does not match lot event_id %', NEW.event_id, lot_event_id;
  END IF;

  IF NOT ticket_lot_is_sellable(NEW.lot_id) THEN
    RAISE EXCEPTION 'ticket lot % is not sellable', NEW.lot_id;
  END IF;

  SELECT s.sold, s.max_tickets INTO sold, max_tickets
  FROM ticket_lot_sales s
  WHERE s.lot_id = NEW.lot_id;

  IF sold >= max_tickets THEN
    RAISE EXCEPTION 'ticket lot % sold out (% / %)', NEW.lot_id, sold, max_tickets;
  END IF;

  RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS tickets_before_insert_guard_trg ON tickets;
CREATE TRIGGER tickets_before_insert_guard_trg
BEFORE INSERT ON tickets
FOR EACH ROW
EXECUTE FUNCTION tickets_before_insert_guard();

-- Finally, make lot_id required.
ALTER TABLE tickets
  ALTER COLUMN lot_id SET NOT NULL;
