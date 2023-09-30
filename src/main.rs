use std::env;
mod config;
mod db;
mod debug_config;
mod score;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    debug_config::init_debug(&args);

    let config = config::RunMode::from_args(&args)?;
    println!("Config: {:?}", config);

    let db_pass = env::var("DB_PASSWORD").expect("DB_PASS must be set");

    let mut db = db::Db::new(&db_pass, "23_3_1")?;

    db.insert_score("Charlie", "ls -l", 300)
        .expect("error inserting score");
    db.insert_score("Alice", "ls -l", 100)
        .expect("error inserting score");
    db.insert_score("Bob", "ls -l", 200)
        .expect("error inserting score");
    db.insert_score("Alice", "ls -l", 97)
        .expect("error inserting score");
    db.get_scores().expect("error getting scores");
    db.clear_table().expect("error clearing table");

    Ok(())
}
