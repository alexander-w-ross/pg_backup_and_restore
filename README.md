# Database Backup and Restore

This script allows you to backup and restore PostgreSQL databases using the pg_dump command. It supports backing up both remote and local databases and provides a command-line interface for specifying various options, such as host, port, username, and database.

## Requirements

- Rust programming language installed
- PostgreSQL installed with `pg_dump` available in the system path
- Clap and Chrono Rust crates

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/pg_backup_and_restore.git
```

2. Change to the repository directory:

```bash
cd pg_backup_and_restore
```

3. Build the project:

```bash
cargo build --release
```

4. Copy the compiled binary to a directory in your system's `PATH`, for example:

```bash
cp target/release/pg_backup_and_restore /usr/local/bin/
```

## Usage

```bash
USAGE: 
    pg_backup_and_restore [SUBCOMMAND]

SUBCOMMANDS
    dump: Backup a database
    OPTIONS
        -o, --output-file-location OUTPUT_FILE_LOCATION: Location to save the backup file (default: .)
    
    seed: Seed a local database
        
OPTIONS
-h, --host HOST: Sets the host to connect to (required)
-p, --port PORT: Sets the port to connect to (default: 5432)
-u, --username USERNAME: Sets the username to connect with (required)
-d, --database DATABASE: Sets the database to connect to (required)
```

Then, you can run the compiled

## Examples

To backup a database:

```bash
$ pg_backup_and_restore dump -h <host> -u <username> -d <database>
```

To seed a database:

```bash
$ pg_backup_and_restore seed -h localhost -u myuser -d mydatabase
```

To seed a database using a backup file from a custom output file location:

```bash
$ pg_backup_and_restore seed -h localhost -u myuser -d mydatabase -o /path/to/backups
```

## How It Works

The application uses the Clap crate to define the command-line interface and parse the provided arguments. Based on the provided subcommand (`dump` or `seed`), the application will either create a backup dump of a database or seed a database using a backup file.

When creating a backup dump, the application generates a backup filename with the current timestamp and the specified output file location. Then, it runs the `pg_dump` command with the provided options and saves the backup file to the specified location.

When seeding a database, the application scans the specified directory (or `/tmp` by default) for the most recent `.sql` backup file. If it finds a suitable backup file, it runs the `psql` command with the provided options and seeds the database using the backup file. If no suitable backup file is found, it prints a message and exits.

## Limitations

This application currently only supports PostgreSQL databases and relies on the `pg_dum`p and `psql` commands being available in the system path