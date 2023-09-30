mod config;
use scoreboard_db::{Db, Score};
mod debug_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    debug_config::init_debug(&args);

    let config = config::RunMode::from_args(&args)?;
    println!("Config: {:?}", config);

    let db_pass = match option_env!("DB_PASSWORD") {
        Some(pass) => pass,
        None => {
            return Err(
                "This program needs to be compiled with the $DB_PASS env variable set".into(),
            )
        }
    };

    let mut db = Db::new("localhost", 3306, "code_challenge", db_pass, "23_3_1")?;

    db.insert_score("Charlie", "ls -l", 300)
        .expect("error inserting score");
    db.insert_score("Alice", "ls -l", 100)
        .expect("error inserting score");
    db.insert_score("Bob", "ls -l", 200)
        .expect("error inserting score");
    db.insert_score("Alice", "ls -l", 97)
        .expect("error inserting score");
    let scores: Vec<Score> = db.get_scores().expect("error getting scores");
    println!("Scores: {:?}", scores);
    db.clear_table().expect("error clearing table");

    Ok(())
}
