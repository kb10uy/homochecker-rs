CREATE TABLE IF NOT EXISTS "users" (
    "id" SERIAL PRIMARY KEY,
    "screen_name" VARCHAR(255),
    "service" VARCHAR(20),
    "url" VARCHAR(255)
);
CREATE INDEX "users_sn_index" ON "users" ("screen_name");
