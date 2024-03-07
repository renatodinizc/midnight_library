-- Add migration script here
ALTER TABLE books
  ADD COLUMN author_id uuid NOT NULL,
  ADD CONSTRAINT fk_books_authors FOREIGN KEY (author_id) REFERENCES authors(id);