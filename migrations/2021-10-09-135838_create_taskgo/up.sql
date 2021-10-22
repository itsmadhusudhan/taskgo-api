-- Your SQL goes here
CREATE TABLE IF NOT EXISTS collections(
    id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS tasks(
    id UUID PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    collection_id VARCHAR NOT NULL REFERENCES collections (id) created_at TIMESTAMP DEFAULT now() NOT NULL,
);