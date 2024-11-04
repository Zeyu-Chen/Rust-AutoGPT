use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Weather {
    id: u64,
    description: String,
    temperature: f32,
}

struct AppState {
    weather: Mutex<Vec<Weather>>,
}

#[async_trait]
trait WeatherService {
    async fn get_weather(&self) -> Result<Vec<Weather>, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
struct OpenWeatherMapClient {
    http_client: HttpClient,
    api_key: String,
}

#[async_trait]
impl WeatherService for OpenWeatherMapClient {
    async fn get_weather(&self) -> Result<Vec<Weather>, Box<dyn std::error::Error>> {
        let response = self.http_client.get(&format!("http://api.openweathermap.org/data/2.5/forecast?zip=94040,us&appid={}", self.api_key)).send().await?;
        let weather_data: Vec<Weather> = response.json().await?;
        Ok(weather_data)
    }
}

async fn get_weather(app_state: web::Data<AppState>, weather_service: web::Data<Box<dyn WeatherService>>) -> impl Responder {
    let weather_data = weather_service.get_weather().await;
    match weather_data {
        Ok(data) => {
            let mut weather = app_state.weather.lock().unwrap();
            *weather = data;
            HttpResponse::Ok().json(&*weather)
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let weather_service = OpenWeatherMapClient {
        http_client: HttpClient::new(),
        api_key: "YOUR_API_KEY".to_string(),
    };

    let data: web::Data<AppState> = web::Data::new(AppState { weather: Mutex::new(Vec::new()) });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(data.clone())
            .app_data(web::Data::new(Box::new(weather_service.clone()) as Box<dyn WeatherService>))
            .route("/weather", web::get().to(get_weather))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}