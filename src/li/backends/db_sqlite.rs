use anyhow::Result;
use browserinfo::{BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::UserAgent;

#[cfg(feature = "server")]
use std::cell::RefCell;

#[cfg(feature = "server")]
use std::path::PathBuf;

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
pub async fn get_ipaddr() -> Result<String> {
    let ipaddr = get_ipaddr_string(&headers);
    dioxus_logger::tracing::debug!("ipaddr: {ipaddr:?}");
    Ok(ipaddr)
}

#[cfg(feature = "server")]
fn get_ipaddr_string(headers: &dioxus::fullstack::HeaderMap) -> String {
    if let Some(s) = headers.get("x-forwarded-for") {
        s.to_str().unwrap().to_string()
    } else {
        "".to_string()
    }
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
        if user_agent_id == 0 {
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
pub async fn save_broinfo(broinfo: BroInfo, return_browser: bool) -> Result<Option<Browser>> {
    let ipaddr = get_ipaddr_string(&headers);
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
        if user_agent_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let referrer_id = get_or_store_referrer(&tx, referrer.get())?;
        if referrer_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        //
        let jsinfo_id = get_or_store_jsinfo(&tx, &jsinfo_ss)?;
        if jsinfo_id == 0 {
            tx.rollback()?;
            return Ok(());
        }
        //
        if ipaddr.is_empty() {
            tx.execute(
                "INSERT INTO Logs (jsinfo_id, user_agent_id, referrer_id) VALUES (?1, ?2, ?3)",
                &[&jsinfo_id, &user_agent_id, &referrer_id],
            )?;
        } else {
            tx.execute(
                "INSERT INTO Logs (jsinfo_id, user_agent_id, referrer_id, ipaddr) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![&jsinfo_id, &user_agent_id, &referrer_id, &ipaddr],
            )?;
        }
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
                id INTEGER PRIMARY KEY,
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
            "INSERT INTO JsInfo (hash, value) SELECT * FROM (SELECT ?1, ?2) AS JsInfo
                WHERE NOT EXISTS (SELECT * FROM JsInfo WHERE hash = ?1 AND value = ?2);",
            &[hash_s, s],
        )?;
    }
    // table: `UserAgent`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS UserAgent (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS UserAgent_value ON UserAgent (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO UserAgent (value) SELECT * FROM (SELECT '') AS UserAgent
            WHERE NOT EXISTS (SELECT * FROM UserAgent WHERE value = '');",
    )?;
    // table: `Referrer`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Referrer (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                value TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS Referrer_value ON Referrer (value);",
    )?;
    conn.execute_batch(
        "INSERT INTO Referrer (value) SELECT * FROM (SELECT '') AS Referrer
            WHERE NOT EXISTS (SELECT * FROM Referrer WHERE value = '');",
    )?;
    // table: `Logs`
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS Logs (
                id INTEGER PRIMARY KEY,
                create_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                jsinfo_id INTEGER NOT NULL,
                user_agent_id INTEGER NOT NULL,
                referrer_id INTEGER NOT NULL,
                ipaddr TEXT,
                user_id INTEGER
        );
        CREATE INDEX IF NOT EXISTS Logs_jsinfo_id ON Logs (jsinfo_id);
        CREATE INDEX IF NOT EXISTS Logs_user_agent_id ON Logs (user_agent_id);
        CREATE INDEX IF NOT EXISTS Logs_referrer_id ON Logs (referrer_id);
        CREATE INDEX IF NOT EXISTS Logs_ipaddr ON Logs (ipaddr);
        CREATE INDEX IF NOT EXISTS Logs_user_id ON Logs (user_id);
        ",
    )?;
    Ok(())
}

#[cfg(feature = "server")]
fn get_or_store_user_agent(tx: &rusqlite::Transaction, user_agent: &str) -> rusqlite::Result<i64> {
    let mut user_agent_id = 0;
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
    let mut referrer_id = 0;
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
fn get_or_store_jsinfo(tx: &rusqlite::Transaction, info_s: &str) -> rusqlite::Result<i64> {
    let hash = create_jsinfo_hash(info_s);
    let hash_s = hash.as_str();
    let mut jsinfo_id = 0;
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
