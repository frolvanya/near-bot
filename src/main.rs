use dotenv::dotenv;
use std::{env, error::Error};

use binance::api::*;
use binance::market::*;

use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "This command is supported:")]
enum Command {
    #[command(description = "help command")]
    Help,
    #[command(description = "handle a username.")]
    Price,
}

async fn responses_to_command(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let market: Market = Binance::new(None, None);

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Price => match market.get_price("NEARUSDT") {
            Ok(symbol_price) => {
                println!("{:#?}", &symbol_price);

                let current_price = &symbol_price.price;
                cx.answer(format!(
                    "NEAR Price: {:.02}$\nCurrent Account: {:.02}$ ",
                    current_price,
                    256. * current_price
                ))
                .await?
            }
            Err(e) => {
                eprint!("{:#?}", e);

                cx.answer(format!(
                    "Something went wrong. Did you use the correct cryptocurrency pair?"
                ))
                .await?
            }
        },
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting NearBinanceBot...");

    let bot = Bot::from_env().auto_send();
    let bot_name: String = "NearBinanceBot".into();

    teloxide::commands_repl(bot, bot_name, responses_to_command).await;
}
