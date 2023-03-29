use clap::{App, Arg};
use std::process::Command;
use chrono;
// pg_dump -h <host> -p <port> -U <username> -W -F t <database_name> > backup_file.tar
fn main() {
    let matches = App::new("Database Backup and Restore")
        .version("0.1.0")
        .author("Alec Ross")
        .about("Backup and restore PostgreSQL databases")
        .arg(
            Arg::new("host")
            .short('h')
            .long("host")
            .value_name("HOST")
            .help("Sets the host to connect to")
            .required(true)
        )
        .arg(
            Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port to connect to")
            .default_value("5432")
        )
        .arg(
            Arg::new("username")
            .short('U')
            .long("username")
            .value_name("USERNAME")
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
        .arg(
            Arg::new("local_username")
            .short('u')
            .long("local-username")
            .value_name("LOCAL_USERNAME")
            .help("Local username to connect with")
            .required(true)
        )
        .arg(
            Arg::new("local_db")
            .short('l')
            .long("local-db")
            .value_name("LOCAL_DB")
            .help("Local database to connect to")
            .required(true)
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();
    let username = matches.value_of("username").unwrap();
    let remote_db = matches.value_of("remote_db").unwrap();
    let local_username = matches.value_of("local_username").unwrap();
    let local_db = matches.value_of("local_db").unwrap();

    let backup_file = format!("{}{}_backup.pg_dump", chrono::offset::Local::now(),remote_db);

    // 1. Backup staging daatabase
    let mut pg_dump = Command::new("pg_dump")
    .arg("-h")
    .arg(host)
    .arg("-p")
    .arg(port)
    .arg("-U")
    .arg(username)
    .arg("-W") // force password
    .arg("-F") // format
    .arg("t") // dump specified tables only
    .arg(remote_db)
    .arg("-f") // output filename
    .arg(&backup_file)
    .spawn()
    .expect("pg_dump failed to start");

    let _ = pg_dump.wait().expect("pg_dump failed to run");
    
    // 2. Create new locaal database
    let mut createdb = Command::new("createdb")
    .arg("-U")
    .arg(local_username)
    .arg(local_db)
    .spawn()
    .expect("createdb failed to start");

    let _ = createdb.wait().expect("createdb failed to run");

    // 3. Restore backup to local database
    let mut pg_restore = Command::new("pg_restore")
    .arg("-U")
    .arg(local_username)
    .arg("-d")
    .arg(local_db)
    .arg(&backup_file)
    .spawn()
    .expect("pg_restore failed to start");

    let _ = pg_restore.wait().expect("pg_restore failed to run");

    // 4. Delete backup file
    let _ = std::fs::remove_file(&backup_file);
    
}
