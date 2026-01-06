-- up migration script
-- table: `JsInfo`
CREATE TABLE IF NOT EXISTS JsInfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    hash TEXT NOT NULL,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS JsInfo_value ON JsInfo (hash, value);

-- table: `UserAgent`
CREATE TABLE IF NOT EXISTS UserAgent (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS UserAgent_value ON UserAgent (value);
INSERT INTO UserAgent (id, value)
    SELECT * FROM (SELECT 0, '') AS UserAgent
    WHERE NOT EXISTS (SELECT * FROM UserAgent WHERE id = 0);

-- table: `Referrer`
CREATE TABLE IF NOT EXISTS Referrer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS Referrer_value ON Referrer (value);
INSERT INTO Referrer (id, value)
    SELECT * FROM (SELECT 0, '') AS Referrer
    WHERE NOT EXISTS (SELECT * FROM Referrer WHERE id = 0);

-- table: `IpAddress`
CREATE TABLE IF NOT EXISTS IpAddress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS IpAddress_value ON IpAddress (value);
INSERT INTO IpAddress (id, value)
    SELECT * FROM (SELECT 0, '') AS IpAddress
    WHERE NOT EXISTS (SELECT * FROM IpAddress WHERE id = 0);

-- table: `Bicmid`
CREATE TABLE IF NOT EXISTS Bicmid (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS Bicmid_value ON Bicmid (value);
INSERT INTO Bicmid (id, value)
    SELECT * FROM (SELECT 0, '') AS Bicmid
    WHERE NOT EXISTS (SELECT * FROM Bicmid WHERE id = 0);

-- table: `User`
CREATE TABLE IF NOT EXISTS User (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    value TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS User_value ON User (value);
INSERT INTO User (id, value)
    SELECT * FROM (SELECT 0, '') AS User
    WHERE NOT EXISTS (SELECT * FROM User WHERE id = 0);

-- table: `Log`
CREATE TABLE IF NOT EXISTS Log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    jsinfo_id INTEGER NOT NULL,
    user_agent_id INTEGER NOT NULL,
    referrer_id INTEGER NOT NULL,
    ipaddress_id INTEGER NOT NULL,
    bicmid_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS Log_jsinfo_id ON Log (jsinfo_id);
CREATE INDEX IF NOT EXISTS Log_user_agent_id ON Log (user_agent_id);
CREATE INDEX IF NOT EXISTS Log_referrer_id ON Log (referrer_id);
CREATE INDEX IF NOT EXISTS Log_ipaddress_id ON Log (ipaddress_id);
CREATE INDEX IF NOT EXISTS Log_bicmid_id ON Log (bicmid_id);
CREATE INDEX IF NOT EXISTS Log_user_id ON Log (user_id);
