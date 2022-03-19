use anyhow;
use teloxide::{
    dispatching2::dialogue::InMemStorage, macros::DialogueState, prelude2::*,
    utils::command::BotCommand,
};

use handlebars::Handlebars;

mod formatter;
mod weather;

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show help command.")]
    Start,
    #[command(description = "show weather at specific location.")]
    Weather,
}

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
    if let Some(text) = msg.text() {
        match Command::parse(text, "ButlerBot") {
            Ok(Command::Help) => {
                dialogue.exit().await?;
                bot.send_message(msg.chat.id, Command::descriptions()).await?
            },
            Ok(Command::Start) => {
                dialogue.exit().await?;
                bot.send_message(msg.chat.id, Command::descriptions()).await?
            },
            Ok(Command::Weather) => {
                dialogue.update(State::ReceiveLocation).await?;
                bot.send_message(
                    msg.chat.id,
                    "Let's start! Send me your location.",
                )
                .await?
            },
            _ => {
                dialogue.exit().await?;
                bot.send_message(msg.chat.id, format!("Unknown command!\n{}", Command::descriptions())).await?
            }
        };
    } else {
        dialogue.exit().await?;
        bot.send_message(msg.chat.id, format!("Unknown command!\n{}", Command::descriptions())).await?;
    }
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
