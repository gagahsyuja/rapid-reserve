use crate::db::get_connection;
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
    check_out_date: String
}

impl Report
{
    fn new(id: i32, guest_id: i32, invoice_id: i32, room_id: i32, check_in_date: String, check_out_date: String) -> Self
    {
        Self { id, guest_id, invoice_id, room_id, check_in_date, check_out_date }
    }
}

#[tauri::command]
pub async fn get_all_reports() -> String
{
    let conn = get_connection().unwrap();

    let mut stmt = conn.prepare("SELECT id, guest_id, invoice_id, room_id, checkin_date, checkout_date FROM report").unwrap();

    let reports = stmt.query_map([], |row| {
        Ok(Report::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
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

    let report = Report::new(0, guest_id, invoice_id, room_id, check_in_date, check_out_date);

    conn.execute("INSERT INTO report
        (guest_id, invoice_id, room_id, checkin_date, checkout_date)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        (&report.guest_id, &report.invoice_id, &report.room_id, &report.check_in_date, &report.check_out_date)
    ).unwrap();

    conn.query_row("SELECT last_insert_rowid()", [], |row| row.get(0)).unwrap()
}
