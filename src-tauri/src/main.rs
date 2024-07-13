// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod room;
mod guest;
mod invoice;
mod report;

use rusqlite::Error;
use db::*;
use guest::*;
use room::*;
use invoice::*;
use report::*;

fn main() -> Result<(), Error>
{
    match get_connection()
    {
        Ok(conn) => setup(conn)?,
        Err(e) => panic!("Failed to get connection to database! ({})", e)
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_all_guests,
            add_guest,
            remove_guest,
            get_guest_information,
            edit_guest,
            set_payment_status,

            get_all_rooms,
            add_room,
            get_room_information,
            remove_room,
            edit_room,
            set_room_occupied,
            get_room_price,

            get_all_reports,
            add_report,

            get_all_invoices,
            add_invoice,
            set_invoice_payment_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
