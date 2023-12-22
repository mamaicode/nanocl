-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "resource_kinds" (
  "name" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "spec_key" UUID NOT NULL REFERENCES specs("key")
);

CREATE TABLE IF NOT EXISTS "resources" (
  "key" VARCHAR NOT NULL UNIQUE PRIMARY KEY,
  "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  "kind" VARCHAR NOT NULL,
  "spec_key" UUID NOT NULL REFERENCES specs("key")
);
