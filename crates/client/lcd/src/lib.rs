pub mod lcd_client;
pub use lcd_client::LCDClient;

#[cfg(test)]
mod tests {
    use crate::lcd_client::LCDClient;
    use tokio;

    #[tokio::test]
    async fn it_works() {
        let terra = LCDClient::new();
        println!("{:?}", terra.wasm.parameters().await);
    }
}
