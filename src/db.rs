use crate::score::Score;
use mysql::prelude::*;
use mysql::*;

pub struct Db {
    connection: PooledConn,
    table: String,
}

impl Db {
    pub fn new(
        password: &str,
        table: &str,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let conn = Self::create_connection(password, "localhost:3306/code_challenge")?;

        let mut me = Db {
            connection: conn,
            table: table.to_string(),
        };

        me.create_table()?;

        Ok(me)
    }

    fn create_connection(
        password: &str,
        url: &str,
    ) -> Result<PooledConn, Box<dyn std::error::Error>> {
        let pool = Pool::new(format!("mysql://root:{}@{}", password, url).as_str())?;
        Ok(pool.get_conn()?)
    }

    fn create_table(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let query = format!(
            "CREATE TABLE if not exists {} ( {} )",
            self.table,
            Score::schema()
        );
        self.connection.query_drop(query)?;
        Ok(())
    }

    pub fn insert_score(
        &mut self,
        name: &str,
        command: &str,
        time_ns: i32,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let score: Score = Score::new(name, command, time_ns);
        let (statement, parameters) = score.as_insert();
        let statement = format!("INSERT INTO {} {}", self.table, statement);
        self.connection.exec_drop(statement, parameters)?;
        Ok(())
    }

    pub fn get_scores(&mut self) -> std::result::Result<Vec<Score>, Box<dyn std::error::Error>> {
        let scores = self.connection.query_map(
            "SELECT name, command, time_ns FROM 23_3_1 ORDER BY time_ns ASC",
            |(name, command, time_ns)| Score {
                name,
                command,
                time_ns,
            },
        )?;

        println!("Scores: {:?}", scores);
        Ok(scores)
    }

    pub fn clear_table(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let drop = format!("DROP TABLE IF EXISTS {}", self.table);
        self.connection.query_drop(drop.as_str())?;
        Ok(())
    }
}
