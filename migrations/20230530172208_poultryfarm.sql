-- Add migration script here
DROP TABLE IF EXISTS sessions;

CREATE TABLE IF NOT EXISTS users (
    id serial primary key,
    email varchar(255) unique not null,
    password varchar not null,
    createdAt timestamp default current_timestamp,
    updatedAt timestamp default current_timestamp
);

CREATE TABLE IF NOT EXISTS farms (
    id serial primary key references users(id),
    name varchar(255) not null,
    createdAt timestamp default current_timestamp,
    updatedAt timestamp default current_timestamp
);

CREATE TABLE IF NOT EXISTS sessions (
    id serial primary key,
    session_id varchar unique not null,
    user_id int unique not null
);