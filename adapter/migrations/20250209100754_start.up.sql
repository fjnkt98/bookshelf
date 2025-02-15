-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS '
BEGIN
    new.updated_at := ''now'';
    return new;
END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS roles (
    role_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    FOREIGN KEY (role_id) REFERENCES roles (role_id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TRIGGER users_updated_at_trigger BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE set_updated_at();

CREATE TABLE IF NOT EXISTS books (
    book_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    isbn TEXT NOT NULL,
    description TEXT NOT NULL,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    FOREIGN KEY (user_id) REFERENCES users (user_id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TRIGGER books_updated_at_trigger BEFORE UPDATE ON books FOR EACH ROW EXECUTE PROCEDURE set_updated_at();
