CREATE TABLE guilds
(
    id              BIGINT PRIMARY KEY,
    logging_channel BIGINT,
    account_minimum_age BIGINT -- Hours
);

CREATE TABLE honeypot
(
    guild_id      BIGINT PRIMARY KEY REFERENCES guilds (id) ON DELETE CASCADE,
    channel_ids   BIGINT[] NOT NULL,
    safe_role_ids BIGINT[] NOT NULL,
    enabled       BOOLEAN  NOT NULL
);

CREATE TYPE moderation_action AS ENUM ('kicked_honeypot', 'kicked_account_age');

CREATE TABLE action_journal
(
    id          BIGSERIAL PRIMARY KEY,
    guild       BIGINT            NOT NULL, -- No Cascade or Reference, we want to keep journals of offenders
    offender_id BIGINT            NOT NULL,
    action      moderation_action NOT NULL,
    time        timestamptz       NOT NULL DEFAULT now()
);