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
    resend_rs::Resend,
    state::{AppState, CustomFirebase},
    std::{env, net::SocketAddr, sync::Arc},
    stripe::Client as StripeClient,
    tokio::net::TcpListener,
    tower_http::{
        cors::{AllowOrigin, CorsLayer},
        trace::TraceLayer,
    },
    tracing::{debug, error, info},
    tracing_subscriber::{
        EnvFilter, fmt,
        layer::{Layer, SubscriberExt},
        util::SubscriberInitExt,
    },
};

#[tokio::main]
async fn main() {
    // Cargar variables de entorno desde un archivo .env
    dotenvy::dotenv().ok();

    // Usar cfg!(debug_assertions) directamente para el logging
    let env_filter: EnvFilter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            EnvFilter::new("debug") // Desarrollo
        } else {
            EnvFilter::new("info") // Producción
        }
    });
    // Inicializar tracing correctamente
    tracing_subscriber::registry()
        .with(env_filter)
        .with({
            let layer = fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .with_ansi(cfg!(debug_assertions)); // Colores en desarrollo
            if cfg!(debug_assertions) {
                layer.compact().boxed() // ← Desarrollo: formato compacto con colores
            } else {
                layer.json().boxed() // ← Producción: JSON estructurado sin colores
            }
        })
        .init();

    if cfg!(debug_assertions) {
        info!("🔧 Modo desarrollo - debug_assertions activado");
    } else {
        info!("🚀 Modo producción - debug_assertions desactivado");
    }

    // Obtener las claves públicas de Firebase
    info!("Fetching Firebase public keys");
    let firebase_keys = match reqwest::get(
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com",
    )
    .await
    {
        Ok(response) => {
            debug!("Firebase keys response status: {}", response.status());
            match response.json().await {
                Ok(keys) => {
                    info!("Firebase public keys fetched successfully");
                    debug!(
                        "Available key IDs: {:?}",
                        if let serde_json::Value::Object(obj) = &keys {
                            obj.keys().collect::<Vec<_>>()
                        } else {
                            vec![]
                        }
                    );
                    keys
                }
                Err(e) => {
                    error!("Failed to parse Firebase keys JSON: {}", e);
                    panic!("Cannot start without Firebase keys");
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch Firebase keys: {}", e);
            panic!("Cannot start without Firebase keys");
        }
    };

    // Crear la instancia de CustomFirebase, con todos los datos neceesarios para la autenticación
    let firebase: CustomFirebase = CustomFirebase {
        firebase_keys,
        firebase_project_id: env::var("FIREBASE_PROJECT_ID")
            .expect("FIREBASE_PROJECT_ID must be set"),
        firebase_api_key: env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set"),
        firebase_database_url: env::var("FIREBASE_DATABASE_URL")
            .expect("FIREBASE_DATABASE_URL must be set"),
    };

    // Cliente de stripe
    let stripe_client: StripeClient =
        StripeClient::new(env::var("STRIPE_API_KEY").expect("STRIPE_API_KEY must be set"));

    // Cliente de resend
    let resend_client: Resend = Resend::new(
        env::var("RESEND_API_KEY")
            .expect("RESEND_API_KEY must be set")
            .as_str(),
    );

    // Inicializar el estado de la aplicación y el enrutador
    let state: Arc<AppState> = Arc::new(AppState {
        firebase,
        firebase_client: HttpClient::new(),
        stripe_client,
        resend_client,
    });

    // Configurar CORS
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "http://localhost:4321".parse().unwrap(), // Frontend desarrollo
            "https://amanahacademia.com".parse().unwrap(), // Dominio de producción
        ])) // Origenes Permitidos
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Métodos permitidos
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]); // Encabezados permitidos

    // Configurar el enrutador de la aplicación
    let app: Router = Router::new()
        .nest("/users", routes::users::router(state.clone())) // FB Auth, FB Realtime DB
        .nest("/comments", routes::comments::router(state.clone())) // FB Auth, FB Realtime DB
        .nest("/payment", routes::payments::router(state.clone())) // Stripe
        .nest("/teachers", routes::teachers::router(state.clone())) // FB Auth, FB Realtime DB
        .nest("/email", routes::email::router(state.clone())) // Email
        .nest("/webhook", routes::webhooks::router(state.clone())) // Webhooks
        .layer(cors) // CORS abierto
        .layer(TraceLayer::new_for_http()) // Logging básico
        .with_state(state);

    // Inicializar el listener TCP y el servidor
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server listening on http://{}", addr);
    match axum::serve(listener, app).await {
        Ok(_) => println!("Server finalized"),
        Err(err) => eprintln!("Error in server: {}", err),
    };
}
