use crate::db::get_connection;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Guest
{
    id: i32,
    nik: String,
    room_id: i32,
    full_name: String,
    check_in_date: String,
    check_out_date: String,
    contact_info: String,
    payment_status: bool
}

impl Guest
{
    pub fn new(id: i32, nik: String, room_id: i32, full_name: String, check_in_date: String, check_out_date: String, contact_info: String, payment_status: bool) -> Self
    {
        Self { id, nik, room_id, full_name, check_in_date, check_out_date, contact_info, payment_status }
    }
}

#[tauri::command]
pub async fn add_guest(nik: String, room_id: i32, full_name: String, check_in_date: String, check_out_date: String, contact_info: String, payment_status: bool) -> i32
{
    let conn = get_connection().unwrap();

    let guest = Guest::new(0, nik, room_id, full_name, check_in_date, check_out_date, contact_info, payment_status);

    conn.execute("INSERT INTO guest
        (nik, room_id, full_name, checkin_date, checkout_date, contact_info, payment_status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&guest.nik, &guest.room_id, &guest.full_name, &guest.check_in_date, &guest.check_out_date, &guest.contact_info, &guest.payment_status)
    ).unwrap();

    conn.query_row("SELECT last_insert_rowid()", [], |row| row.get(0)).unwrap()
}

#[tauri::command]
pub async fn get_all_guests() -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, nik, room_id, full_name, checkin_date, checkout_date, contact_info, payment_status FROM guest").unwrap();

    let guests = stmt.query_map([], |row| {
        Ok(Guest::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
        ))
    }).unwrap();

    let mut vec = Vec::new();

    for guest in guests
    {
        vec.push(guest.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}

#[tauri::command]
pub async fn remove_guest(guest_id: i32)
{
    let conn = get_connection().unwrap();

    conn.execute("DELETE FROM guest WHERE id = ?1", [guest_id]).unwrap();
}

#[tauri::command]
pub async fn get_guest_information(guest_id: i32) -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, nik, room_id, full_name, checkin_date, checkout_date, contact_info, payment_status FROM guest WHERE id = ?1").unwrap();


    let guests = stmt.query_map([guest_id], |row| {
        Ok(Guest::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            row.get(7)?,
        ))
    }).unwrap();

    let mut vec = Vec::new();

    for guest in guests
    {
        vec.push(guest.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}

#[tauri::command]
pub async fn edit_guest(id: i32, nik: String, room_id: i32, full_name: String, check_in_date: String, check_out_date: String, contact_info: String, payment_status: bool)
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare(
        "UPDATE guest SET
         nik = ?2, room_id = ?3, full_name = ?4, checkin_date = ?5, checkout_date = ?6, contact_info = ?7, payment_status = ?8
         WHERE id = ?1"
    ).unwrap();

    stmt.execute(params![id, nik, room_id, full_name, check_in_date, check_out_date, contact_info, payment_status]).unwrap();
}

#[tauri::command]
pub async fn set_payment_status(guest_id: i32, status: bool)
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare(
        "UPDATE guest SET
         payment_status = ?2
         WHERE id = ?1"
    ).unwrap();

    stmt.execute(params![guest_id, status]).unwrap();
}
