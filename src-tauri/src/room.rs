use crate::db::get_connection;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Room
{
    id: i32,
    number: i32,
    bed_type: String,
    occupied: bool,
    price: i32
}

impl Room
{
    pub fn new(id: i32, number: i32, bed_type: String, occupied: bool, price: i32) -> Self
    {
        Self { id, number, bed_type, occupied, price }
    }
}

#[tauri::command]
pub async fn add_room(number: i32, bed_type: String, occupied: bool, price: i32)
{
    let conn = get_connection().unwrap();

    let room = Room::new(0, number, bed_type, occupied, price);

    conn.execute("INSERT INTO room
        (number, bed_type, occupied, price)
        VALUES (?1, ?2, ?3, ?4)",
        (&room.number, &room.bed_type, &room.occupied, &room.price)
    ).unwrap();
}

#[tauri::command]
pub async fn get_all_rooms() -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, number, bed_type, occupied, price FROM room").unwrap();

    let rooms = stmt.query_map([], |row| {
        Ok(Room::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    }).unwrap();
    
    let mut vec: Vec<Room> = Vec::new();

    for room in rooms
    {
        vec.push(room.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}

#[tauri::command]
pub async fn set_room_occupied(room_id: i32, occupied: bool)
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("UPDATE room SET occupied = ?2 WHERE id = ?1").unwrap();

    stmt.execute(params![room_id, occupied]).unwrap();
}

#[tauri::command]
pub async fn edit_room(id: i32, number: i32, bed_type: String, occupied: bool, price: i32)
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("UPDATE room
        SET number = ?2, bed_type = ?3, occupied = ?4, price = ?5
        WHERE id = ?1").unwrap();

    stmt.execute(params![id, number, bed_type, occupied, price]).unwrap();
}

#[tauri::command]
pub async fn get_room_information(id: i32) -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, number, bed_type, occupied, price
        FROM room
        WHERE id = ?1").unwrap();

    let rooms = stmt.query_map([id], |row| {
        Ok(Room::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?
        ))
    }).unwrap();

    let mut vec: Vec<Room> = Vec::new();

    for room in rooms
    {
        vec.push(room.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}

#[tauri::command]
pub async fn remove_room(room_id: i32)
{
    let conn = get_connection().unwrap();

    conn.execute("DELETE FROM room WHERE id = ?1", [room_id]).unwrap();
}

#[tauri::command]
pub async fn get_room_price(room_id: i32) -> i32
{
    let conn = get_connection().unwrap();

    conn.query_row(
        "SELECT price FROM room WHERE id = ?1",
        [room_id],
        |row| row.get(0)
    ).unwrap()
}

#[tauri::command]
pub async fn get_room_bed_type(room_id: i32) -> String
{
    let conn = get_connection().unwrap();

    conn.query_row(
        "SELECT bed_type FROM room WHERE id = ?1",
        [room_id],
        |row| row.get(0)
    ).unwrap()
}
