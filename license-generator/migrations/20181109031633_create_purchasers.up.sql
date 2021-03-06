-- Copyright (c) 2018  Teddy Wing
--
-- This file is part of DomeKey Web.
--
-- DomeKey Web is free software: you can redistribute it and/or modify it
-- under the terms of the GNU Affero General Public License as published
-- by the Free Software Foundation, either version 3 of the License, or
-- (at your option) any later version.
--
-- DomeKey Web is distributed in the hope that it will be useful, but
-- WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
-- Affero General Public License for more details.
--
-- You should have received a copy of the GNU Affero General Public
-- License along with DomeKey Web. If not, see
-- <https://www.gnu.org/licenses/>.

BEGIN;

CREATE TABLE purchasers (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    secret VARCHAR(255),
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TRIGGER purchasers_before_insert
    BEFORE INSERT
    ON purchasers FOR EACH ROW
        SET
            NEW.updated_at = UTC_TIMESTAMP(),
            NEW.created_at = UTC_TIMESTAMP();

CREATE TRIGGER purchasers_before_update
    BEFORE UPDATE
    ON purchasers FOR EACH ROW
        SET NEW.updated_at = UTC_TIMESTAMP();

COMMIT;
