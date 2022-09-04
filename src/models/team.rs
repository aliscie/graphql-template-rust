
use diesel::prelude::*;

#[derive(Queryable, GraphQLObject, Clone, Serialize)]
#[graphql(description = "Team")]
pub struct Team{
    pub id : i32,
    pub name : String,
}

impl Team{
    pub fn get(connection : &mut PgConnection) -> QueryResult<Vec<Team>>{
        use crate::schema::teams::dsl::*;
        teams.limit(100).load::<Team>(connection)
    }
}
