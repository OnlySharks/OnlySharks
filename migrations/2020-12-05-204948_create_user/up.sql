-- Your SQL goes here
CREATE TABLE users (
    id TEXT UNIQUE NOT NULL DEFAULT md5(random()::text),
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    creationdate TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    displayname TEXT NOT NULL,
    pronouns TEXT NOT NULL DEFAULT 'they/them',
    description TEXT NOT NULL DEFAULT '🦈🦈',
    birthday DATE NOT NULL,
    followers INTEGER NOT NULL DEFAULT '0',
    posts TEXT[] NOT NULL DEFAULT '{}',
    likedposts TEXT[] NOT NULL DEFAULT '{}',
    following TEXT[] NOT NULL DEFAULT '{}',
    authkey TEXT UNIQUE NOT NULL,
    PRIMARY KEY (id)
)
