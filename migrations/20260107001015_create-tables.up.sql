-- up migration script
-- table: `js_info`
CREATE TABLE IF NOT EXISTS jsinfos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    hash TEXT NOT NULL,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS ui_jsinfos_01 ON jsinfos (hash, value);

-- table: `user_agents`
CREATE TABLE IF NOT EXISTS user_agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS ui_user_agents_01 ON user_agents (value);
INSERT INTO user_agents (id, value)
    SELECT * FROM (SELECT 0, '') AS user_agents
    WHERE NOT EXISTS (SELECT * FROM user_agents WHERE id = 0);

-- table: `referrers`
CREATE TABLE IF NOT EXISTS referrers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS ui_referrers_01 ON referrers (value);
INSERT INTO referrers (id, value)
    SELECT * FROM (SELECT 0, '') AS referrers
    WHERE NOT EXISTS (SELECT * FROM referrers WHERE id = 0);

-- table: `ip_addresses`
CREATE TABLE IF NOT EXISTS ip_addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS ui_ip_addresses_01 ON ip_addresses (value);
INSERT INTO ip_addresses (id, value)
    SELECT * FROM (SELECT 0, '') AS ip_addresses
    WHERE NOT EXISTS (SELECT * FROM ip_addresses WHERE id = 0);

-- table: `bicmids`
CREATE TABLE IF NOT EXISTS bicmids (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS ui_bicmids ON bicmids (value);
INSERT INTO bicmids (id, value)
    SELECT * FROM (SELECT 0, '') AS bicmids
    WHERE NOT EXISTS (SELECT * FROM bicmids WHERE id = 0);

-- table: `users`
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS ui_users_01 ON users (value);
INSERT INTO users (id, value)
    SELECT * FROM (SELECT 0, '') AS users
    WHERE NOT EXISTS (SELECT * FROM users WHERE id = 0);

-- table: `logs`
CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    jsinfo_id INTEGER NOT NULL,
    user_agent_id INTEGER NOT NULL,
    referrer_id INTEGER NOT NULL,
    ip_address_id INTEGER NOT NULL,
    bicmid_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS ix_logs_01 ON logs (jsinfo_id);
CREATE INDEX IF NOT EXISTS ix_logs_02 ON logs (user_agent_id);
CREATE INDEX IF NOT EXISTS ix_logs_03 ON logs (referrer_id);
CREATE INDEX IF NOT EXISTS ix_logs_04 ON logs (ip_address_id);
CREATE INDEX IF NOT EXISTS ix_logs_05 ON logs (bicmid_id);
CREATE INDEX IF NOT EXISTS ix_logs_06 ON logs (user_id);
