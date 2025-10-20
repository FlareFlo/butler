CREATE TABLE guilds
(
    id              BIGINT PRIMARY KEY,
    honeypot        BIGSERIAL UNIQUE,
    logging_channel BIGINT
);

CREATE TABLE honeypot
(
    id            BIGSERIAL PRIMARY KEY REFERENCES guilds (honeypot) ON DELETE CASCADE,
    channel_ids   BIGINT[] NOT NULL,
    safe_role_ids BIGINT[] NOT NULL,
    enabled       BOOLEAN NOT NULL
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


CREATE TABLE account_age
(
    guild_id         BIGINT UNIQUE REFERENCES guilds (id) ON DELETE CASCADE,
    user_id          BIGINT UNIQUE,
    account_creation timestamptz NOT NULL,
    PRIMARY KEY (guild_id, user_id)
);