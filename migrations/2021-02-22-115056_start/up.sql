-- Your SQL goes here
CREATE TABLE "themes" (
  "id" serial PRIMARY KEY NOT NULL,
  "name" text NOT NULL
);

CREATE TABLE "tasks" (
  "id" serial PRIMARY KEY NOT NULL,
  "theme" serial NOT NULL,
  "task_text" text NOT NULL,
  "input" text NOT NULL,
  "output" text NOT NULL
);

ALTER TABLE "tasks" ADD FOREIGN KEY ("theme") REFERENCES "themes" ("id");
