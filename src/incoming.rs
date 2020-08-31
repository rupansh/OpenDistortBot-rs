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

use crate::bot::Bot;
use image::{GenericImageView, ImageOutputFormat::Jpeg, imageops::FilterType::Nearest};
use telegram_bot::prelude::*;
use telegram_bot::{Error, Message, MessageKind, MessageOrChannelPost, InputFileUpload};


async fn cmd_distort(bot: &Bot, message: Message) -> Result<(), Error> {
    if let Some( MessageOrChannelPost::Message(reply)) = message.reply_to_message.as_deref() {
        match reply.kind {
            MessageKind::Photo { ref data, .. } => {
                let photo = data.last().unwrap();
                if photo.width > bot.config.maxw || photo.height > bot.config.maxh {
                    bot.api.send(message.text_reply("Your Image is too big!")).await?;
                    return Ok(());
                }

                let p_path = bot.api.send(photo.get_file()).await?.file_path.unwrap_or_default();
                if let Ok(photo) = bot.get_photo(&p_path).await {
                    let img = match image::load_from_memory(&photo) {
                        Ok(i) => i,
                        Err(_) => {
                            bot.api.send(message.text_reply("Invalid Image!")).await?;
                            return Ok(());
                        }
                    };

                    let (w, h) = img.dimensions();
                    let resized = seam_carving::easy_resize(&img, (0.6*w as f32) as usize, (0.6*h as f32) as usize);
                    let rdy = resized.resize(w, h, Nearest);

                    let mut buf: Vec<u8> = Vec::new();
                    rdy.write_to(&mut buf, Jpeg(100)).unwrap();

                    let mphoto = InputFileUpload::with_data(buf, "photo.jpg");
                    bot.api.send(message.photo_reply(mphoto)).await?;
                } else {
                    bot.api.send(message.text_reply("Distort Failed!")).await?;
                }
            },
            _ => ()
        }
    }

    Ok(())
}

pub async fn command_parser(bot: &Bot, message: Message) -> Result<(), Error> {
    match message.kind {
        MessageKind::Text { ref data, .. } => match data.as_str() {
            "/distort" => cmd_distort(bot, message).await?,
            _ => ()
        },
        _ => ()
    }

    Ok(())
}
