
use std::sync::Arc;

use juniper::{graphql_object, RootNode, EmptyMutation};
use juniper::{EmptySubscription, FieldResult};

use crate::db::DbPool;
use crate::models::team::Team;


#[derive(Clone)]
pub struct Context {
    pub db_pool: Arc<DbPool>,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    #[graphql(description = "Get all teams")]
    fn get_team(context : &Context) -> FieldResult<Vec<Team>>{
        let connection = &mut context.db_pool.get()?;
        let results = Team::get(connection)?;
        Ok(results)
    }
}


pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {},  EmptyMutation::new(), EmptySubscription::new())
}

pub fn create_context(db_pool: Arc<DbPool>) -> Context {
    Context { db_pool }
}


