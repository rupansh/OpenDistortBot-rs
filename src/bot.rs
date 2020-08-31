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

// Helpers
use bytes::Bytes;
use crate::config::BotConfig;
use telegram_bot::*;


const FILE_URL: &str = "https://api.telegram.org/file/bot";

pub struct Bot {
    pub api: Api,
    pub config: BotConfig,
}

impl Bot {
    pub fn new(config: BotConfig) -> Bot {
        let api = Api::new(&config.api_token);
        Bot {
            api,
            config
        }
    }

    pub async fn get_photo(&self, path: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
        let photo_url = &format!("{}{}/{}", FILE_URL, &self.config.api_token, path);
        let resp = reqwest::get(photo_url).await?.bytes().await;
    
        return Ok(resp.unwrap_or_default());
    }
}