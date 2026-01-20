use {
    amanahacademia::{
        controllers,
        models::{
            metrics::ServiceAccount,
            state::{AppState, CalOptions, CustomFirebase, GAOptions, KeyCache, MailchimpOptions},
        },
        routes,
    },
    axum::{
        Router,
        http::{
            Method,
            header::{AUTHORIZATION, CONTENT_TYPE},
        },
    },
    reqwest::Client as HttpClient,
    resend_rs::Resend,
    serde_json::Value,
    std::{collections::HashMap, env, net::SocketAddr, sync::Arc, time::Instant},
    stripe::Client as StripeClient,
    tokio::{net::TcpListener, sync::RwLock},
    tower_http::{
        cors::{AllowOrigin, CorsLayer},
        trace::TraceLayer,
    },
    tracing::{error, info},
    tracing_subscriber::{
        EnvFilter, fmt,
        layer::{Layer, SubscriberExt},
        util::SubscriberInitExt,
    },
};

#[tokio::main]
async fn main() {
    // Cargar variables de entorno
    dotenvy::dotenv().ok();

    // Nivel de logging: debug en dev (mensajes verbosos), info en prod (solo eventos significativos)
    let env_filter: EnvFilter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            EnvFilter::new("debug") // Desarrollo
        } else {
            EnvFilter::new("info") // Producción
        }
    });

    // Inicializar tracing
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

    // Las keys públicas de Firebase son necesarias para validar JWTs en cada request
    let initial_keys = fetch_firebase_keys()
        .await
        .expect("Failed to fetch initial Firebase keys");
    let firebase_keys = Arc::new(RwLock::new(KeyCache {
        keys: initial_keys,
        fetched_at: Instant::now(),
    }));

    async fn fetch_firebase_keys() -> Result<Value, Box<dyn std::error::Error>> {
        let response = reqwest::get(
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com",
    )
    .await?;

        if !response.status().is_success() {
            return Err("Firebase keys endpoint returned error".into());
        }

        let keys: Value = response.json().await?;

        if !keys.is_object() || keys.as_object().map_or(true, |m| m.is_empty()) {
            return Err("Firebase public keys are empty".into());
        }

        info!("Firebase public keys fetched successfully");
        Ok(keys)
    }

    let firebase_options: CustomFirebase = CustomFirebase {
        firebase_keys,
        firebase_project_id: env::var("FIREBASE_PROJECT_ID")
            .expect("FIREBASE_PROJECT_ID must be set"),
        firebase_api_key: env::var("FIREBASE_API_KEY").expect("FIREBASE_API_KEY must be set"),
        firebase_database_url: env::var("FIREBASE_DATABASE_URL")
            .expect("FIREBASE_DATABASE_URL must be set"),
        firebase_database_secret: env::var("FIREBASE_DATABASE_SECRET")
            .expect("FIREBASE_DATABASE_SECRET must be set"),
        firebase_client: HttpClient::new(),
    };

    let stripe_client: StripeClient =
        StripeClient::new(env::var("STRIPE_API_KEY").expect("STRIPE_API_KEY must be set"));

    let resend_client: Resend = Resend::new(
        env::var("RESEND_API_KEY")
            .expect("RESEND_API_KEY must be set")
            .as_str(),
    );

    let mailchimp_client: MailchimpOptions = MailchimpOptions::new(
        env::var("MAILCHIMP_API_KEY").expect("MAILCHIMP_API_KEY must be set"),
        env::var("MAILCHIMP_SERVER_PREFIX").expect("MAILCHIMP_SERVER_PREFIX must be set"),
        env::var("MAILCHIMP_LIST_ID").expect("MAILCHIMP_LIST_ID must be set"),
    );

    // Configurar cliente HTTP con timeouts mayores para Cal.com
    let cal_client: HttpClient = HttpClient::builder()
        .timeout(std::time::Duration::from_secs(60)) // Timeout total de 60 segundos
        .connect_timeout(std::time::Duration::from_secs(10)) // Timeout de conexión de 10 segundos
        .pool_idle_timeout(std::time::Duration::from_secs(90)) // Mantener conexiones idle por 90 segundos
        .pool_max_idle_per_host(10) // Máximo de conexiones idle por host
        .tcp_keepalive(std::time::Duration::from_secs(30)) // Enviar keepalive cada 30 segundos
        .build()
        .expect("Failed to build Cal.com HTTP client");

    let cal_options: CalOptions = CalOptions {
        client: cal_client,
        base_url: env::var("CAL_BASE_URL").expect("CAL_BASE_URL must be set"),
        api_key: env::var("CAL_API_KEY").expect("CAL_API_KEY must be set"),
        team_id: env::var("TEAM_ID").expect("TEAM_ID must be set"),
        booking_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        recent_changes: Arc::new(tokio::sync::RwLock::new(Vec::new())),
    };

    // Configurar cliente HTTP para Google Analytics
    let ga_options: GAOptions = GAOptions {
        client: HttpClient::new(),
        service_account: ServiceAccount {
            client_email: env::var("GA_CLIENT_EMAIL").expect("GA_CLIENT_EMAIL must be set"),
            private_key: env::var("GA_PRIVATE_KEY").expect("GA_PRIVATE_KEY must be set"),
        },
        base_url: "https://analyticsdata.googleapis.com/v1beta".to_string(),
        property_id: env::var("GA_PROPERTY_ID").expect("GA_PROPERTY_ID must be set"),
    };

    // Inicializar el estado de la aplicación y el enrutador
    let state: Arc<AppState> = Arc::new(AppState {
        firebase_options,
        stripe_client,
        resend_client,
        mailchimp_client,
        cal_options,
        ga_options,
    });

    // Configuración de CORS (Cross-Origin Resource Sharing)
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "http://localhost:4321".parse().unwrap(), // Frontend desarrollo
            "https://amanahacademia.com".parse().unwrap(), // Dominio de producción
            "https://amanahacademia.vercel.app".parse().unwrap(), // Dominio alternativo de producción
        ]))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    // Polling cada 5 minutos para recuperar bookings que no llegaron vía webhook.
    // Se spawnea antes de `with_state` porque necesita ownership del clone.
    let state_for_polling: Arc<AppState> = state.clone();
    tokio::spawn(async move {
        info!("Iniciando tarea de polling de Cal.com");
        controllers::webhook::polling_task(state_for_polling).await;
        error!("La tarea de polling ha terminado inesperadamente");
    });

    // Configurar el enrutador de la aplicación
    let app: Router = Router::new()
        .nest("/users", routes::users::router(state.clone()))
        .nest("/comments", routes::comments::router(state.clone()))
        .nest("/payment", routes::payments::router(state.clone()))
        .nest("/teachers", routes::teachers::router(state.clone()))
        .nest("/email", routes::email::router(state.clone()))
        .nest("/mailchimp", routes::mailchimp::router(state.clone()))
        .nest("/cal", routes::cal::router(state.clone()))
        .nest("/metrics", routes::metrics::router(state.clone()))
        .nest("/webhook", routes::webhooks::router(state.clone()))
        .layer(cors)
        .layer(TraceLayer::new_for_http()) // Logging de requests para debugging
        .with_state(state); // Estado compartido

    // Inicializar el listener TCP y arrancar el servidor
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Server listening on http://{}", addr);
    match axum::serve(listener, app).await {
        Ok(_) => info!("Server finalized"),
        Err(err) => error!("Error in server: {}", err),
    };
}
