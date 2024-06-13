use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::engine::local::Mem;
use surrealdb::opt::RecordId;
use surrealdb::Surreal;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: RecordId, // record id of this Person record
    name: String,
    location_rid: RecordId, // RecordId is a reference to a Location record
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    id: RecordId,    // record id of this record
    address: String, // address of this location
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new SurrealDB instance
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test").use_db("test").await?;

    let table = "location_tbl";
    let address = "123 Main St";
    let location_123_main_st_response = db
        .query(r#"CREATE ONLY $table SET address = $addr;"#)
        .bind(("table", table))
        .bind(("address", address))
        .await?;
    dbg!(&location_123_main_st_response);
    let mut location_123_main_st_response = db
        .query(r#"CREATE ONLY location_tbl SET address = $address;"#) // OK
        .bind(("address", address))
        .await?;
    dbg!(&location_123_main_st_response);

    let location_123_main_st_result: Option<Location> = location_123_main_st_response.take(0)?;
    dbg!(&location_123_main_st_result);

    // Get the Location record
    let location_123_main_st: Location = location_123_main_st_result.unwrap();
    dbg!(&location_123_main_st);

    // Add a Person "Big Dude" to person_tbl and reference the Location of 123_main_st
    let name = "Big Dude";
    let mut person_big_dude_response = db
        .query(r#"CREATE ONLY person_tbl SET name = $name, location_rid = $location_rid"#)
        .bind(("name", name))
        .bind(("location_rid", &location_123_main_st.id))
        .await?;
    dbg!(&person_big_dude_response);
    let person_big_dude_result: Option<Person> = person_big_dude_response.take(0)?;
    dbg!(&person_big_dude_result);

    // Get the Person record
    let person_big_dude: Person = person_big_dude_result.unwrap();
    dbg!(&person_big_dude);

    // Add a Person "Little Dude" to person_tbl and reference the Location of 123_main_st
    let name = "Little Dude";
    let mut person_little_dude_response = db
        .query(r#"CREATE ONLY person_tbl SET name = $name, location_rid = $location_rid"#)
        .bind(("name", name))
        .bind(("location_rid", &location_123_main_st.id))
        .await?;
    dbg!(&person_little_dude_response);
    let person_little_dude_result: Option<Person> = person_little_dude_response.take(0)?;
    dbg!(&person_little_dude_result);

    // Get the Person record
    let person_little_dude = person_little_dude_result.unwrap();
    dbg!(&person_little_dude);

    // Deference the Location record from the Person record
    let location_123_main_st_response: Option<Location> =
        db.select(person_big_dude.location_rid).await?;
    dbg!(&location_123_main_st_response);

    //let table = person_little_dude.id.tb;
    let mut persons_response = db
        //.query(r#"SELECT * FROM $rid"#).bind(("rid", person_big_dude.id)) // OK, return only person_big_dude
        //.query(r#"SELECT * FROM $rid"#).bind(("rid", person_little_dude.id)) // OK, return only person_little_dude
        .query(r#"SELECT * FROM person_tbl"#) // OK, retuns all records
        //.query(format!(r#"SELECT * FROM {table}"#)) // OK, return all records
        //.query(r#"SELECT * FROM $table"#).bind(("table", person_big_dude.id.tb)) // error: "invalid type: string \"person_tbl\", expected struct Person"
        //.query(r#"SELECT * FROM "person_tbl""#)                  // error: "invalid type: string \"person_tbl\", expected struct Person"
        .await?;
    dbg!(&persons_response);

    let persons: Vec<Person> = persons_response.take(0)?;
    dbg!(&persons);

    Ok(())
}
