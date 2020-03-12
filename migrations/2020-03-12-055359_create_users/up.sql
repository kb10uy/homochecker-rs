CREATE TABLE IF NOT EXISTS "users" (
    "id" SERIAL PRIMARY KEY,
    "screen_name" VARCHAR(256),
    "avatar_url" VARCHAR(512),
    "service_url" VARCHAR(512)
);
CREATE INDEX "users_sn_index" ON "users" ("screen_name");
