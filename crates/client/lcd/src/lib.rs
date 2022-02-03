pub mod lcd_client;
pub use lcd_client::{BlockingLCDClient, LCDClient};

#[cfg(test)]
mod tests {
    use crate::{lcd_client::BasicApiRequester, LCDClient};
    use reqwest::Client;
    use tokio;

    #[tokio::test]
    async fn it_works() {
        let requester = BasicApiRequester::new(Client::new());
        let terra = LCDClient::new(requester);
        println!("{:?}", terra.wasm.parameters().await);
    }
}
