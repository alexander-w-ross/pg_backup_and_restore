use clap::{App, Arg, SubCommand};
use std::process::Command;
use std::fs;
use std::path::Path;
use chrono;

fn main() {
    let app = App::new("Database Backup and Restore")
        .version("0.1.0")
        .author("Alec Ross")
        .about("Backup and restore PostgreSQL databases");

    let common_args = vec![
        Arg::new("host")
            .short('H')
            .long("host")
            .value_name("HOST")
            .help("Sets the host to connect to")
            .required(true),

        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port to connect to")
            .default_value("5432"),

        Arg::new("username")
            .short('u')
            .long("username")
            .value_name("USERNAME")
            .help("Username to connect with")
            .required(true),

        Arg::new("database")
            .short('d')
            .long("database")
            .value_name("DATABASE")
            .help("Database to connect to")
            .required(true),
    ];

    let dump = SubCommand::with_name("dump")
        .about("Backup a dump database")
        .args(
            &[
                Arg::new("output_file_location")
                    .short('o')
                    .long("output-file-location")
                    .value_name("OUTPUT_FILE_LOCATION")
                    .help("Location to save the backup file")
                    .default_value("."),
            ]
            .iter()
            .chain(common_args.iter())
            .cloned()
            .collect::<Vec<_>>(),
        );

        let seed = SubCommand::with_name("seed")
        .about("Backup a seed database")
        .args(
            &[
                Arg::new("output_file_location")
                    .short('o')
                    .long("output-file-location")
                    .value_name("OUTPUT_FILE_LOCATION")
                    .help("Location to search for the backup file")
                    .default_value("."),
            ]
            .iter()
            .chain(common_args.iter())
            .cloned()
            .collect::<Vec<_>>(),
        );
    

    let matches = app.subcommand(dump).subcommand(seed).get_matches();

    if let Some(matches) = matches.subcommand_matches("dump") {
        println!("Dumping database...");
        let host = matches.value_of("host").unwrap();
        let port = matches.value_of("port").unwrap();
        let username = matches.value_of("username").unwrap();
        let database = matches.value_of("database").unwrap();
        let output_file_location = matches.value_of("output_file_location").unwrap();

        let backup_file = format!(
            "{}/{}{}_backup.sql",
            output_file_location,
            chrono::offset::Local::now().format("%Y%m%d_%H%M%S"),
            database
        );

        println!("Saving backup to: {}", backup_file);
        let mut pg_dump = Command::new("pg_dump")
            .arg("-h")
            .arg(host)
            .arg("-p")
            .arg(port)
            .arg("-U")
            .arg(username)
            .arg("-W") // force password
            .arg(database)
            .arg("-f") // output filename
            .arg(&backup_file)
            .spawn()
            .expect("pg_dump failed to start");

        let _ = pg_dump.wait().expect("pg_dump failed to run");

        println!("Database dumped successfully");

    } else if let Some(matches) = matches.subcommand_matches("seed") {
        println!("Seeding database...");
        let _host = matches.value_of("host").unwrap();
        let _port = matches.value_of("port").unwrap();
        let username = matches.value_of("username").unwrap();
        let database = matches.value_of("database").unwrap();
        let output_file_location = matches.value_of("output_file_location").unwrap_or(".");

        let backup_dir = Path::new(output_file_location);
        let mut latest_backup_file: Option<(String, std::time::SystemTime)> = None;

        println!("Searching for backup file in: {}", backup_dir.display());
        if let Ok(entries) = fs::read_dir(backup_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "sql" {
                            if let Ok(metadata) = entry.metadata() {
                                let modified = metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                                if latest_backup_file
                                    .as_ref()
                                    .map(|(_, latest_modified)| modified > *latest_modified)
                                    .unwrap_or(true)
                                {
                                    latest_backup_file = Some((path.to_string_lossy().into_owned(), modified));
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some((backup_file, _)) = latest_backup_file {
            println!("Using backup file: {}", backup_file);

            let file = std::fs::File::open(&backup_file).expect("Failed to open backup file");


            let mut seed_db = Command::new("psql")
            .arg("-U")
            .arg(username)
            .arg("-d")
            .arg(database)
            .stdin(std::process::Stdio::from(file))
            .spawn()
            .expect("seed_db failed to start");

            let _ = seed_db.wait().expect("seed_db failed to run");
            
            println!("Database seeded successfully");
        } else {
            println!("No backup file found in the specified directory");
        }
    }
}