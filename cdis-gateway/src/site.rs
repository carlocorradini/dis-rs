use std::convert::Infallible;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::response::sse::{Event as SseEvent, Sse};
use axum::Router;
use axum::routing::get;
use axum_extra::{headers, TypedHeader};
use futures::stream::Stream;
use tokio::signal;
use tokio_stream::StreamExt;
// use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{debug, error};
use tracing::log::trace;
use crate::{Command};
use crate::config::Config;
use crate::site::templates::{ConfigMetaTemplate, ConfigTemplate, HomeTemplate};
use crate::stats::{GatewayStats, SseStat};

// const ASSETS_DIR: &str = "templates";

struct SiteState {
    config: Config,
    stats_tx: tokio::sync::broadcast::Sender<SseStat>,
}

pub async fn run_site(config: Config,
                      stats_tx: tokio::sync::broadcast::Sender<SseStat>,
                      cmd_tx: tokio::sync::broadcast::Sender<Command>) {
    // let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(ASSETS_DIR);
    // let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    let state = Arc::new(SiteState {
        config: config.clone(),
        stats_tx
    });

    let router = Router::new()
        // .fallback_service(static_files_service)
        .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http())
        .route("/", get(home))
        .route("/styles.css", get(styles))
        .route("/index.js", get(scripts))
        .route("/script_htmx.js", get(script_htmx))
        .route("/script_htmx_sse.js", get(script_htmx_sse))
        .route("/config", get(config_info))
        .route("/meta", get(meta_info))
        .route("/clicked", get(clicked))
        .with_state(state);

    // TODO handle bind error
    let host_ip = format!("127.0.0.1:{}", config.site_host);
    let listener = tokio::net::TcpListener::bind(&host_ip)
        .await
        .expect(format!("Failed to bind TCP socket for Web UI - {}", host_ip).as_str());
    tracing::debug!("Site listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    match cmd_tx.send(Command::Quit) {
        Ok(_) => {}
        Err(_) => { error!("Could not send Command::Quit.") }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    State(state): State<Arc<SiteState>>,
) -> Sse<impl Stream<Item = Result<SseEvent, Infallible>>> {
    debug!("`{}` connected", user_agent.as_str());

    let mut rx = state.stats_tx.subscribe();
    let stream = async_stream::try_stream! {
        yield SseEvent::default().event("status").data("SSE connected");

        while let Ok(stat) = rx.recv().await {
            yield match stat {
                SseStat::DisSocket(stat) => {
                    SseEvent::default().event("dis_socket").data(format!("<div>packets: {}</div></br><div>bytes: {}</div>", stat.packets_received, stat.bytes_received).as_str())
                }
                SseStat::CdisSocket(stat) => {
                    SseEvent::default().event("cdis_socket").data(format!("<div>packets: {}</div></br><div>bytes: {}</div>", stat.packets_received, stat.bytes_received).as_str())
                }
                SseStat::Encoder(stat) => {
                    SseEvent::default().event("encoder").data(format!("<div>encoded: {}</div></br><div>rejected: {}</div>", stat.received_count.values().len(), stat.rejected_count).as_str())
                }
                SseStat::Decoder(stat) => {
                    SseEvent::default().event("decoder").data(format!("<div>decoded: {}</div></br><div>rejected: {}</div>", stat.received_count.values().len(), stat.rejected_count).as_str())
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            // .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

pub async fn styles() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../build/styles.css").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub async fn scripts() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../build/index.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub async fn script_htmx() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../assets/scripts/htmx-1.9.12-min.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub async fn script_htmx_sse() -> Result<impl IntoResponse, Response> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(include_str!("../assets/scripts/htmx-1.9.12-ext-sse.js").to_owned());

    match response {
        Ok(response) => { Ok(response) }
        Err(e) => { Err((StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP error: {e}")).into_response()) }
    }
}

pub async fn config_info(State(state): State<Arc<SiteState>>) -> impl IntoResponse {
    ConfigMetaTemplate {
        name: state.config.meta.name.clone(),
        author: state.config.meta.author.clone(),
        version: state.config.meta.version.clone(),
    }
}

pub async fn meta_info(State(state): State<Arc<SiteState>>) -> impl IntoResponse {
    ConfigMetaTemplate {
        name: state.config.meta.name.clone(),
        author: state.config.meta.author.clone(),
        version: state.config.meta.version.clone(),
    }
}

pub async fn clicked() -> impl IntoResponse {
    ConfigTemplate
}

pub async fn home() -> impl IntoResponse {
    HomeTemplate
}

mod templates {
    use askama_axum::Template;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct HomeTemplate;

    #[derive(Template)]
    #[template(path = "config.html")]
    pub struct ConfigTemplate;

    #[derive(Template)]
    #[template(path = "config_meta.html")]
    pub struct ConfigMetaTemplate {
        pub(crate) name: String,
        pub(crate) author: String,
        pub(crate) version: String,
    }
}
