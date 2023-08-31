use std::env;

pub mod echo;

#[tokio::main]
async fn main() {

    // Get the token from the environment variable
    let token = env::var("DISCORD_TOKEN").expect("A token wasn't found in the environment.");

    echo::connection_test(&token).await;

}
