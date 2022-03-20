use anyhow;
use teloxide::{
    dispatching2::dialogue::InMemStorage,
    macros::DialogueState,
    prelude2::*,
    types::{ButtonRequest, KeyboardButton, KeyboardMarkup, KeyboardRemove},
    utils::command::BotCommand,
};

use handlebars::Handlebars;

mod formatter;
mod weather;

type BotDialogue = Dialogue<State, InMemStorage<State>>;

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

fn make_weather_keyboard() -> KeyboardMarkup {
    let button = KeyboardButton::new("🗺️").request(ButtonRequest::Location);
    let keyboard: Vec<Vec<KeyboardButton>> = vec![vec![button]];

    KeyboardMarkup::new(keyboard)
}

async fn handle_start(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: BotDialogue,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        match Command::parse(text, "ButlerBot") {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions())
                    .await?;
                dialogue.exit().await?;
            }
            Ok(Command::Start) => {
                bot.send_message(msg.chat.id, Command::descriptions())
                    .await?;
                dialogue.exit().await?;
            }
            Ok(Command::Weather) => {
                let keyboard = make_weather_keyboard();
                bot.send_message(msg.chat.id, "Let's start! Send me your location.")
                    .reply_markup(keyboard)
                    .await?;
                dialogue.update(State::ReceiveLocation).await?;
            }
            _ => {
                bot.send_message(
                    msg.chat.id,
                    format!("Unknown command!\n{}", Command::descriptions()),
                )
                .await?;
                dialogue.exit().await?;
            }
        };
    } else {
        dialogue.exit().await?;
        bot.send_message(
            msg.chat.id,
            format!("Unknown command!\n{}", Command::descriptions()),
        )
        .await?;
    }
    Ok(())
}

async fn handle_receive_location<'a>(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: BotDialogue,
    registry: Handlebars<'a>,
) -> anyhow::Result<()> {
    match msg.location() {
        Some(location) => {
            let weather = weather::get_weather(location.latitude, location.longitude).await?;

            let message = registry.render("weather", &weather)?;

            bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .reply_markup(KeyboardRemove::new())
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
