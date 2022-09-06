use std::pin::Pin;
use sea_orm::ConnectionTrait;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, Subscription};
use entity::teams::{self, Entity as Team};
use futures_util::Stream;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};

use entity::chat_room::{self, Entity as ChatRoom};
use entity::members::{self, Entity as Member};
use tokio::sync::mpsc;

use std::sync::Arc;

use crate::{Message, Shared};

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

    async fn chatrooms<'a>(&self, ctx: &Context<'a>) -> Result<Vec<chat_room::Model>, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        ChatRoom::find().all(db).await
    }

    async fn chatroom<'a>(
        &self,
        ctx: &Context<'a>,
        id: i32,
    ) -> Result<Option<chat_room::Model>, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        ChatRoom::find_by_id(id).one(db).await
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

    async fn create_chat_room(&self, ctx: &Context<'_>) -> Result<chat_room::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        ChatRoom::insert(chat_room::ActiveModel {
            messages: ActiveValue::Set(serde_json::json!([])),
            ..Default::default()
        })
        .exec_with_returning(db)
        .await
    }

    async fn send_message(
        &self,
        ctx: &Context<'_>,
        message: String,
        chat_room_id: i32,
        sender_id: i32,
    ) -> Result<Message, DbErr> {
        let message = Message {
            created_at: chrono::Utc::now().naive_utc().to_string(),
            message,
            sender_id,
        };
        let m = serde_json::to_string(&message).unwrap();
        let db = ctx.data_unchecked::<DatabaseConnection>();
        db.execute(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            format!(
                "update chat_rooms set messages = messages || jsonb_build_array('{}'::jsonb) where id = {}",
                m, chat_room_id
            ),
        ))
        .await?;
        let shared = ctx.data_unchecked::<Arc<Shared>>();
        {
            if let Some(y) = shared.senders.write().await.get_mut(&chat_room_id) {
                for i in y{
                    let _ = i.send(message.clone()).await;
                }
            }
        }
        Ok(message)
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn recieve_messages(
        &self,
        ctx: &Context<'_>,
        chat_room_id: i32,
    ) -> Pin<Box<dyn Stream<Item = Message> + Send + Sync + 'static>> {
        let shared = ctx.data_unchecked::<Arc<Shared>>();
        let (stream_tx, stream_rx) = mpsc::channel(1);
        let (tx, mut rx) = mpsc::channel(1);
        {
            shared
                .senders
                .write()
                .await
                .entry(chat_room_id.clone())
                .or_default()
                .push(tx);
        }
        let shared = shared.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                match stream_tx.send(msg).await {
                    Ok(_) => {}
                    Err(_) => {
                        // If sending failed, then remove the user from shared data
                        println!("[Remote] stream tx sending error. Remote {}", &chat_room_id);
                        shared.senders.write().await.remove(&chat_room_id);
                    }
                }
            }
        });
        Box::pin(tokio_stream::wrappers::ReceiverStream::new(stream_rx))
    }
}

pub type QuerySchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
