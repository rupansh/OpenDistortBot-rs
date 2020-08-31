/*
   Copyright 2020 Rupansh Sekar
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
     http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

mod bot;
mod config;
mod incoming;

use bot::Bot;
use config::{CONF_PATH, BotConfig};
use futures::StreamExt;
use incoming::command_parser;
use telegram_bot::*;



#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: BotConfig = confy::load_path(CONF_PATH).unwrap();
    let bot = Bot::new(config);

    // Fetch new updates via long poll method
    let mut stream = bot.api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            let res = command_parser(&bot, message).await;
            if res.is_err() {
                println!("Bot Error: {:#?}", res.err())
            }
        }
    }

    Ok(())
}
