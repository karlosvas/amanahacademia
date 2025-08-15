mod controllers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use {
    axum::Router,
    reqwest::Client,
    state::{AppState, CustomFirebase},
    std::{env, net::SocketAddr, sync::Arc},
    tokio::net::TcpListener,
    tower_http::{cors::CorsLayer, trace::TraceLayer},
};

#[tokio::main]
async fn main() {
    // Cargar variables de entorno desde un archivo .env
    dotenvy::dotenv().ok();

    // Obtener las claves públicas de Firebase
    let firebase_keys = reqwest::get(
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com",
    )
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

    // Crear la instancia de CustomFirebase, con todos los datos neceesarios para la autenticación
    let firebase: CustomFirebase = CustomFirebase {
        firebase_keys,
        firebase_project_id: env::var("FIREBASE_PROJECT_ID")
            .expect("FIREBASE_PROJECT_ID must be set"),
        firebase_api_key: env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set"),
    };
    // Inicializar el estado de la aplicación y el enrutador
    let state: Arc<AppState> = Arc::new(AppState {
        firebase,
        client: Client::new(),
    });

    // Configurar el enrutador de la aplicación
    let app: Router = Router::new()
        .nest("/user", routes::user::router(state.clone()))
        .layer(CorsLayer::permissive()) // CORS abierto
        .layer(TraceLayer::new_for_http()) // Logging básico
        .with_state(state);

    // Inicializar el listener TCP y el servidor
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor escuchando en http://{}", addr);
    match axum::serve(listener, app).await {
        Ok(_) => println!("Servidor detenido"),
        Err(err) => eprintln!("Error en el servidor: {}", err),
    };
}
