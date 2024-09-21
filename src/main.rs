use anyhow::Result;
use matrix_sdk::ruma::UserId;
use matrix_sdk::{
    config::SyncSettings, ruma::events::room::message::RoomMessageEventContent, Client,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let homeserver_url = "https://matrix.org";
    let username = "@your-username:matrix.org";
    let password = "your-password";

    let client = Client::builder()
        .homeserver_url(homeserver_url)
        .build()
        .await?;

    client
        .login_username(UserId::parse(username)?, password)
        .send()
        .await?;

    client.add_event_handler(
        |event: RoomMessageEventContent, room_id, client| async move {
            if let Some(msg_body) = event.body() {
                if msg_body == "!hello" {
                    let content = RoomMessageEventContent::text_plain("Hello!");
                    client.room_send(&room_id, content, None).await.unwrap();
                }
            }
        },
    );

    // 开始同步
    client.sync(SyncSettings::default()).await?;

    Ok(())
}
