use ipgeolocate::{Locator, Service};
use tokio;

#[tokio::main]
async fn main() {
    let service = Service::IpApi;
    // get ip from command line
    let ip = std::env::args().nth(1);
    match ip {
        Some(ip) => {
            match Locator::get(&ip, service).await {
            Ok(ip) => println!("{} - {} ({})", ip.ip, ip.city, ip.country),
            Err(error) => println!("Error: {}", error),
        };
        }
        None => {
            println!("Usage:\n$ geolocator <ip>");
            return;
        }
    }
}