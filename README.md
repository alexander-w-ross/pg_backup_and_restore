# PostgreSQL Backup and Restore Script
`pg_backup_and_restore` is a command-line utility that simplifies the process of backing up a PostgreSQL database from a staging server and restoring it to a local database. This utility is particularly useful for developers who need to work with an up-to-date copy of a staging database on their local machines.

## Requirements

- Rust 1.31 or later
- PostgreSQL client tools (`pg_dump`, `createdb`, and `pg_restore`) installed and available in your system's `PATH`

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

5. Usage

```bash
pg_backup_and_restore -h <host> -p <port> -U <username> -s <staging_db> -u <local_username> -l <local_db>
```

Replace the following placeholders with the appropriate values:

- `<host>`: The staging database host
- `<port>`: The staging database port (default is 5432)
- `<username>`: The staging database username
- `<staging_db>`: The staging database name
- `<local_username>`: The local database username
- `<local_db>`: The local database name
The script will prompt you to enter the password for the staging database user.

Example:

```bash
pg_backup_and_restore -h staging.example.com -p 5432 -U myuser -s my_staging_db -u mylocaluser -l my_local_db
```

This command will backup the my_staging_db database from the staging server and restore it to the my_local_db local database.