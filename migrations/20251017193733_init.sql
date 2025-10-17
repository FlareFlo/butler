CREATE TABLE guilds (
    id integer PRIMARY KEY,
    honeypot BIGSERIAL,
    logging_channel integer
);

CREATE TABLE honeypot (
    id BIGSERIAL PRIMARY KEY,
    channel_ids integer[],
    safe_role_ids integer[],
    enabled BOOLEAN DEFAULT true
);
