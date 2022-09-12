use std::pin::Pin;
use sea_orm::ConnectionTrait;

use async_graphql::{Context, EmptyMutation,EmptySubscription, MergedObject, MergedSubscription,Object, Schema, Subscription};
use entity::teams::{self, Entity as Team, TeamsQuery, TeamMutation};
// use entity::posts::{self, Entity as Post};
use futures_util::Stream;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

use entity::chats::{self, Subscription as ChatSubscription, Mutation as ChatMutation, Query as ChatQuery};

use entity::users::{self, Entity as Member, MembersQuery, MemberMutation};
use tokio::sync::mpsc;

use std::sync::Arc;



#[derive(MergedObject, Default)]
pub struct QueryRoot(TeamsQuery, MembersQuery,ChatQuery);


#[derive(MergedObject, Default)]
pub struct MutationRoot(
    TeamMutation,
    MemberMutation,
    ChatMutation
);


#[derive(MergedSubscription, Default)]
pub struct SubscriptionRoot(
    ChatSubscription
);

pub type QuerySchema = Schema<
    QueryRoot,
    MutationRoot,
    SubscriptionRoot,
>;
