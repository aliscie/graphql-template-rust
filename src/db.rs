use sea_orm::prelude::*;
use dotenv::dotenv;
use sea_orm::Database;

async fn init_pool(database_url : &str) -> Result<DatabaseConnection, DbErr>{
    Database::connect(database_url).await
}

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url =  std::env::var("DATABASE_URL").expect("POSTGRES url not given!");
    init_pool(&database_url).await.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

