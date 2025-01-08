use std::time::Duration;

use ractor::concurrency::sleep;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use tracing::info;

struct Ping;
enum PingMsg {
    Ping { from: ActorRef<PingMsg> },
    Pong { from: ActorRef<PingMsg> },
}

impl Actor for Ping {
    type Msg = PingMsg;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        _: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            PingMsg::Ping { from } => {
                info!(target: "club", from = %from.get_id(), myself = %myself.get_id(), "got ping");
                // from.send_after(Duration::from_secs(1), || PingMsg::Pong { from: myself })
                //     .await??;
                sleep(Duration::from_secs(1)).await;
                ractor::cast!(from, PingMsg::Pong { from: myself })?;
            }
            PingMsg::Pong { from } => {
                info!(target: "club", from = %from.get_id(), myself = %myself.get_id(), "got pong");
                // It seems to work fine with send_after
                // from.send_after(Duration::from_secs(1), || PingMsg::Ping { from: myself })
                //     .await??;
                sleep(Duration::from_secs(1)).await;
                ractor::cast!(from, PingMsg::Ping { from: myself })?;
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), ActorProcessingErr> {
    tracing_subscriber::fmt::init();

    let (ping, _) = Actor::spawn(None, Ping, ()).await?;
    let (pong, handle) = Actor::spawn(None, Ping, ()).await?;

    ractor::cast!(pong, PingMsg::Ping { from: ping })?;

    handle.await?;
    Ok(())
}
