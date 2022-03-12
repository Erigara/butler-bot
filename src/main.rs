use anyhow;
use teloxide::{dispatching2::dialogue::InMemStorage, macros::DialogueState, prelude2::*};

use handlebars::Handlebars;

mod weather;
mod formatter;

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(DialogueState, Clone)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(handle_start)]
    Start,

    #[handler(handle_receive_location)]
    ReceiveLocation,
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    teloxide::enable_logging!();
    log::info!("Starting ButlerBot...");

    let bot = Bot::from_env().auto_send();
    let registry = formatter::new()?;

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .dispatch_by::<State>(),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new(), registry])
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;

    Ok(())
}

async fn handle_start(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> anyhow::Result<()> {
    bot.send_message(msg.chat.id, "Let's start! Send me your location.")
        .await?;
    dialogue.update(State::ReceiveLocation).await?;
    Ok(())
}

async fn handle_receive_location<'a>(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    registry: Handlebars<'a>,
) -> anyhow::Result<()> {
    match msg.location() {
        Some(location) => {
            let weather = weather::get_weather(location.latitude, location.longitude).await?;
            
            let message = registry.render("weather", &weather)?;

            bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Can't get location, please, try again.")
                .await?;
        }
    }

    Ok(())
}