BEGIN;

DROP TRIGGER purchasers_before_insert;
DROP TRIGGER purchasers_before_update;
DROP TABLE purchasers;

COMMIT;
