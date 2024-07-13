use std::path::PathBuf;
use std::fs::create_dir_all;
use rusqlite::{ Connection, Result };
use home::home_dir;

pub fn get_connection() -> Result<Connection>
{
    // let conn = match home_dir()
    // {
        // Some(dir) => {
            //
            // let mut path: PathBuf = [dir.to_str().unwrap(), ".local", "share", "rapid-reserve"].iter().collect();
            //
            // if !path.exists()
            // {
            //     create_dir_all(&path).unwrap();
            // }
            //
            // path.push("db.sql");

            Connection::open("rapid-reserve.db")
        // }

    // };

    // conn
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
             contact_info   TEXT,
             payment_status INTEGER
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
             id             INTEGER PRIMARY KEY AUTOINCREMENT,
             guest_id       INTEGER,
             invoice_id     INTEGER,
             room_id        INTEGER,
             checkin_date   DATE,
             checkout_date  DATE,
             FOREIGN KEY(guest_id) REFERENCES guest(id),
             FOREIGN KEY(invoice_id) REFERENCES invoice(id),
             FOREIGN KEY(room_id) REFERENCES invoice(id)
         );
         COMMIT;"
    )?;

    Ok(())
}
