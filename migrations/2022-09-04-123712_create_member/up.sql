-- Your SQL goes here

create table members(
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    knockouts INT NOT NULL DEFAULT 0,
    team_id SERIAL NOT NULL REFERENCES teams(id)
);
