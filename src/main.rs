use listener::{listen_pump_new_coins, listen_pump_new_coins_discord};

pub mod pump;
pub mod listener;
pub mod provider;
pub mod jito;
pub mod constants;
pub mod raydium;

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    env_logger::init();
    
    if let Err(e) = listen_pump_new_coins_discord().await {
        eprintln!("Error listening for PumpFun events: {}", e);
    }
}

