use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use entity::teams::{self, Entity as Team};
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};

use entity::members::{self, Entity as Member};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn teams<'a>(&self, ctx: &Context<'a>) -> Result<Vec<teams::Model>, DbErr> {
        let database = ctx.data_unchecked::<DatabaseConnection>();
        Team::find().all(database).await
    }
    async fn members<'a>(&self, ctx: &Context<'a>) -> Result<Vec<members::Model>, DbErr> {
        let database = ctx.data_unchecked::<DatabaseConnection>();
        Member::find().all(database).await
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_member(
        &self,
        ctx: &Context<'_>,
        name: String,
        team_id: i32,
    ) -> Result<members::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let res = Member::insert(members::ActiveModel {
            name: ActiveValue::Set(name),
            team_id: ActiveValue::Set(team_id),
            ..Default::default()
        })
        .exec_with_returning(db)
        .await?;
        return Ok(res);
    }

    async fn create_team(&self, ctx: &Context<'_>, name: String) -> Result<teams::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        let res = Team::insert(teams::ActiveModel {
            name: ActiveValue::Set(name),
            ..Default::default()
        })
        .exec_with_returning(db)
        .await?;
        return Ok(res);
    }
}

pub type QuerySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
