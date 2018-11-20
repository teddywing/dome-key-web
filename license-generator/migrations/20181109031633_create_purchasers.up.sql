BEGIN;

CREATE TABLE purchasers (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    secret VARCHAR(255),
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TRIGGER purchasers_created_at_before_insert
    BEFORE INSERT
    ON purchasers FOR EACH ROW
        SET NEW.created_at = UTC_TIMESTAMP();

CREATE TRIGGER purchasers_updated_at_before_insert
    BEFORE INSERT
    ON purchasers FOR EACH ROW
        SET NEW.updated_at = UTC_TIMESTAMP();

CREATE TRIGGER purchasers_updated_at_before_update
    BEFORE UPDATE
    ON purchasers FOR EACH ROW
        SET NEW.updated_at = UTC_TIMESTAMP();

COMMIT;
