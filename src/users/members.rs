//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use async_graphql::{ComplexObject, Context, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "Member")]
#[sea_orm(table_name = "members")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::teams::models::Entity",
        from = "Column::TeamId",
        to = "crate::teams::models::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Teams,
}

impl Related<crate::teams::models::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model {
    async fn team(&self, ctx: &Context<'_>) -> Result<crate::teams::Model, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        self.find_related(crate::teams::Entity)
            .one(db)
            .await
            .map(|b| b.unwrap())
    }
}
