INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$sZ80l4K07HSZ/3XmwEhZd.douzhR7vvPk695SU3wSUPJVktlzqwmm',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
