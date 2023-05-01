mod config;
mod database;
pub mod paths;


fn main() {
    println!("MCMF database!");
    
    paths::create_required_directories();
    database::create_members_table();
    
    // inserting a record into the table
    let last_inserted_row_id: i64;
    match database::insert_member("name", "address", "phone", "email", "parish", "ecclesiastical_district", "comments") {
        Ok(id) => {
            last_inserted_row_id = id;
            println!("Successfully added the member and last row_id is {}", id);
        },
        Err(e) => {
            last_inserted_row_id = -9999;
            println!("Error in adding the new member: {}", e);
        }
    };

    println!("Inserted new user with ID {}", last_inserted_row_id);
}
