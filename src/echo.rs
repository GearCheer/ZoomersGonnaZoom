use serenity::{prelude::{GatewayIntents, EventHandler, Context}, Client, async_trait, model::prelude::Ready};
use serenity::model::channel::Message;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handeler for hte `message` event. Whenever a new message
    // is received - the closure passed in will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message){
        if msg.content == "!profile" {

            // Send a message back when the above content is received.
            // There can be errors with sending a message, so log to 
            // stderr if it fails.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hyundai").await {
                eprintln!("Failed to send message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a 
    // shard is booted and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids (server IDs), current
    // user data, private channels, and more.
    async fn ready(&self, _: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
    }

}

/// # connection_test
/// Try to get a message from a guild and echo the message
/// in the same channel its received
pub async fn connection_test(token: &str) {

    // Get the types of interations we want the bot to pick up. The bot
    // will be 
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents).event_handler(Handler).await.expect("Couldn't create client");

    // Start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential vackoff untill it reconnects
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    

}