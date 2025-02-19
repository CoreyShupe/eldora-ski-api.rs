use eldora_ski_api::*;

#[tokio::main]
async fn main() {
    let client = EldoraSkiApiClient::create();

    display::weather_forecast::print_data(&client).await;
}
