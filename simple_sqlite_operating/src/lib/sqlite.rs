use rusqlite::{Connection, Result};


#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use rusqlite::{Connection, Result, Column};
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
            Some(conn) => println!("connection successed"),
            None => println!("connection failure"),
        }


        // let conn = Connection::open()
    }

    use rusqlite::params;
    #[test]
    pub fn test_for_connection_and_execute() {
        let mut cwd = current_dir().unwrap();
        // let file = get_file_under_path(&cwd);
        fd::test::print_everything(&cwd);
        println!("=========================");
        
        move_to_direntry(&mut cwd, "school.db".to_string());
        print_pwd(&cwd);

        // TODO 这个地方需要错误处理，但是我目前掌握相对来说不是很好
        let conn = Connection::open(&cwd).unwrap();
        match conn.path() {
            Some(conn) => println!("connection successed"),
            None => println!("connection failure"),
        }
        //.head on
        //.mode column 
        let stmt = conn.prepare("SELECT * FROM ADVISOR").unwrap();
        let col_num = stmt.column_count();
        let cols = stmt.columns();
        
        // let mut table = Vec::new();

        // println!("col: {}", col_num);

        let mut stmt = conn.prepare("SELECT * FROM ADVISOR").unwrap();
        let mut rows = stmt.query([]).unwrap();

        // let mut table:Vec<_> = Vec::new();
        // for j in 0..col_num {
        //     let tmp:Vec<_> = Vec::new();
        //     while let Some(row) = rows.next().unwrap() {
        //         tmp.push(row.get(j).unwrap());
        //     }
        //     println!("{:?}", tmp);
        //     table.push(tmp);
        // }
        // let mut vec_of_object = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            // let mut tmp: Vec<String> = Vec::new();
            for i in 0..col_num {
                let a: String = row.get(i).unwrap();
                print!("{:?}", a);
            } 
            println!("");
            // println!("tmp: {:?}", tmp
            // vec_of_object.push(tmp);
        }
    
        // for row in vec_of_object {
        //     println!("{:?}", row);
        // }

        // let conn = Connection::open()
    }

    pub fn col_type_check(cols: &Vec<Column<'_>>, idx: usize) -> String {
        cols[idx].decl_type().unwrap().to_string()
    }

}