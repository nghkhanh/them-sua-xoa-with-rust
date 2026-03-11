--: User()

--! get_users : User
SELECT
    id,
    email
FROM auth.users;

-- 👇 add `create_user` query
--! create_user
INSERT INTO
    auth.users (email, external_id)
VALUES
    (:email, :external_id);

-- 👇 add `update_user` query
--! update_user
UPDATE auth.users
SET email = :email
WHERE id = :id;

-- 👇 add `delete_user` query
--! delete_user
DELETE FROM auth.users
WHERE id = :id;
