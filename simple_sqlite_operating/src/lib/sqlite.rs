use rusqlite::{Connection, Result};
use rusqlite::types::Value;

pub fn query_info(conn: &Connection, query: String) -> Result<QueryInfo> {
    let mut stmt = conn.prepare(&query)?;
    let columns = stmt.columns().into_iter().map(|col| {
        ColumnInfo {
            name: col.name().into(),
            declared_type: col.decl_type().map(String::from),
        }
    }).collect::<Vec<_>>();

    let rows: Vec<Vec<RowValue>> = stmt.query_map((), |row| {
        Ok((0..columns.len()).map(|idx| {
            Ok(RowValue { value: row.get(idx)? })
        }).collect::<Result<_>>()?)
    })?.collect::<Result<_>>()?;

    Ok( QueryInfo { 
        query,
        columns,
        rows,
    })
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryInfo {
    pub query: String,
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<RowValue>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub declared_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowValue {
    pub value: Value,
}

impl core::fmt::Display for RowValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self.value {
            Value::Integer(i) => write!(f, "{0: <10}", i),
            Value::Real(n) => write!(f, "{0: <20}", n),
            Value::Text(text) => write!(f, "{0: <25}", text),
            Value::Null => write!(f, "{0: <10}", "(null)"),
            Value::Blob(bytes) => write!(f, "{0:<20}", bytes.len()),
            // Value::Integer(i) => write!(f, "(i64) {:?}", i),
            // Value::Real(n) => write!(f, "(f64) {:?}", n),
            // Value::Text(text) => write!(f, "(text) {:?}", text),
            // Value::Null => write!(f, "(null)"),
            // Value::Blob(bytes) => write!(f, "(blob) <{} bytes>", bytes.len()),
        }
    }
}
#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use super::*;
    use crate::lib::fd::{self, *};

    #[test]
    pub fn test_for_connection() {
        
        let mut cwd = current_dir().unwrap();
        // let file = get_file_under_path(&cwd);
        fd::test::print_everything(&cwd);
        println!("=========================");
        
        move_to_direntry(&mut cwd, "school.db".to_string());
        print_pwd(&cwd);

        // TODO 这个地方需要错误处理，但是我目前掌握相对来说不是很好
        let conn = Connection::open(&cwd).unwrap();
        match conn.path() {
            Some(_) => println!("connection successed"),
            None => println!("connection failure"),
        }
    }

    #[test]
    pub fn test_for_select() {
        let conn = create_conn_to_school_db();
        // run select query
        let query = "SELECT * FROM course";
        let query_info = match query_info(&conn, query.to_string()) {
            Ok(conn) => conn,
            Err(_) => QueryInfo {
                query: String::new(),
                columns: Vec::new(),
                rows: Vec::new(),
            },
        };

        for row in query_info.rows.iter() {
            for col in row.iter() {
                print!("{}", col);
            }
            println!("");
        }
    }

    
    pub fn create_conn_to_school_db() -> Connection{
        let mut cwd = current_dir().unwrap();
        fd::test::print_everything(&cwd);
        println!("==========================");

        move_to_direntry(&mut cwd, "school.db".to_string());
        print_pwd(&cwd);

        let conn = Connection::open(&cwd).unwrap();
        match conn.path() {
            Some(_) => println!("connection successed"),
            None => println!("connection failure"),
        }
        conn
    }
}