use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::Teams;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Members::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Members::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Members::Name).string().not_null())
                    .col(ColumnDef::new(Members::Knockouts).integer().not_null())
                    .col(ColumnDef::new(Members::TeamId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Members::Table)
                            .from_col(Members::TeamId)
                            .to_tbl(Teams::Table)
                            .to_col(Teams::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE `members`";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}

#[derive(Iden)]
pub enum Members {
    Table,
    Id,
    Name,
    Knockouts,
    TeamId,
}
