CREATE TABLE IF NOT EXISTS "tasks"
(
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        VARCHAR NOT NULL,
    cron        VARCHAR NOT NULL,
    repeat_mode VARCHAR NOT NULL DEFAULT 'repeat',
    state       VARCHAR NOT NULL DEFAULT 'enabled',
    guild_id    INTEGER NOT NULL,
    message_id  INTEGER NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages (id)
        ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS messages
(
    id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    content    TEXT    NOT NULL,
    guild_id   INTEGER NOT NULL,
    channel_id INTEGER NOT NULL
);

INSERT INTO messages (content, guild_id, channel_id)
VALUES ('hello, world from the db!', '1', '1');
INSERT INTO tasks (name, cron, guild_id, message_id)
VALUES ('test-task', '* * * * *', '1', '1');