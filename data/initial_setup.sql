INSERT INTO roles (name) VALUES ('Admin'), ('User') ON CONFLICT DO NOTHING;

INSERT INTO users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig' AS name,
    'eleazar.fig@example.com' AS email,
    '$2b$12$6M0ptzzI79f51Wih96p0xu6aUGVa47ko5BruKzID2/dIYmj0TzRxi' AS password_hash,
    role_id
FROM
    roles
WHERE
    name = 'Admin';
