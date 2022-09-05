pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220905_111757_create_members;
mod m20220905_151853_create_chat_room;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220905_111757_create_members::Migration),
            Box::new(m20220905_151853_create_chat_room::Migration),
        ]
    }
}
