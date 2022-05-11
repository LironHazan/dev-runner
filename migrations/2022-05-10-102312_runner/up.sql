
CREATE TABLE scripts_exec_log
(
    id SERIAL PRIMARY KEY,
    script VARCHAR NOT NULL,
    timestamp TIMESTAMP NOT NULL
)