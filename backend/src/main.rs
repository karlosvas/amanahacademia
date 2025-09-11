mod controllers;
mod middleware;
mod models;
mod routes;
mod services;
mod state;

use {
    axum::{
        Router,
        http::{
            Method,
            header::{AUTHORIZATION, CONTENT_TYPE},
        },
    },
    reqwest::Client as HttpClient,
    state::{AppState, CustomFirebase},
    std::{env, net::SocketAddr, sync::Arc},
    // stripe::Client as StripeClient,
    tokio::net::TcpListener,
    tower_http::{
        cors::{AllowOrigin, CorsLayer},
        trace::TraceLayer,
    },
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

    // Cliente de stripe
    // let stripe_client: StripeClient =
    //     StripeClient::new(env::var("STRIPE_API_KEY").expect("STRIPE_API_KEY must be set"));

    // Inicializar el estado de la aplicación y el enrutador
    let state: Arc<AppState> = Arc::new(AppState {
        firebase,
        firebase_client: HttpClient::new(),
        // stripe_client,
    });

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "http://localhost:3000".parse().unwrap(), // Redirecciones internas
            "http://localhost:4321".parse().unwrap(), // Frontend desarrollo
            "https://amanahacademia.com".parse().unwrap(), // Dominio de producción
        ])) // Origenes Permitidos
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Métodos permitidos
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]) // Encabezados permitidos
        .allow_credentials(true); // Permitir cookies y credenciales

    // Configurar el enrutador de la aplicación
    let app: Router = Router::new()
        .nest("/users", routes::users::router(state.clone())) // FB Auth, FB Realtime DB
        .nest("/comments", routes::comments::router(state.clone())) // FB Auth, FB Realtime DB
        // .nest("/payment", routes::payments::router(state.clone())) // Stripe
        .nest("/teachers", routes::teachers::router(state.clone())) // FB Auth, FB Realtime DB
        .nest("/webhook", routes::webhooks::router(state.clone())) // Webhooks
        .layer(cors) // CORS abierto
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
