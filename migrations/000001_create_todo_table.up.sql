CREATE TABLE IF NOT EXISTS todos(
  id serial primary key,
  title text NOT NULL,
  description varchar(200),
  completed boolean NOT NULL default false,
  created timestamp with time zone NOT NULL default (now() at time zone 'utc'),
  due_date date NOT NULL
);
