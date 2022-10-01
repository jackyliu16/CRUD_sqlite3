mod lib;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[allow(dead_code)]
fn main() -> Result<()> {
    // fd::main();
    // let conn = create_db("test.db".to_string())?;
    // insert_data(&conn)?;
    // let persons = get_data(&conn)?;
    // for p in persons {
    //     println!("Hi: {:?}", p);
    // }
    Ok(())
}

#[allow(dead_code)]
fn get_data(conn: &Connection) -> Result<Vec<Person>> {
    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iterator = stmt.query_map([], |row| {
        Ok(Person{
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;
    let mut persons = Vec::new();
    for p in person_iterator {
        persons.push(p?);
    }
    Ok(persons)
}

#[allow(dead_code)]
fn insert_data(conn: &Connection) -> Result<()> {
    let p1 = Person {
        id: 1,
        name: "Jacky".to_string(),
        data: None,
    };
    let p2 = Person {
        id: 2,
        name: "Jack".to_string(),
        data: None,
    };

    for person in [p1, p2] {
        conn.execute(
            "INSERT INTO person (id, name, data)
            VALUES (?1, ?2, ?3), (?4, ?5, ?6);",
            params![person.id, person.name, person.data],
        )?;
    }
    Ok(())
}

#[allow(dead_code)]
fn create_db(file_name: String) -> Result<Connection> {
    let conn = Connection::open(file_name)?;
    let _ = conn.execute("DROP TABLE person", []);

    conn.execute(
        "CREATE TABLE PERSON(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                data BLOB,          
            ",
            [] 
        )?;
    Ok(conn)
}


