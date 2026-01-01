use anyhow::Result;
use browserinfo::{BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::UserAgent;

#[cfg(feature = "server")]
use std::cell::RefCell;

#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
use super::get_ipaddress_string;

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: RefCell<rusqlite::Connection> = {
        let db_path = get_db_path_();
        // Open the database from the persisted "broinfo.db" file
        let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");
        // Create tables if it doesn't already exist
        create_tables(&conn).unwrap();
        // Return the connection
        RefCell::new(conn)
    };
}

#[cfg(feature = "server")]
fn get_db_path_() -> PathBuf {
    let key1 = "BROWSERINFOCM_DB_PATH";
    if let Ok(s) = std::env::var(key1) {
        return PathBuf::from(s);
    }
    let key2 = "BROWSERINFOCM_DB_BASE_PATH";
    let mut data_dir = if let Ok(s) = std::env::var(key2) {
        let pb = PathBuf::from(s);
        let _ = std::fs::create_dir_all(&pb);
        pb
    } else {
        data_dir()
    };
    let key3 = "BROWSERINFOCM_DB_FILE";
    let db_file = if let Ok(s) = std::env::var(key3) {
        s
    } else {
        "browserinfocm.db".to_string()
    };
    data_dir.push(db_file);
    data_dir
}

#[cfg(feature = "server")]
fn data_dir() -> PathBuf {
    let data_dir: PathBuf;
    #[cfg(not(feature = "backend_homedir"))]
    {
        data_dir = PathBuf::from("/var/local/data/broinfo");
        let _ = std::fs::create_dir_all(&data_dir);
    }
    #[cfg(feature = "backend_homedir")]
    {
        data_dir = data_dir_on_desktop();
    }
    return data_dir;
}

#[cfg(feature = "backend_homedir")]
#[cfg(feature = "server")]
fn data_dir_on_desktop() -> PathBuf {
    let mut data_dir = match std::env::home_dir() {
        Some(home) => home,
        None => {
            eprintln!("could NOT get `home_dir()`");
            PathBuf::from(".")
        }
    };
    data_dir.push(".data");
    data_dir.push("broinfo");
    let _ = std::fs::create_dir_all(&data_dir);
    data_dir
}

#[post("/api/v1/mikan1")]
pub async fn get_db_path() -> Result<String> {
    let db_path = get_db_path_();
    let db_path_s = db_path.display().to_string();
    dioxus_logger::tracing::debug!("db_path: {db_path_s:?}");
    Ok(db_path_s)
}

#[post("/api/v1/ringo1", headers: dioxus::fullstack::HeaderMap)]
pub async fn get_ipaddress() -> Result<String> {
    let ipaddr = get_ipaddress_string(&headers);
    dioxus_logger::tracing::debug!("ipaddr: {ipaddr:?}");
    Ok(ipaddr)
}

#[cfg(feature = "backend_user_agent")]
#[post("/api/v1/useragent1")]
pub async fn save_user_agent(ua: UserAgent) -> Result<()> {
    let ua_s = ua.get().trim_start_matches('"').trim_end_matches('"');
    //
    #[cfg(feature = "backend_text")]
    write_backend_text("user_agent.txt", ua_s)?;
    //
    DB.with_borrow_mut(|f| {
        let tx = f.transaction()?;
        //
        let user_agent_id = get_or_store_user_agent(&tx, &ua_s)?;
        if user_agent_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        tx.commit()
    })?;
    //
    dioxus_logger::tracing::debug!("save_user_agent: {ua_s:?}");
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    Ok(())
}

