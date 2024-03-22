CREATE TABLE users(
  id uuid DEFAULT gen_random_uuid() NOT NULL,
  PRIMARY KEY (id),
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  created_at timestamptz NOT NULL
);