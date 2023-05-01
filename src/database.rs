use rusqlite::{Connection, Result, params};


pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("assets/sqlite.db")?;
    Ok(conn)
}


pub fn create_table(table_name: &str, columns: Vec<String>, conn: &Connection) -> Result<()> {

    let create_statement = format!("CREATE TABLE IF NOT EXISTS {} (\n{}\n)", table_name, columns.iter()
        .map(|s| if s == "id" {format!("    {} INTEGER PRIMARY KEY", s)} else {format!("    {} TEXT NOT NULL", s)})
        .collect::<Vec<_>>()
        .join(",\n")
    );
    
    println!("{}", create_statement);
    // let table_name_str: &str = table_name;
    conn.execute(
        &create_statement,
        [],
    )?;

    Ok(())
}


const MEMBERS_TABLE_NAME: &str = "members";


pub fn create_members_table() {
    // CREATION OF TABLE in DB
    match create_connection() {
        Ok(conn) => {
            let columns: Vec<String> = vec!["id","name","address","phone","email","parish","ecclesiastical_district","comments"]
                .iter().map(|&s| s.to_owned()).collect();
            match create_table(MEMBERS_TABLE_NAME, columns, &conn) {
                Ok(()) => println!("Successfully created '{}' table", MEMBERS_TABLE_NAME),
                Err(er) => panic!("Unable to create table '{}' due to the error: '{:?}'", MEMBERS_TABLE_NAME, er)
            };
        },
        Err(error) => println!("Unable to create connection due to the error: {}", error)
    };
}


pub fn insert_member(
    name: &str, address: &str, phone: &str, email: &str, parish: &str, ecclesiastical_district: &str, comments: &str) -> Result<i64> {
    let id: i64;
    match create_connection() {
        Ok(conn) => {
            conn.execute(
                &format!("INSERT INTO {MEMBERS_TABLE_NAME} (name, address, phone, email, parish, ecclesiastical_district, comments) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"),
                params![name, address, phone, email, parish, ecclesiastical_district, comments],
            )?;
            id = conn.last_insert_rowid();
        },
        Err(error) => {
            // id = -9999;
            panic!("Unable to create this new member due to {}", error);
        }
    };
    Ok(id)
}