#[post("/api/v1/browserinfo1", headers: dioxus::fullstack::HeaderMap)]
pub async fn save_broinfo(
    broinfo: BroInfo,
    bicmid: String,
    user: String,
    return_browser: bool,
) -> Result<Option<Browser>> {
    let ipaddress = get_ipaddress_string(&headers);
    let user_agent = broinfo.basic.user_agent.clone();
    let referrer = broinfo.basic.referrer.clone();

    let jsinfo_s = toml::to_string(&broinfo.jsinfo).unwrap();
    let jsinfo_ss = jsinfo_s.replace("\n", "<BR>");

    #[cfg(feature = "backend_text")]
    write_backend_text("jsinfo.txt", &jsinfo_s)?;
    //
    DB.with_borrow_mut(|f| {
        let tx = f.transaction()?;
        //
        let user_agent_id = get_or_store_user_agent(&tx, user_agent.get())?;
        if user_agent_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let referrer_id = get_or_store_referrer(&tx, referrer.get())?;
        if referrer_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let ipaddress_id = get_or_store_ipaddress(&tx, &ipaddress)?;
        if ipaddress_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let bicmid_id = get_or_store_bicmid(&tx, &bicmid)?;
        if bicmid_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let user_id = get_or_store_user(&tx, &user)?;
        if user_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let jsinfo_id = get_or_store_jsinfo(&tx, &jsinfo_ss)?;
        if jsinfo_id == -1 {
            tx.rollback()?;
            return Ok(());
        }
        //
        tx.execute(
            "INSERT INTO Log (jsinfo_id, user_agent_id, referrer_id, ipaddress_id, bicmid_id, user_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![&jsinfo_id, &user_agent_id, &referrer_id, &ipaddress_id, &bicmid_id, &user_id],
        )?;
        //
        tx.commit()
    })?;
    //
    dioxus_logger::tracing::debug!("save_broinfo: {jsinfo_ss:?}");
    //
    #[cfg(feature = "backend_delay")]
    let _ = sleep_x(2000).await;
    //
    if return_browser {
        Ok(Some(broinfo.to_browser()?))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "backend_text")]
#[cfg(feature = "server")]
fn write_backend_text(fnm: &str, data: &str) -> Result<()> {
    use std::io::Write;
    //
    // Open file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(fnm)?;
    // And then write a newline to it with data
    let _ = file.write_fmt(format_args!("{data}\n"));
    Ok(())
}

#[cfg(feature = "backend_delay")]
#[cfg(feature = "server")]
async fn sleep_x(millis: u64) -> Result<()> {
    async_std::task::sleep(std::time::Duration::from_millis(millis)).await;
    Ok(())
}

// Create tables if it doesn't already exist
#[cfg(feature = "server")]
fn create_tables(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    // table: `JsInfo`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS JsInfo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                hash TEXT NOT NULL,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS JsInfo_value ON JsInfo (hash, value);",
    )?;
    {
        let s = "";
        let hash = create_jsinfo_hash(s);
        let hash_s = hash.as_str();
        conn.execute(
            "INSERT INTO JsInfo (id, hash, value) SELECT * FROM (SELECT 0, ?1, ?2) AS JsInfo
                WHERE NOT EXISTS (SELECT * FROM JsInfo WHERE id = 0);",
            &[hash_s, s],
        )?;
    }
    // table: `UserAgent`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS UserAgent (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS UserAgent_value ON UserAgent (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO UserAgent (id, value) SELECT * FROM (SELECT 0, '') AS UserAgent
            WHERE NOT EXISTS (SELECT * FROM UserAgent WHERE id = 0);",
    )?;
    // table: `Referrer`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Referrer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS Referrer_value ON Referrer (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO Referrer (id, value) SELECT * FROM (SELECT 0, '') AS Referrer
            WHERE NOT EXISTS (SELECT * FROM Referrer WHERE id = 0);",
    )?;
    // table: `IpAddress`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS IpAddress (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS IpAddress_value ON IpAddress (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO IpAddress (id, value) SELECT * FROM (SELECT 0, '') AS IpAddress
            WHERE NOT EXISTS (SELECT * FROM IpAddress WHERE id = 0);",
    )?;
    // table: `Bicmid`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Bicmid (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS Bicmid_value ON Bicmid (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO Bicmid (id, value) SELECT * FROM (SELECT 0, '') AS Bicmid
            WHERE NOT EXISTS (SELECT * FROM Bicmid WHERE id = 0);",
    )?;
    // table: `User`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS User (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS User_value ON User (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO User (id, value) SELECT * FROM (SELECT 0, '') AS User
            WHERE NOT EXISTS (SELECT * FROM User WHERE id = 0);",
    )?;
    // table: `Log`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Log (
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
        ",
    )?;
    Ok(())
}

