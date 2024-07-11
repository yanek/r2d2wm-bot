CREATE TABLE IF NOT EXISTS "tasks"
(
    "id"          VARCHAR NOT NULL UNIQUE,
    "name"        VARCHAR NOT NULL,
    "cron"        VARCHAR NOT NULL,
    "repeat_mode" VARCHAR NOT NULL DEFAULT 'repeat',
    "state"       VARCHAR NOT NULL DEFAULT 'enabled',
    "guild_id"    INTEGER NOT NULL,
    "message_id"  VARCHAR NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("message_id") REFERENCES "messages" ("id")
        ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "messages"
(
    "id"         VARCHAR NOT NULL UNIQUE,
    "content"    TEXT    NOT NULL,
    "guild_id"   INTEGER NOT NULL,
    "channel_id" INTEGER NOT NULL,
    PRIMARY KEY ("id")
);

INSERT INTO messages (id, content, guild_id, channel_id)
VALUES ('c9fd113b-9065-4055-b790-066d6fe759ae', 'hello, world from the db!', '0', '0');
INSERT INTO tasks (id, name, cron, guild_id, message_id)
VALUES ('1eeef8f8-181a-4867-bc46-d4673eedaa63', 'test-task', '* * * * *', '0', 'c9fd113b-9065-4055-b790-066d6fe759ae');