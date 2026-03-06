CREATE TABLE IF NOT EXISTS child_profiles (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    birthday DATE NOT NULL,
    gender BOOLEAN NOT NULL,
    weight INTEGER,
    height INTEGER
);