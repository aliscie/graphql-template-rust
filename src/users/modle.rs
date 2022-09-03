extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;

use crate::db::PgPool;
use crate::schema::members;

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;



#[derive(Queryable)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub knockouts: i32,
    pub team_id: i32,
}

#[juniper::object(description = "A member of a team")]
impl Member {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn knockouts(&self) -> i32 {
        self.knockouts
    }

    pub fn team_id(&self) -> i32 {
        self.team_id
    }
}

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn members(context: &Context) -> Vec<Member> {
        use crate::schema::members::dsl::*;
        let connection = context.db.get().unwrap();
        ;
        members
            .limit(100)
            .load::<Member>(&connection)
            .expect("Error loading members")
    }
}
