CREATE TYPE category_enum AS ENUM (
  'action',
  'adventure', 
  'horror',
  'scifi',
  'romance',
  'comedy'
);

CREATE TYPE visibility_enum AS ENUM (
  'public',
  'private'
);

CREATE TABLE IF NOT EXISTS videos (
  id          uuid           PRIMARY KEY DEFAULT uuidv7(),
  title       varchar(150)   NOT NULL,
  description text   NOT NULL,
  categories  category_enum[]   NOT NULL,
  visibility  visibility_enum[] NOT NULL 
);
