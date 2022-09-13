use async_graphql::{Context, EmptyMutation, EmptySubscription, MergedObject, Object, Schema, Subscription};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

use super::models::{self, Entity as Team};

#[derive(Default)]
pub struct TeamMutation;


#[Object]
impl TeamMutation {
    async fn create_team(&self, ctx: &Context<'_>, name: String) -> Result<super::models::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let res = Team::insert(super::models::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        })
            .exec_with_returning(db)
            .await?;
        return Ok(res);
    }
}
