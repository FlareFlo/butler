CREATE TABLE guilds (
    id bigint PRIMARY KEY,
    honeypot BIGSERIAL,
    logging_channel bigint
);

CREATE TABLE honeypot (
    id BIGSERIAL PRIMARY KEY,
    channel_ids bigint[] NOT NULL,
    safe_role_ids bigint[] NOT NULL,
    enabled BOOLEAN NOT NULL
);
