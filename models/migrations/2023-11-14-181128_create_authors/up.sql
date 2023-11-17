-- Your SQL goes here
CREATE TABLE authors (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  token VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE authors ADD CONSTRAINT unique_email UNIQUE (email);

ALTER TABLE authors ADD CONSTRAINT unique_token UNIQUE (token);

ALTER TABLE posts ADD COLUMN author_id INTEGER;

ALTER TABLE posts ADD CONSTRAINT fk_author_id FOREIGN KEY (author_id) REFERENCES authors (id);
