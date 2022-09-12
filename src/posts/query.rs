use async_graphql::{Context, EmptyMutation, EmptySubscription,MergedObject, Object, Schema, Subscription};
use crate::users::{self, Entity as Member};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

#[derive(Default)]
pub struct MembersQuery;

#[Object]
impl MembersQuery {
    async fn members<'a>(&self, ctx: &Context<'a>) -> Result<Vec<crate::users::Model>, DbErr> {
        let database = ctx.data_unchecked::<DatabaseConnection>();
        Member::find().all(database).await
    }

}
