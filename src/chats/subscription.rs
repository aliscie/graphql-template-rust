use async_graphql::{Context, EmptyMutation, EmptySubscription,MergedObject, Object, Schema, Subscription};
use crate::users::{self, Entity as Member};
use crate::chats::models::{self, Entity as ChatRoom};
use std::pin::Pin;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait, Statement};
use std::sync::{Arc};
use tokio::sync::mpsc;
use futures_util::Stream;
use crate::chats::{Message, Shared};

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn recieve_messages(
        &self,
        ctx: &Context<'_>,
        chat_room_id: i32,
    ) -> Pin<Box<dyn Stream<Item=Message> + Send + Sync + 'static>> {
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

