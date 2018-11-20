BEGIN;

DROP TRIGGER purchasers_created_at_before_insert;
DROP TRIGGER purchasers_updated_at_before_insert;
DROP TRIGGER purchasers_updated_at_before_update;
DROP TABLE purchasers;

COMMIT;
