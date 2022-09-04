
use juniper::{graphql_object, RootNode, EmptyMutation};
use juniper::{EmptySubscription, FieldResult};

use sea_orm::{DatabaseConnection, EntityTrait};
use entity::teams::{Entity as Team, self};


#[derive(Clone)]
pub struct Context {
    pub pool: DatabaseConnection,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Get all teams")]
    async fn get_team(context : &Context) -> FieldResult<Vec<teams::Model>>{
        let connection = &context.pool;
        let results = Team::find().all(connection).await?;
        Ok(results)
    }
}


pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {},  EmptyMutation::new(), EmptySubscription::new())
}

pub fn create_context(pool : DatabaseConnection) -> Context {
    Context { pool }
}


