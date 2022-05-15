
CREATE TABLE scripts_exec_log
(
    id SERIAL PRIMARY KEY,
    script_name VARCHAR NOT NULL,
    run_start TIMESTAMP NOT NULL
)