use crate::models::Planet;
use tokio::runtime::Runtime;

pub async fn fetch_planets_from_api() -> Result<Vec<Planet>, reqwest::Error> {
    // let url = "http://127.0.0.1:8000/api/planets";
    let response = reqwest::get("http://127.0.0.1:8000/api/planets")
        .await?
        .json::<Vec<Planet>>()
        .await?;
    Ok(response)
}

pub fn get_planets() -> Vec<Planet> {
    let runtime = Runtime::new().unwrap();

    let planets: Vec<Planet> = runtime
        .block_on(fetch_planets_from_api())
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            vec![]
        });
    // if planets.is_empty() {
    //     println!("No planets found");
    // } else {
    //     for planet in &planets {
    //         println!("{:?}", planet);
    //     }
    // }
    planets
}
