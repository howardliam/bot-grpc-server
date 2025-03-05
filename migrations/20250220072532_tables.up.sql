-- Add up migration script here
CREATE TABLE guild (guild_id bigint PRIMARY KEY);

CREATE TABLE logs_settings (
    guild_id bigint PRIMARY KEY REFERENCES guild (guild_id) ON DELETE CASCADE,
    enabled boolean DEFAULT false,
    channel_id bigint DEFAULT 0
);

CREATE TABLE tickets_settings (
    guild_id bigint PRIMARY KEY REFERENCES guild (guild_id) ON DELETE CASCADE,
    enabled boolean DEFAULT false,
    channel_id bigint DEFAULT 0
);

CREATE TABLE ticket (
    id serial PRIMARY KEY,
    guild_id bigint REFERENCES guild (guild_id) ON DELETE CASCADE,
    author_id bigint,
    title text NOT NULL,
    info text NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW ()
);

CREATE TABLE automod_settings (
    guild_id bigint PRIMARY KEY REFERENCES guild (guild_id) ON DELETE CASCADE,
    autoban_enabled boolean DEFAULT false,
    autoban_threshold int DEFAULT 5,
    autokick_enabled boolean DEFAULT false,
    autokick_threshold int DEFAULT 3
);

CREATE TABLE warn (
    id serial PRIMARY KEY,
    guild_id bigint REFERENCES guild (guild_id) ON DELETE CASCADE,
    staff_member_id bigint,
    target_user_id bigint,
    reason text NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW ()
);
