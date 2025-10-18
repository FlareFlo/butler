CREATE TABLE guilds (
    id bigint PRIMARY KEY,
    honeypot BIGSERIAL UNIQUE REFERENCES honeypot(id),
    logging_channel bigint
);

CREATE TABLE honeypot (
    id BIGSERIAL PRIMARY KEY REFERENCES guilds(honeypot) ON DELETE CASCADE,
    channel_ids bigint[] NOT NULL,
    safe_role_ids bigint[] NOT NULL,
    enabled BOOLEAN NOT NULL
);

CREATE TYPE moderation_action AS ENUM ('kicked_honeypot', 'kicked_account_age');

CREATE TABLE action_journal (
    id BIGSERIAL PRIMARY KEY,
    guild bigint NOT NULL, -- No Cascade or Reference, we want to keep journals of offenders
    offender_id bigint NOT NULL,
    action moderation_action NOT NULL,
    time timestamptz NOT NULL DEFAULT now()
);