#[cfg(feature = "server")]
fn get_or_store_user_agent(tx: &rusqlite::Transaction, user_agent: &str) -> rusqlite::Result<i64> {
    let mut user_agent_id = -1;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM UserAgent WHERE value = ?1",
        &[user_agent],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO UserAgent (value) VALUES (?1)", &[user_agent])?;
        user_agent_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        user_agent_id = id;
    }
    Ok(user_agent_id)
}

#[cfg(feature = "server")]
fn get_or_store_referrer(tx: &rusqlite::Transaction, referrer: &str) -> rusqlite::Result<i64> {
    let mut referrer_id = -1;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM Referrer WHERE value = ?1",
        &[referrer],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO Referrer (value) VALUES (?1)", &[referrer])?;
        referrer_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        referrer_id = id;
    }
    Ok(referrer_id)
}

#[cfg(feature = "server")]
fn get_or_store_ipaddress(tx: &rusqlite::Transaction, ipaddress: &str) -> rusqlite::Result<i64> {
    let mut ipaddress_id = -1;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM IpAddress WHERE value = ?1",
        &[ipaddress],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO IpAddress (value) VALUES (?1)", &[ipaddress])?;
        ipaddress_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        ipaddress_id = id;
    }
    Ok(ipaddress_id)
}

#[cfg(feature = "server")]
fn get_or_store_bicmid(tx: &rusqlite::Transaction, bicmid: &str) -> rusqlite::Result<i64> {
    let mut bicmid_id = -1;
    let r: rusqlite::Result<i64> =
        tx.query_one("SELECT id FROM Bicmid WHERE value = ?1", &[bicmid], |row| {
            row.get(0)
        });
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO Bicmid (value) VALUES (?1)", &[bicmid])?;
        bicmid_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        bicmid_id = id;
    }
    Ok(bicmid_id)
}

#[cfg(feature = "server")]
fn get_or_store_user(tx: &rusqlite::Transaction, user: &str) -> rusqlite::Result<i64> {
    let mut user_id = -1;
    let r: rusqlite::Result<i64> =
        tx.query_one("SELECT id FROM User WHERE value = ?1", &[user], |row| {
            row.get(0)
        });
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute("INSERT INTO User (value) VALUES (?1)", &[user])?;
        user_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        user_id = id;
    }
    Ok(user_id)
}

#[cfg(feature = "server")]
fn get_or_store_jsinfo(tx: &rusqlite::Transaction, info_s: &str) -> rusqlite::Result<i64> {
    let hash = create_jsinfo_hash(info_s);
    let hash_s = hash.as_str();
    let mut jsinfo_id = -1;
    let r: rusqlite::Result<i64> = tx.query_one(
        "SELECT id FROM JsInfo WHERE hash = ?1 AND value = ?2",
        &[hash_s, info_s],
        |row| row.get(0),
    );
    if let Err(rusqlite::Error::QueryReturnedNoRows) = r {
        tx.execute(
            "INSERT INTO JsInfo (hash, value) VALUES (?1, ?2)",
            &[hash_s, info_s],
        )?;
        jsinfo_id = tx.last_insert_rowid();
    } else if let Ok(id) = r {
        jsinfo_id = id;
    }
    Ok(jsinfo_id)
}

#[cfg(feature = "server")]
fn create_jsinfo_hash(s: &str) -> String {
    use base64::Engine;

    let hash_bytes = hmac_sha256::Hash::hash(s.as_bytes());
    let hash_base64_s = base64::engine::general_purpose::STANDARD_NO_PAD.encode(hash_bytes);
    hash_base64_s
}

#[cfg(feature = "server")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_jsinfo_hash_00() {
        let s0 = "";
        let s = create_jsinfo_hash(s0);
        assert_eq!(s.len(), 43);
        assert_eq!(s, "47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU");
    }

    #[test]
    fn test_create_jsinfo_hash_01() {
        let s0 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let s = create_jsinfo_hash(s0);
        assert_eq!(s.len(), 43);
        assert_eq!(s, "OWQpS2ZGE3mNGkd+uK0CEYtI0MVzjEJ2EyAvLtEjtfE");
    }
}
