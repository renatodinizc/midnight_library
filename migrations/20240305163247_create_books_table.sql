CREATE TABLE books(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  title TEXT NOT NULL,
  author TEXT NOT NULL,
  genre TEXT NOT NULL,
  created_at timestamptz NOT NULL
);
