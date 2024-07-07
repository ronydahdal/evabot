use std::collections::HashMap;
use std::env;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::id::ChannelId;
use serenity::model::gateway::GatewayIntents;
use serenity::builder::{CreateEmbed, CreateMessage};
use tokio::time::{interval, Duration};
use dotenv::dotenv;
use chrono::{Datelike, Local, Timelike, Weekday};
use serde::Deserialize;
use serde_json::from_str;

// Structure for each evangelion character's data 
// generate Deserialize for handling JSON, Clone for duplicating JSON data
#[derive(Deserialize, Clone)]
struct Character {
    gif: String,
    quotes: Vec<String>,
}

// event handler for the bot, we'll feed it JSON data and the intended Channel ID to message in
struct Handler {
    json: HashMap<String, Character>,
    channel_id: u64,
}

// implement EventHandler for events -- what will the bot do?
// define as async trait to make function asynchronous 
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("bot is connected");

        // define json and channel_id from attributes defined in main
        let json = self.json.clone();
        let channel_id = self.channel_id;
        
        // run always and send an embedded message of today's character at 12pm
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60 * 60)); // check every hour
            loop {
                interval.tick().await;
                let now = Local::now();
                let hour = now.hour();
                let day = now.weekday();
                let character = select_character(day);

                if hour == 12 {
                    if let Some((gif, quote)) = gif_and_quote(&json, &character) {
                        let channel_id = ChannelId::new(channel_id);
                        let embed = CreateEmbed::new()
                            .description(quote)
                            .image(gif);
                        let builder = CreateMessage::new().embed(embed);

                        if let Err(why) = channel_id.send_message(&ctx.http, builder).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }
            }
        });
    }
}

// for the bot to run, five steps are done in main:
//  1) import .env credentials for discord server's channel ID and bot token 
//  2) parse JSON data
//  3) define bot's 'intentions', in which in our case is to simply message
//  4) authorize the bot's client with a token
//  5) feed the client's event_handler the JSON data and channel ID

// set up tokio (async) runtime 
#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let channel_id: u64 = env::var("CHANNEL_ID")
    .expect("Expected Channel ID")
    .parse()
    .expect("Channel ID must be u64");

    let json = parse_json().expect("Failed to parse JSON");

    let handler = Handler {
        json,
        channel_id,
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// given a character, return the link to the character's gif and a random chosen qoute from their JSON entry
fn gif_and_quote(json: &HashMap<String, Character>, character: &str) -> Option<(String, String)> {
    if let Some(character_entry) = json.get(character) {
        if let Some(quote) = character_entry.quotes.choose(&mut thread_rng()) {
            let random_quote = quote.to_string();
            let gif_link = character_entry.gif.clone();
            return Some((gif_link, random_quote));
        }
    }
    None
}

// from gifs.json --> HashMap with character names as keys and their data as values
fn parse_json() -> Result<HashMap<String, Character>, serde_json::Error> {
    let json = std::fs::read_to_string("src/gifs.json").unwrap();
    let characters: Result<HashMap<String, Character>, _> = from_str(&json);

    // return characters
    characters

    // match characters {
    //     Ok(chars) => println!("{:#?}", chars),
    //     Err(e) => println!("Failed to parse JSON: {}", e),
    // }
}

// match the day with the character -- with a 5% it's Pen Pen day
fn select_character(day: Weekday) -> String {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.05) { // 5% chance to select Pen Pen
        return "penpen".to_string();
    }

    match day {
        Weekday::Mon => "misato".to_string(),
        Weekday::Tue => "rei".to_string(),
        Weekday::Wed => "gendo".to_string(),
        Weekday::Thu => "asuka".to_string(),
        Weekday::Fri => "kaji".to_string(),
        Weekday::Sat => "kaworu".to_string(),
        Weekday::Sun => "shinji".to_string(),
    }
}
