use std::path::PathBuf;
use rusqlite::{ Connection, Result };
use tauri::api::path::local_data_dir;

#[tauri::command]
pub fn get_database_path() -> String
{
    match local_data_dir()
    {
        Some(dir) => {
            let path: PathBuf = [dir.to_str().unwrap(), "rapid-reserve-data.db"].iter().collect();
            path.to_string_lossy().to_string()
        },
        None => "none".to_string()
    }
}

pub fn get_connection() -> Result<Connection>
{
    match local_data_dir()
    {
        Some(dir) => {
            let path: PathBuf = [dir.to_string_lossy().to_string(), "rapid-reserve-data.db".to_string()].iter().collect();
            Connection::open(path)
        },
        None => Connection::open_in_memory()
    }
}

pub fn setup(conn: Connection) -> Result<()>
{
    conn.execute_batch(
        "BEGIN;
         CREATE TABLE IF NOT EXISTS room (
             id         INTEGER PRIMARY KEY AUTOINCREMENT,
             number     INTEGER,
             bed_type   TEXT,
             occupied   INTEGER,
             price      INTEGER
         );
         CREATE TABLE IF NOT EXISTS guest (
             id             INTEGER PRIMARY KEY AUTOINCREMENT,
             nik            TEXT,
             room_ID        INTEGER,
             full_name      TEXT,
             checkin_date   DATE,
             checkout_date  DATE,
             duration       INTEGER,
             contact_info   TEXT,
             payment_status INTEGER,
             is_checked_out INTEGER
         );
         CREATE TABLE IF NOT EXISTS invoice (
             id             INTEGER PRIMARY KEY AUTOINCREMENT,
             guest_id       INTEGER,
             items_json     TEXT,
             amount_to_pay  INTEGER,
             date           DATE,
             due_date       DATE,
             has_paid       INTEGER,
             FOREIGN KEY(guest_id) REFERENCES guest(id) ON DELETE NO ACTION
         );
         CREATE TABLE IF NOT EXISTS report (
             id                     INTEGER PRIMARY KEY AUTOINCREMENT,
             guest_id               INTEGER,
             invoice_id             INTEGER,
             room_id                INTEGER,
             checkin_date           DATE,
             actual_checkin_date    DATE,
             checkout_date          DATE,
             actual_checkout_date   DATE,
             FOREIGN KEY(guest_id) REFERENCES guest(id),
             FOREIGN KEY(invoice_id) REFERENCES invoice(id),
             FOREIGN KEY(room_id) REFERENCES invoice(id)
         );
         COMMIT;"
    )?;

    Ok(())
}
