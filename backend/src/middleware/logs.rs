// use axum::http::HeaderMap;

// pub async fn show_data(headers: &HeaderMap) {
//     // Log estructurado
//     println!("=== Payment Request Info ===");

//     // User aggent
//     if let Some(user_agent) = headers.get("user-agent").and_then(|v| v.to_str().ok()) {
//         println!("User-Agent: {}", user_agent);
//     }

//     // Content-Type
//     if let Some(content_type) = headers.get("content-type").and_then(|v| v.to_str().ok()) {
//         println!("Content-Type: {}", content_type);
//     }

//     // IP real
//     if let Some(ip) = headers.get("cf-connecting-ip") {
//         println!("IP real del visitante: {:?}", ip.to_str().unwrap_or(""));
//     }

//     // País
//     if let Some(country) = headers.get("cf-ipcountry") {
//         println!("País: {:?}", country.to_str().unwrap_or(""));
//     }

//     // Ciudad
//     if let Some(city) = headers.get("cf-city") {
//         println!("Ciudad: {:?}", city.to_str().unwrap_or(""));
//     }

//     // Continente
//     if let Some(continent) = headers.get("cf-continent") {
//         println!("Continente: {:?}", continent.to_str().unwrap_or(""));
//     }

//     // Latitud y longitud
//     if let Some(lat) = headers.get("cf-latitude") {
//         println!("Latitud: {:?}", lat.to_str().unwrap_or(""));
//     }

//     // Longitud
//     if let Some(lon) = headers.get("cf-longitude") {
//         println!("Longitud: {:?}", lon.to_str().unwrap_or(""));
//     }
//     println!("========================");
// }
