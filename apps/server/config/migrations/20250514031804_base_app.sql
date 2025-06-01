CREATE TABLE "users" (
    "id" UUID PRIMARY KEY,
    "username" TEXT NOT NULL,
    "email" TEXT UNIQUE NOT NULL,
    "password" TEXT NOT NULL,
    "validated" BOOLEAN NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL,
    "updated_at" TIMESTAMPTZ NOT NULL
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW."updated_at" = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();
