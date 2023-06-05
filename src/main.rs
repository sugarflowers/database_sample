use rusqlite::{params, Connection, ToSql};
use std::error::Error;

#[derive()]
struct Data {
    id: i32,
    name: String,
}
#[derive()]
struct Count {
    count: i32,
}

#[derive()]
struct Conn {
    conn: Connection,
}


impl Conn {
    fn new() -> Self {
        Conn {
            conn: Connection::open_in_memory().unwrap(),
        }
    }

    fn open(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        self.conn = Connection::open(filename)?; 
        Ok(())
    }

    fn execute(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<(), Box<dyn Error>> {
        let conn = &mut self.conn;
        let trans = conn.transaction()?;
        trans.execute(sql, params)?;
        trans.commit()?;
        Ok(())
    }

    fn fetch(&mut self, sql: &str, params: &[&dyn ToSql]) -> Vec<Result<Data, rusqlite::Error>> {
        let conn = &mut self.conn;
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map(params, |row| {
            Ok(Data {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        }).unwrap().collect()
    }

    fn count(&mut self) -> i32 {
        let conn = &mut self.conn;
        let mut stmt = conn.prepare(r#"select count(*) from sample_table"#).unwrap();
        let rows = stmt.query_map(params![], |row| {
            Ok(Count {
                count: row.get(0).unwrap(),
            })
        }).unwrap();
        let mut count = 0;
        for row in rows {
            count = row.unwrap().count;
        }
        count
    }
}
fn main() {

    let mut c = Conn::new();
    c.open("sample.sqlite").unwrap();

    let sql = r#"CREATE TABLE IF NOT EXISTS "sample_table" (
        "id" INTEGER,
        "name" TEXT,
        PRIMARY KEY("id" AUTOINCREMENT)
    )"#;

    c.execute(sql, params![]).unwrap();
    c.execute("insert into sample_table (name) values (?1)", params!["hanako".to_string()]).unwrap();

    let rows = c.fetch("select * from sample_table", params![]);
    for row in rows {
        let r = row.unwrap();
        println!("{} {}", r.id, r.name);
    }

    let cnt = c.count();
    println!("{}", cnt);

    /*
    let sql = r#"delete from sample_table"#;
    c.execute(sql, params![]).unwrap();
    */
}
