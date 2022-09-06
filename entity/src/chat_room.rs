//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use async_graphql::{ SimpleObject, ComplexObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "ChatRoom")]
#[sea_orm(table_name = "chat_rooms")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub messages: Json,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[ComplexObject]
impl Model{}
