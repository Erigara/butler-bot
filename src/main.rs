use anyhow;
use teloxide::{
    dispatching2::dialogue::InMemStorage,
    macros::DialogueState,
    prelude2::*,
    types::{ButtonRequest, KeyboardButton, KeyboardMarkup},
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
    #[command(description = "show weather at specific location.")]
    Weather,
}

#[derive(DialogueState, Clone)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(handle_main_menu)]
    MainMenu,

    #[handler(handle_send_weather_forecast)]
    SendWeatherForecast,
}

impl Default for State {
    fn default() -> Self {
        Self::MainMenu
    }
}

fn make_main_menu_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::default()
        .append_row(vec![KeyboardButton::new("/help")])
        .append_row(vec![KeyboardButton::new("/weather")])
        .resize_keyboard(true)
}

fn make_weather_keyboard() -> KeyboardMarkup {
    let button = KeyboardButton::new("📍").request(ButtonRequest::Location);
    KeyboardMarkup::default()
        .append_row(vec![button])
        .resize_keyboard(true)
}

fn filter_start_command(msg: Message) -> bool {
    match msg.text() {
        Some("/start") => true,
        _ => false,
    }
}

async fn handle_start(bot: AutoSend<Bot>, msg: Message) -> anyhow::Result<()> {
    if let Some(_) = msg.text() {
        let keyboard = make_main_menu_keyboard();
        bot.send_message(msg.chat.id, Command::descriptions())
            .reply_markup(keyboard)
            .await?;
    }
    Ok(())
}

async fn handle_main_menu(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: BotDialogue,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        match Command::parse(text, "ButlerBot") {
            Ok(Command::Help) => {
                let keyboard = make_main_menu_keyboard();
                bot.send_message(msg.chat.id, Command::descriptions())
                    .reply_markup(keyboard)
                    .await?;
                dialogue.exit().await?;
            }
            Ok(Command::Weather) => {
                let keyboard = make_weather_keyboard();
                bot.send_message(msg.chat.id, "Let's start! Send me your location.")
                    .reply_markup(keyboard)
                    .await?;
                dialogue.update(State::SendWeatherForecast).await?;
            }
            _ => {
                let keyboard = make_main_menu_keyboard();
                bot.send_message(
                    msg.chat.id,
                    format!("Unknown command!\n{}", Command::descriptions()),
                )
                .reply_markup(keyboard)
                .await?;
                dialogue.exit().await?;
            }
        };
    } else {
        let keyboard = make_main_menu_keyboard();
        bot.send_message(
            msg.chat.id,
            format!("Unknown command!\n{}", Command::descriptions()),
        )
        .reply_markup(keyboard)
        .await?;
        dialogue.exit().await?;
    }
    Ok(())
}

async fn handle_send_weather_forecast<'a>(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: BotDialogue,
    registry: Handlebars<'a>,
) -> anyhow::Result<()> {
    let forecast = match msg.location() {
        Some(location) => {
            weather::get_weather_forecast_by_location_coords(location.latitude, location.longitude)
                .await?
        }
        None => match msg.text() {
            Some(name) => weather::get_weather_forecast_by_location_name(name).await?,
            None => None,
        },
    };

    match forecast {
        Some(forecast) => {
            let keyboard = make_main_menu_keyboard();
            let message = registry.render("weather", &forecast)?;
            bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .reply_markup(keyboard)
                .await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Can't find location, please, try again.")
                .await?;
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    teloxide::enable_logging!();
    log::info!("Starting ButlerBot...");

    let bot = Bot::from_env().auto_send();
    let registry = formatter::create_registry()?;

    let handler = Update::filter_message()
        .branch(dptree::filter(filter_start_command).endpoint(handle_start))
        .branch(
            dptree::entry()
                .enter_dialogue::<Message, InMemStorage<State>, State>()
                .dispatch_by::<State>(),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new(), registry])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;

    Ok(())
}
