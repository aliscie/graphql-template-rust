use async_graphql::{Context, EmptyMutation, EmptySubscription,MergedObject, Object, Schema, Subscription};
use super::models::{self, Entity as Team};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

#[derive(Default)]
pub struct TeamsQuery;

#[Object]
impl TeamsQuery {
    async fn teams<'a>(&self, ctx: &Context<'a>) -> Result<Vec<super::models::Model>, DbErr> {
        let database = ctx.data_unchecked::<DatabaseConnection>();
        Team::find().all(database).await
    }

}
