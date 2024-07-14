use crate::db::get_connection;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all="camelCase")]
struct Report
{
    id: i32,
    guest_id: i32,
    invoice_id: i32,
    room_id: i32,
    check_in_date: String,
    actual_check_in_date: Option<String>,
    check_out_date: String,
    actual_check_out_date: Option<String>
}

impl Report
{
    fn new(id: i32, guest_id: i32, invoice_id: i32, room_id: i32, check_in_date: String, actual_check_in_date: Option<String>, check_out_date: String, actual_check_out_date: Option<String>) -> Self
    {
        Self { id, guest_id, invoice_id, room_id, check_in_date, actual_check_in_date, check_out_date, actual_check_out_date }
    }
}

#[tauri::command]
pub async fn get_all_reports() -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, guest_id, invoice_id, room_id, checkin_date, actual_checkin_date, checkout_date, actual_checkout_date FROM report").unwrap();

    let reports = stmt.query_map([], |row| {
        Ok(Report::new(
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
    
    let mut vec: Vec<Report> = Vec::new();

    for report in reports
    {
        vec.push(report.unwrap());
    }

    serde_json::to_string_pretty(&vec).unwrap()
}

#[tauri::command]
pub async fn add_report(guest_id: i32, invoice_id: i32, room_id: i32, check_in_date: String, check_out_date: String) -> i32
{
    let conn = get_connection().unwrap();

    let report = Report::new(0, guest_id, invoice_id, room_id, check_in_date, None, check_out_date, None);

    conn.execute("INSERT INTO report
        (guest_id, invoice_id, room_id, checkin_date, actual_checkin_date, checkout_date, actual_checkout_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&report.guest_id, &report.invoice_id, &report.room_id, &report.check_in_date, &report.actual_check_in_date, &report.check_out_date, &report.actual_check_out_date)
    ).unwrap();

    conn.query_row("SELECT last_insert_rowid()", [], |row| row.get(0)).unwrap()
}

#[tauri::command]
pub async fn set_report_actual_check_in_date(guest_id: i32, date: String)
{
    let conn = get_connection().unwrap();

    conn.execute("UPDATE report SET actual_checkin_date = ?2 WHERE guest_id = ?1", params![guest_id, date]).unwrap();
}

#[tauri::command]
pub async fn set_report_actual_check_out_date(guest_id: i32, date: String)
{
    let conn = get_connection().unwrap();

    conn.execute("UPDATE report SET actual_checkout_date = ?2 WHERE guest_id = ?1", params![guest_id, date]).unwrap();
}
