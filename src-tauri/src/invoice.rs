use crate::db::get_connection;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
struct Invoice
{
    id: i32,
    guest_id: i32,
    items_json: String,
    amount_to_pay: i32,
    date: String,
    due_date: String,
    has_paid: bool
}

impl Invoice
{
    fn new(id: i32, guest_id: i32, items_json: String, amount_to_pay: i32, date: String, due_date: String, has_paid: bool) -> Self
    {
        Self { id, guest_id, items_json, amount_to_pay, date, due_date, has_paid }
    }
}

#[tauri::command]
pub async fn get_all_invoices() -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, guest_id, items_json, amount_to_pay, date, due_date, has_paid FROM invoice").unwrap();

    let invoices = stmt.query_map([], |row| {
        Ok(Invoice::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?
        ))
    }).unwrap();
    
    let mut vec: Vec<Invoice> = Vec::new();

    for invoice in invoices
    {
        vec.push(invoice.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}

#[tauri::command]
pub async fn add_invoice(guest_id: i32, items_json: String, amount_to_pay: i32, date: String, due_date: String, has_paid: bool) -> i32
{
    let conn = get_connection().unwrap();

    let room = Invoice::new(0, guest_id, items_json, amount_to_pay, date, due_date, has_paid);

    conn.execute("INSERT INTO invoice
        (guest_id, items_json, amount_to_pay, date, due_date, has_paid)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&room.guest_id, &room.items_json, &room.amount_to_pay, &room.date, &room.due_date, &room.has_paid)
    ).unwrap();

    conn.query_row("SELECT last_insert_rowid()", [], |row| row.get(0)).unwrap()
}

#[tauri::command]
pub async fn set_invoice_payment_status(guest_id: i32, status: bool)
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("UPDATE invoice SET has_paid = ?2 WHERE guest_id = ?1").unwrap();

    stmt.execute(params![guest_id, status]).unwrap();
}

#[tauri::command]
pub async fn get_invoice_information(id: i32) -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("
        SELECT id, guest_id, items_json, amount_to_pay, date, due_date, has_paid 
        FROM invoice
        WHERE id = ?1"
    ).unwrap();

    let invoices = stmt.query_map([id], |row| {
        Ok(Invoice::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?
        ))
    }).unwrap();

    let mut vec: Vec<Invoice> = Vec::new();

    for invoice in invoices
    {
        vec.push(invoice.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}
