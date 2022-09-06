use async_graphql::{ComplexObject, Context, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "Post")]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String, //TODO add data validation, max_length=500
    pub content: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::members::Entity")]
    Member,
}

impl Related<super::members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model {}
