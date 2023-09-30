use mysql::prelude::*;
use mysql::*;
use std::env;

#[derive(Debug, PartialEq, Eq)]
struct Score {
    name: String,
    command: String,
    time_ns: i32,
}

fn main() {
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASS must be set");
    create_table(&db_pass).expect("error creating table");

    clear_table(&db_pass).expect("error clearing table");
    insert_score(&db_pass, "Charlie", "ls -l", 300).expect("error inserting score");
    insert_score(&db_pass, "Alice", "ls -l", 100).expect("error inserting score");
    insert_score(&db_pass, "Bob", "ls -l", 200).expect("error inserting score");
    insert_score(&db_pass, "Alice", "ls -l", 97).expect("error inserting score");
    get_scores(&db_pass).expect("error getting scores");
}

fn create_table(password: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = format!("mysql://root:{}@localhost:3306/code_challenge", password);
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r"
        CREATE TABLE if not exists 23_3_1 (
            id INT NOT NULL AUTO_INCREMENT,
            name TEXT NOT NULL,
            command TEXT NOT NULL,
            time_ns INT NOT NULL,
            PRIMARY KEY (id)
        )",
    )?;

    Ok(())
}

fn insert_score(
    password: &str,
    name: &str,
    command: &str,
    time_ns: i32,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = format!("mysql://root:{}@localhost:3306/code_challenge", password);
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;

    conn.exec_drop(
        r"
        INSERT INTO 23_3_1 (name, command, time_ns)
        VALUES (:name, :command, :time_ns)",
        params! {
            "name" => name,
            "command" => command,
            "time_ns" => time_ns,
        },
    )?;

    Ok(())
}

fn get_scores(password: &str) -> std::result::Result<Vec<Score>, Box<dyn std::error::Error>> {
    let url = format!("mysql://root:{}@localhost:3306/code_challenge", password);
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;

    let scores = conn.query_map(
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

fn clear_table(password: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = format!("mysql://root:{}@localhost:3306/code_challenge", password);
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;

    conn.query_drop("DELETE FROM 23_3_1")?;

    Ok(())
}
