use std::fs;
use std::path::Path;
use libminecorrosion::configure::build_cmdline;
use libminecorrosion::database::initialize_database;

fn main() {
    println!("minecmd");

    // build_cmdline();
    let x = initialize_database("/Volumes/tmpfs/minecorrosion_test.sqlite3".to_string());
    println!("break");
}
