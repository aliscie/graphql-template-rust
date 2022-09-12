use async_graphql::{Context, EmptyMutation, EmptySubscription, MergedObject, Object, Schema, Subscription};
use crate::chats::{self, Entity as ChatRoom};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn chatrooms<'a>(&self, ctx: &Context<'a>) -> Result<Vec<crate::chats::models::Model>, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        ChatRoom::find().all(db).await
    }

    async fn chatroom<'a>(
        &self,
        ctx: &Context<'a>,
        id: i32,
    ) -> Result<Option<crate::chats::models::Model>, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        ChatRoom::find_by_id(id).one(db).await
    }
}
