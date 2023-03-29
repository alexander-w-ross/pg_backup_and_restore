use clap::{App, Arg};
use std::process::Command;
use chrono;

fn main() {
    let matches = App::new("Database Backup and Restore")
        .version("0.1.0")
        .author("Alec Ross")
        .about("Backup and restore PostgreSQL databases")
        .arg(
            Arg::new("remote_host")
            .short('h')
            .long("remote-host")
            .value_name("REMOTE_HOST")
            .help("Sets the host to connect to")
            .required(true)
        )
        .arg(
            Arg::new("remote_port")
            .short('p')
            .long("remote-port")
            .value_name("REMOTE_PORT")
            .help("Sets the remote port to connect to")
            .default_value("5432")
        )
        .arg(
            Arg::new("remote_username")
            .short('u')
            .long("remote-username")
            .value_name("REMOTE_USERNAME")
            .help("Remote username to connect with")
            .required(true)
        )
        .arg(
            Arg::new("remote_db")
            .short('d')
            .long("remote-db")
            .value_name("REMOTE_DB")
            .help("Remote database to connect to")
            .required(true)
        )
        .get_matches();

    let remote_host = matches.value_of("remote_host").unwrap();
    let remote_port = matches.value_of("remote_port").unwrap();
    let remote_username = matches.value_of("remote_username").unwrap();
    let remote_db = matches.value_of("remote_db").unwrap();


    let backup_file = format!("{}{}_backup.sql", chrono::offset::Local::now().format("%Y%m%d_%H%M%S"), remote_db);




    // 1. Backup staging daatabase
    let mut pg_dump = Command::new("pg_dump")
    .arg("-h")
    .arg(remote_host)
    .arg("-p")
    .arg(remote_port)
    .arg("-U")
    .arg(remote_username)
    .arg("-W") // force password
    .arg(remote_db)
    .arg("-f") // output filename
    .arg(&backup_file)
    .spawn()
    .expect("pg_dump failed to start");

    let _ = pg_dump.wait().expect("pg_dump failed to run");
    
}
