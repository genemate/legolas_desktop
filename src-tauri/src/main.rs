// LE.GO.LAS Desktop — main binary entry point.
// The actual application logic lives in lib.rs.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    steptwo_desktop_lib::run();
}
