-- Add migration script here
CREATE TABLE authors(
  id uuid DEFAULT gen_random_uuid() NOT NULL,
  PRIMARY KEY (id),
  name TEXT NOT NULL,
  nationality TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
