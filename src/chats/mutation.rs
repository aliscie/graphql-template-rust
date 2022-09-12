use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};
use std::pin::Pin;
use sea_orm::ConnectionTrait;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, MergedObject,Subscription};
use crate::chats::models::{self, Entity as ChatRoom};

use crate::chats::{Message, Shared};

use futures_util::Stream;

use tokio::sync::mpsc;
use std::sync::Arc;

#[derive(Default)]
pub struct Mutation;


#[Object]
impl Mutation {
    async fn create_chat_room(&self, ctx: &Context<'_>) -> Result<crate::chats::models::Model, DbErr> {
        let db = ctx.data_unchecked::<DatabaseConnection>();
        ChatRoom::insert(crate::chats::models::ActiveModel {
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
                for i in y {
                    let _ = i.send(message.clone()).await;
                }
            }
        }
        Ok(message)
    }
}
