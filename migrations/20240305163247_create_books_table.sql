CREATE TABLE books(
  id uuid DEFAULT gen_random_uuid() NOT NULL,
  PRIMARY KEY (id),
  title TEXT NOT NULL,
  author TEXT NOT NULL,
  genre TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
