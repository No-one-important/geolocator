use ipgeolocate::{Locator, Service};
use tokio;

// Prints the city where 1.1.1.1 is.
#[tokio::main]
async fn main() {
    let service = Service::IpApi;
    // get ip from command line
    let ip = std::env::args().nth(1).unwrap_or("1.1.1.1".to_string());

    match Locator::get(&ip, service).await {
        Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
        Err(error) => println!("Error: {}", error),
    };
}