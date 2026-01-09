use anyhow::Result;
use browserinfo::{BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::UserAgent;

#[cfg(feature = "server")]
use std::path::PathBuf;

#[cfg(feature = "server")]
use super::get_ipaddress_string;

#[cfg(feature = "server")]
use sqlx::Transaction;

#[cfg(feature = "server")]
use sqlx::Row;

#[cfg(feature = "server")]
use dioxus::fullstack::Lazy;

// The database is only available to server code
#[cfg(feature = "server")]
static DB: Lazy<sqlx::SqlitePool> = Lazy::new(|| async move {
    let pool = create_sqlx_pool().await?;
    dioxus::Ok(pool)
});

#[cfg(feature = "server")]
async fn create_sqlx_pool() -> Result<sqlx::sqlite::SqlitePool> {
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::str::FromStr;
    //
    let db_path = get_db_path_();
    let sq_uri = format!("sqlite://{}", db_path.display());
    // Open the database from the persisted "broinfo.sqlite3" file
    let opts = SqliteConnectOptions::from_str(&sq_uri)?.create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;
    // Create tables if it doesn't already exist
    create_tables(&pool).await?;
    Ok(pool)
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
        "browserinfocm.sqlite3".to_string()
    };
    data_dir.push(db_file);
    data_dir
}

#[cfg(feature = "server")]
fn data_dir() -> PathBuf {
    let data_dir: PathBuf;
    #[cfg(not(feature = "backend_homedir"))]
    {
        data_dir = PathBuf::from("/var/local/data/browserinfocm");
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
    data_dir.push("browserinfocm");
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
    loop {
        let mut tx = DB.begin().await?;
        //
        let user_agent_id = get_or_store_user_agent(&mut tx, ua_s).await?;
        if user_agent_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        tx.commit().await?;
        break;
    }
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
    loop {
        let mut tx: Transaction<'_, sqlx::Sqlite> = DB.begin().await?;
        //
        let user_agent_id = get_or_store_user_agent(&mut tx, user_agent.get()).await?;
        if user_agent_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        let referrer_id = get_or_store_referrer(&mut tx, referrer.get()).await?;
        if referrer_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        let ipaddress_id = get_or_store_ipaddress(&mut tx, &ipaddress).await?;
        if ipaddress_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        let bicmid_id = get_or_store_bicmid(&mut tx, &bicmid).await?;
        if bicmid_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        let user_id = get_or_store_user(&mut tx, &user).await?;
        if user_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        let jsinfo_id = get_or_store_jsinfo(&mut tx, &jsinfo_ss).await?;
        if jsinfo_id == -1 {
            tx.rollback().await?;
            break;
        }
        //
        sqlx::query(concat!(
            r#"INSERT INTO Log"#,
            r#" (jsinfo_id, user_agent_id, referrer_id, ipaddress_id, bicmid_id, user_id)"#,
            r#" VALUES (?, ?, ?, ?, ?, ?)"#
        ))
        .bind(&jsinfo_id)
        .bind(&user_agent_id)
        .bind(&referrer_id)
        .bind(&ipaddress_id)
        .bind(&bicmid_id)
        .bind(&user_id)
        .execute(&mut *tx)
        .await?;
        //
        tx.commit().await?;
        break;
    }
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
async fn create_tables(pool: &sqlx::sqlite::SqlitePool) -> Result<()> {
    const SQL: &str = include_str!("../../../migrations/20260107001015_create-tables.up.sql");
    sqlx::raw_sql(SQL).execute(pool).await?;
    //
    // `JsInfo` special data
    {
        let s = "";
        let hash = create_jsinfo_hash(s);
        let hash_s = hash.as_str();
        const SQL: &str = concat!(
            r#"INSERT INTO JsInfo (id, hash, value)"#,
            r#" SELECT * FROM (SELECT 0, ?, ?) AS JsInfo"#,
            r#" WHERE NOT EXISTS (SELECT * FROM JsInfo WHERE id = 0)"#
        );
        sqlx::query(SQL)
            .persistent(false)
            .bind(hash_s)
            .bind(s)
            .execute(pool)
            .await?;
    }
    Ok(())
}

#[cfg(feature = "server")]
macro_rules! simple_get_or_store {
    ($func:ident, $tbl: expr) => {
        async fn $func(tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>, val: &str) -> Result<i64> {
            let mut tbl_id = -1;
            let r = sqlx::query(concat!(r#"SELECT id FROM "#, $tbl, r#" WHERE value = ?"#))
                .bind(val)
                .fetch_one(&mut **tx)
                .await;
            if let Ok(row) = r {
                tbl_id = row.get(0);
            } else if let Err(sqlx::Error::RowNotFound) = r {
                let r = sqlx::query(concat!(r#"INSERT INTO "#, $tbl, r#" (value) VALUES (?)"#))
                    .bind(val)
                    .execute(&mut **tx)
                    .await?;
                tbl_id = r.last_insert_rowid();
            } else if let Err(e) = r {
                return Err(e.into());
            }
            Ok(tbl_id)
        }
    };
}

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_user_agent, "UserAgent");

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_referrer, "Referrer");

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_ipaddress, "IpAddress");

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_bicmid, "Bicmid");

#[cfg(feature = "server")]
simple_get_or_store!(get_or_store_user, "User");

#[cfg(feature = "server")]
async fn get_or_store_jsinfo(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    info_s: &str,
) -> Result<i64> {
    let hash = create_jsinfo_hash(info_s);
    let hash_s = hash.as_str();
    let mut jsinfo_id = -1;
    let r = sqlx::query(r#"SELECT id FROM JsInfo WHERE hash = ? AND value = ?"#)
        .bind(hash_s)
        .bind(info_s)
        .fetch_one(&mut **tx)
        .await;
    if let Ok(row) = r {
        jsinfo_id = row.get(0);
    } else if let Err(sqlx::Error::RowNotFound) = r {
        let r = sqlx::query(r#"INSERT INTO JsInfo (hash, value) VALUES (?, ?)"#)
            .bind(hash_s)
            .bind(info_s)
            .execute(&mut **tx)
            .await?;
        jsinfo_id = r.last_insert_rowid();
    } else if let Err(e) = r {
        return Err(e.into());
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
