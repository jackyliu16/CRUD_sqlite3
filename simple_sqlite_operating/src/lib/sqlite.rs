use rusqlite::{Connection, Result};


#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use rusqlite::{Connection, Result};
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
}