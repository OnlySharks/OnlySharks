-- Your SQL goes here
CREATE TABLE posts (
  id TEXT NOT NULL DEFAULT md5(random()::text),
  creatorid TEXT NOT NULL,
  date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  content TEXT NOT NULL,
  images TEXT[],
  likes INTEGER NOT NULL DEFAULT '0',
  PRIMARY KEY (id)
)