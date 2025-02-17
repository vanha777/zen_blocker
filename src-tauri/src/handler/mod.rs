use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use std::{error::Error, fmt::format};

use crate::Endpoint;

pub async fn send(endpoint: &Endpoint) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut url = endpoint.endpoint.clone().unwrap();
    // build header
    let mut headers = HeaderMap::new();
    match endpoint.header.clone() {
        Some(x) => {
            for (key, value) in x {
                headers.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_bytes(value.as_bytes()).unwrap(),
                );
            }
        }
        None => (),
    }
    // build body if exsist
    let body = endpoint.body.clone();
    // build query if exsist
    let query = match endpoint.query.clone() {
        Some(x) => {
            let mut query = String::new();
            for (k, v) in endpoint.query.clone().unwrap() {
                let new_query = format!("{k}={v}");
                query = format!("{query}&{new_query}");
            }
            Some(query)
        }
        None => None,
    };
    // add query to url if exsist
    match query {
        Some(x) => url = format!("{url}?{x}"),
        None => (),
    };

    match endpoint.method.clone().unwrap().as_str() {
        "GET" => {
            let response = client
                .get(url)
                .headers(headers)
                .send()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        "POST" => {
            let response = client
                .post(url)
                .headers(headers)
                .json(&body)
                .send()
                .await
                .map_err(|x| "Failed to send response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        "PUT" => {
            let response = client
                .put(url)
                .headers(headers)
                .json(&body)
                .send()
                .await
                .map_err(|x| "Failed to send response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        "DELETE" => {
            let response = client
                .delete(url)
                .headers(headers)
                .send()
                .await
                .map_err(|x| "Failed to send response of the api".to_string())?
                .text()
                .await
                .map_err(|x| "Failed to parse response of the api".to_string())?;
            Ok(response)
        }
        _ => Err("Api Method are not yet supported".to_string()),
    }
}

fn construct_headers(subscription_key: &str, fred_api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert(
        "Ocp-Apim-Subscription-Key",
        HeaderValue::from_str(subscription_key).unwrap(),
    );
    headers.insert("FredApiKey", HeaderValue::from_str(fred_api_key).unwrap());

    headers
}

pub async fn send_dummy_data(
    url: &str,
    subscription_key: &str,
    fred_api_key: &str,
    payload: &str,
) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let headers = construct_headers(subscription_key, fred_api_key);

    let response = client
        .post(url)
        .headers(headers)
        .body(payload.to_string())
        .send()
        .await?
        .text()
        .await?;

    println!("Response data: {}", response);

    Ok(())
}

pub fn get_error_skeleton_base64() -> String {
    String::from(
        r#"
    /9j/4AAQSkZJRgABAQEBLAEsAAD/4QBHRXhpZgAASUkqAAgAAAABAA4BAgAlAAAAGgAAAAAAAABWZWN0b3IgZ3JhcGhpYyBvZiBubyB0aHVtYm5haWwgc3ltYm9s/+EFUWh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8APD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4KPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyI+Cgk8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPgoJCTxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiIHhtbG5zOnBob3Rvc2hvcD0iaHR0cDovL25zLmFkb2JlLmNvbS9waG90b3Nob3AvMS4wLyIgeG1sbnM6SXB0YzR4bXBDb3JlPSJodHRwOi8vaXB0Yy5vcmcvc3RkL0lwdGM0eG1wQ29yZS8xLjAveG1sbnMvIiAgIHhtbG5zOkdldHR5SW1hZ2VzR0lGVD0iaHR0cDovL3htcC5nZXR0eWltYWdlcy5jb20vZ2lmdC8xLjAvIiB4bWxuczpkYz0iaHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnRzLzEuMS8iIHhtbG5zOnBsdXM9Imh0dHA6Ly9ucy51c2VwbHVzLm9yZy9sZGYveG1wLzEuMC8iICB4bWxuczppcHRjRXh0PSJodHRwOi8vaXB0Yy5vcmcvc3RkL0lwdGM0eG1wRXh0LzIwMDgtMDItMjkvIiB4bWxuczp4bXBSaWdodHM9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9yaWdodHMvIiBwaG90b3Nob3A6Q3JlZGl0PSJHZXR0eSBJbWFnZXMvaVN0b2NrcGhvdG8iIEdldHR5SW1hZ2VzR0lGVDpBc3NldElEPSIxMTQ3NTQ0ODA3IiB4bXBSaWdodHM6V2ViU3RhdGVtZW50PSJodHRwczovL3d3dy5pc3RvY2twaG90by5jb20vbGVnYWwvbGljZW5zZS1hZ3JlZW1lbnQ/dXRtX21lZGl1bT1vcmdhbmljJmFtcDt1dG1fc291cmNlPWdvb2ds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AAAAAAAAB//EACUQAAEDBAICAwEBAQAAAAAAAAABAhIDERMyEFAgITAxQGCgQv/aAAgBAQABBQKbibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibibv9QkVUVFT+IsqmNTGhZE4VLp/CQUxkUTzft38VUxkE8bmRDIpNwi3Qqd5FVMamNC1vGaGQmq+NPh+vb2uY1MaEUTxkiGQmvws27aCmMiieN0QyIZFLqvjjUxr8L9uviqmMgnlNDISVfga23Lm38malT66uyqY1MaFkTy9lnGNTGpjUxqY1MamNTGpjUxqIyy+DmXXGpjUxqY1GtjwqXTGpjUxqY1FaqdKlMiidO7Xp8iGRRiqruFqIZDIZDIZDIZDIZDIZDIZDIZDIZDIZDIZDIZDIZDIZDIZBFvw7XpG+2lT64p7Dtf3U/sdr0lPh+vFPYdr+X7EpmNBWqnlT2Ha9Izbwp7DtfytbZOXJZfCnsO16d+xT2Ha/kT78KnjT2Ha9KzUqcU9h2v5fvwqL78Kew7XpafD9SnsO1/K11i9+HPt5U9h2vSt9O4X7p7DtfFjbisQVtvz09h2vTJ7QqbU9h2viiWQ+xzLfmp7Dtemp6lT6p7DtfCmni5njjFaqfPT2Ha9NT+x3ttPYdryntU9J4ubcVLcU05dT+ansO16ZvpeG+njteaafAqXMfi5qKK1U+SnsO16dvtD/sdrwiXX8Dqfx09h2vT0/rl2vFNPX4XNRRUVPhp7Dtenp7cu1Gpdfx/Y5nwU9h2vTp6Xl2pT+/yuZcVLeVPYdr1DfbeHalP89rjmW8aew7XqKfLtSn+lzLipbmnsO16hm3DtRFsZFMhkMhkMhkMhkMhkMhkMhkMhkMhkMhkMhkMhkHOlzT2Ha9WqXRUt+6m3h2vUs16J2vU016N6+uqupdS6l1LqXUupdS6l1LqXUupdS6l1LqXUupdS6l1LqXUupdS6l1LqXUupdS6l1/1Zf/EAB8QAQEAAgEFAQEAAAAAAAAAADEAAVAgEBEhMEBgoP/aAAgBAQAGPwJmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZmZm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
    "#,
    )
}
