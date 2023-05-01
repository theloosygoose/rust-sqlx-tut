use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://saves/save_01.db";

use serde::{Serialize, Deserialize};


#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct Personality {
    pub work_ethic: u16,
    pub dog: u16,
    pub loyalty: u16,
}

#[derive(Clone, FromRow, Debug, Serialize)]
pub struct InsertPlayer {
    pub name: String,
    pub active: bool,
    pub player_id: String,

    #[serde(flatten)]
    pub personality: Personality,
}

#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct GetPlayer {
    pub name: String,
    pub active: bool,
    pub player_id: String,

    #[sqlx(flatten)]
    pub personality: Personality,
}


#[tokio::main]
pub async fn create_game() {
    //CHECKS FOR SAVEGAME IF NONE EXISTS THEN CREATE ONE 
    //IF EXISTS READ THE SAVE
    
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating savefile {}", DB_URL);

        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create savefile success"),
            Err(error) => panic!("Error creating savefile !!! {}", error),
        }
    } else {
        println!("Save already exists");
    }

    //Then Connect to the database via pool
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    //Gets root dir of application and migrations file
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    //Get migrations
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("Error In Migration: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    //Create a player to insert into the databse 
    let player_01 = InsertPlayer{
        name: "Joel".to_string(),
        active: true,
        player_id: "joel20023".to_string(),
        personality: Personality { 
            work_ethic: 22, 
            dog: 50, 
            loyalty: 99 
        }
    };

    println!("Attempting to insert into savefile {:#?}", player_01);
    

    let result = sqlx::query(
        "INSERT INTO players 
        (name,player_id,active,work_ethic,dog,loyalty) VALUES (?, ?, ?, ?, ?, ?)"
    )
        .bind(player_01.name)
        .bind(player_01.player_id)
        .bind(player_01.active)
        .bind(player_01.personality.work_ethic)
        .bind(player_01.personality.dog)
        .bind(player_01.personality.loyalty)
        .execute(&db)
        .await
        .unwrap();
    
    println!("Query result: {:?}", result);

    let players_results = sqlx::query_as::<_, GetPlayer>(
        "SELECT id, player_id, name, active, work_ethic, dog, loyalty FROM players"
    )
        .fetch_all(&db)
        .await
        .unwrap();
    
    for player in players_results {
        println!("name: {}, personality: {:#?}, active: {}", &player.name, &player.personality, player.active);
    } 

}
