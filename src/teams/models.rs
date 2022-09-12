use async_graphql::{ComplexObject, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "Team")]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::users::Entity")]
    Members,
}

impl Related<crate::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model{}
