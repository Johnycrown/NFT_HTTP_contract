
use ic_cdk::export::candid::{CandidType, Deserialize, Func, Principal};
use ic_cdk::export::Principal;
use ic_cdk::print;
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(CandidType, Serialize, Deserialize)]
struct WeatherData {
    // Define the structure to hold weather data
    temperature: f32,
    location: String,
    condition: String,
}

#[derive(CandidType, Serialize, Deserialize)]
struct PaymentInfo {
    amount: u64,
    sender: Principal,
}

#[derive(CandidType, Serialize, Deserialize)]
struct NFT {
    // Define the structure for NFT
    id: u64,
    owner: Principal,
    metadata: String,
}

#[derive(CandidType, Serialize, Deserialize)]
enum WeatherCondition {
    Sunny,
    Rainy,
    Snowy,
}

#[derive(CandidType, Serialize, Deserialize)]
struct WeatherNFTContract;

#[ic_cdk_macros::init]
fn init() -> WeatherNFTContract {
    WeatherNFTContract
}

#[ic_cdk_macros::update]
async fn get_weather_data(location: String) -> Option<WeatherData> {
    // Function to get weather data from an external API
    let api_key = "<your_api_key>";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", location, api_key);

    let response = reqwest::get(&url).await.ok()?;
    let weather_data: WeatherData = response.json().await.ok()?;
    Some(weather_data)
}

#[ic_cdk_macros::update]
fn mint_nft(weather_data: WeatherData) -> Option<NFT> {
    // Function to mint NFT based on weather conditions
    let condition = match weather_data.condition.as_str() {
        "Clear" => WeatherCondition::Sunny,
        "Rain" | "Drizzle" | "Thunderstorm" => WeatherCondition::Rainy,
        "Snow" => WeatherCondition::Snowy,
        _ => return None, // Unsupported weather condition
    };

    match condition {
        WeatherCondition::Sunny => {
            // Allow minting NFT for sunny weather
            let nft = NFT {
                id: 1, // Unique ID for the NFT
                owner: ic_cdk::caller(),
                metadata: "Sunny Day NFT".to_string(), // Metadata for the NFT
            };
            Some(nft)
        }
        _ => None, // Disallow minting NFT for other weather conditions
    }
}

#[ic_cdk_macros::update]
fn make_payment(payment_info: PaymentInfo, weather_data: WeatherData) -> bool {
    // Function to conditionally allow payment based on weather data
    if weather_data.temperature > 10.0 && weather_data.condition == "Clear" {
        // Allow payment if temperature > 10°C and weather is clear
        // Implement payment logic here
        ic_cdk::print("Payment successful!");
        true
    } else {
        ic_cdk::print("Payment disallowed due to weather conditions.");
        false
    }
}
