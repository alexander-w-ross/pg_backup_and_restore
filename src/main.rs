use clap::{App, Arg};
use std::process:Command;

fn main() {
    let matches = App::new("Database Backup and Restore")
        .version("0.1.0")
        .author("Alec Ross")
        .about("Backup and restore PostgreSQL databases")
}
