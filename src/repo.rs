use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Pool};

pub async fn init_db(url: &str) -> Result<SqlitePool> {
    let pool = Pool::<Sqlite>::connect(url).await?;
    // schema
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS burns(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tx_hash_b64 TEXT UNIQUE,
            lt INTEGER,
            utime INTEGER,
            jetton_master_raw TEXT,
            owner_raw TEXT,
            jetton_wallet_raw TEXT,
            amount_raw TEXT,
            custom_payload BLOB,
            created_at INTEGER DEFAULT (strftime('%s','now'))
        );
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS attestations(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kind TEXT,
            payload_borsh BLOB,
            payload_hash_hex TEXT,
            src_tx_hash_b64 TEXT,
            created_at INTEGER DEFAULT (strftime('%s','now'))
        );
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS cursors(
            k TEXT PRIMARY KEY,
            v TEXT
        );
    "#).execute(&pool).await?;

    Ok(pool)
}

pub async fn insert_burn(
    db: &SqlitePool,
    tx_hash_b64: &str,
    lt: i64,
    utime: i64,
    jetton_master_raw: &str,
    owner_raw: &str,
    jetton_wallet_raw: &str,
    amount_raw: &str,
    custom_payload: Option<&[u8]>,
) -> Result<u64> {
    let res = sqlx::query!(
        r#"INSERT OR IGNORE INTO burns
           (tx_hash_b64, lt, utime, jetton_master_raw, owner_raw, jetton_wallet_raw, amount_raw, custom_payload)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        tx_hash_b64, lt, utime, jetton_master_raw, owner_raw, jetton_wallet_raw, amount_raw, custom_payload
    )
    .execute(db)
    .await?;
    Ok(res.rows_affected())
}

pub async fn list_burns(db: &SqlitePool, limit: i64) -> Result<Vec<BurnRow>> {
    let rows = sqlx::query_as!(
        BurnRow,
        r#"SELECT id, tx_hash_b64, lt, utime, jetton_master_raw,
                  owner_raw, jetton_wallet_raw, amount_raw, created_at
           FROM burns ORDER BY id DESC LIMIT ?"#,
        limit
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn insert_attestation(
    db: &SqlitePool,
    kind: &str,
    payload_borsh: &[u8],
    payload_hash_hex: &str,
    src_tx_hash_b64: Option<&str>,
) -> Result<i64> {
    let res = sqlx::query!(
        r#"INSERT INTO attestations(kind, payload_borsh, payload_hash_hex, src_tx_hash_b64, created_at)
           VALUES (?, ?, ?, ?, strftime('%s','now'))"#,
        kind, payload_borsh, payload_hash_hex, src_tx_hash_b64
    )
    .execute(db)
    .await?;
    Ok(res.last_insert_rowid())
}

pub async fn list_attestations(db: &SqlitePool, limit: i64) -> Result<Vec<AttRow>> {
    let rows = sqlx::query_as!(
        AttRow,
        r#"SELECT id, kind, payload_hash_hex, src_tx_hash_b64, created_at
           FROM attestations ORDER BY id DESC LIMIT ?"#,
        limit
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn set_cursor(db: &SqlitePool, k: &str, v: &str) -> Result<()> {
    sqlx::query!(r#"INSERT INTO cursors(k, v) VALUES(?, ?)
                    ON CONFLICT(k) DO UPDATE SET v=excluded.v"#, k, v)
        .execute(db).await?;
    Ok(())
}

pub async fn get_cursor(db: &SqlitePool, k: &str) -> Result<Option<String>> {
    let row = sqlx::query!(r#"SELECT v FROM cursors WHERE k=? "#, k)
        .fetch_optional(db).await?;
    Ok(row.map(|r| r.v))
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct BurnRow {
    pub id: i64,
    pub tx_hash_b64: String,
    pub lt: i64,
    pub utime: i64,
    pub jetton_master_raw: String,
    pub owner_raw: String,
    pub jetton_wallet_raw: String,
    pub amount_raw: String,
    pub created_at: i64,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct AttRow {
    pub id: i64,
    pub kind: String,
    pub payload_hash_hex: String,
    pub src_tx_hash_b64: Option<String>,
    pub created_at: i64,
}
