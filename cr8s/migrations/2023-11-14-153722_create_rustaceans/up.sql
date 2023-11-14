CREATE TABLE rustaceans (
	id serial primary key,
	name varchar not null,
	email varchar not null,
	created_at TIMESTAMP default now() not null
);