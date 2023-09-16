DROP TABLE IF EXISTS sessions;
DROP TABLE IF EXISTS activations;

CREATE TABLE IF NOT EXISTS users (
    id serial primary key,
    active boolean default FALSE,
    email varchar(255) unique not null,
    password varchar not null,
    createdAt timestamp default current_timestamp,
    updatedAt timestamp default current_timestamp
);

CREATE TABLE IF NOT EXISTS farms (
    id serial primary key,
    user_id int not null references users(id),
    name varchar(255) not null,
    males int default 0,
    females int default 0,
    createdAt timestamp default current_timestamp,
    updatedAt timestamp default current_timestamp
);

CREATE TABLE IF NOT EXISTS sessions (
    id serial primary key,
    session_id varchar unique not null,
    user_id int unique not null
);

CREATE TABLE IF NOT EXISTS activations (
    id serial primary key,
    email varchar(255) unique not null,
    activation_id varchar unique not null
);