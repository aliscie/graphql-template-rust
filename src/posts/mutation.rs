use async_graphql::{Context, EmptyMutation, EmptySubscription, MergedObject, Object, Schema, Subscription};

use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

use crate::posts::{self, Entity as Post};

#[derive(Default)]
pub struct PostMutation;


#[Object]
impl PostMutation {
    async fn create_post(&self, ctx: &Context<'_>, title: String) -> Result<crate::posts::models::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let res = Post::insert(crate::posts::models::ActiveModel {
            title: ActiveValue::Set(title),
            ..Default::default()
        })
            .exec_with_returning(db)
            .await?;
        return Ok(res);
    }
}
