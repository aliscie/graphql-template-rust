use async_graphql::{Context, EmptyMutation, EmptySubscription, MergedObject, Object, Schema, Subscription};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};
use crate::users::{self, Entity as Member};

#[derive(Default)]
pub struct Mutation;


#[Object]
impl Mutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        name: String,
        team_id: i32,
    ) -> Result<crate::users::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let res = Member::insert(crate::users::ActiveModel {
            name: ActiveValue::Set(name),
            team_id: ActiveValue::Set(team_id),
            ..Default::default()
        })
            .exec_with_returning(db)
            .await?;
        return Ok(res);
    }
}
