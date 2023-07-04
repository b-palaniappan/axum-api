use std::error::Error;

use reqwest::{Client, header};
use tracing::info;

use crate::api::model::location::{Items, Position};

pub async fn get_geo_location(address: &String) -> Result<Position, Box<dyn Error>> {
    let here_api_url = "https://geocode.search.hereapi.com";
    let here_api_key = std::env::var("HERE_MAP_API_KEY");

    match here_api_key {
        Ok(api_key) => {
            let location_response = Client::new()
                .get(here_api_url.to_owned() + "/v1/geocode")
                .header(header::CONTENT_TYPE, "application/json")
                .query(&[("q", address), ("apiKey", &api_key.to_owned()), ("in", &"countryCode:USA".to_owned())])
                .send()
                .await?;

            info!("location response {:?}", &location_response);

            if location_response.status().is_success() {
                let location_res: Items = location_response.json().await?;
                if !location_res.items.is_empty() {
                    let location_one = location_res.items.get(0);
                    return match location_one {
                        Some(item) => {
                            let location_geo_code: Position = Position {
                                lat: item.position.lat,
                                lng: item.position.lng,
                            };
                            Ok(location_geo_code)
                        }
                        None => {
                            Err(Box::try_from(anyhow::Error::msg("Location Error")).unwrap())
                        }
                    }
                }
            }
        }
        Err(_) => {
            return Err(Box::try_from(anyhow::Error::msg("Unable to get api key")).unwrap());
        }
    }

    return Err(Box::try_from(anyhow::Error::msg("Location Error")).unwrap());
}
