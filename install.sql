BEGIN;

CREATE ROLE dbmeter WITH password '12345' login;
CREATE SCHEMA IF NOT EXISTS dbmeter;
GRANT usage ON Schema dbmeter TO dbmeter;

CREATE OR REPLACE FUNCTION dbmeter.pg_stat_statements()
   RETURNS pg_stat_statements as
$$
   SELECT * FROM pg_stat_statements
$$ language sql SECURITY DEFINER;

COMMIT;
