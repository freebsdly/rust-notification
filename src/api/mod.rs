mod interface;

use crate::api::interface::ApiDoc;
use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
use axum::{Router, serve};
use axum_prometheus::GenericMetricLayer;
use axum_prometheus::Handle;
use axum_prometheus::PrometheusMetricLayerBuilder;
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use log::info;
use serde::Deserialize;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct ApiState {}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiServiceArgs {
    #[serde(alias = "address")]
    pub address: String,
    pub port: u16,
    pub timeout: u64,
}

pub struct ApiService {
    args: ApiServiceArgs,
    cancel_token: CancellationToken,
}

impl ApiService {
    pub fn new(token: CancellationToken, args: ApiServiceArgs) -> Result<Self, anyhow::Error> {
        Ok(Self {
            cancel_token: token,
            args,
        })
    }

    fn routes() -> Router<Arc<ApiState>> {
        Router::new()
            .route("/", get(interface::index))
            .route("/users", get(interface::get_users))
    }

    pub fn start(&self) -> Result<(), anyhow::Error> {
        info!("starting api service");
        let token = self.cancel_token.clone();
        let args = self.args.clone();
        let addr = format!("{}:{}", args.address, args.port);
        info!("listening on {}", addr);
        let listener = std::net::TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let app_state = ApiState {};
        let state = Arc::new(app_state);
        tokio::spawn(Self::start_app(token, listener, args, state));
        Ok(())
    }

    async fn start_app(
        token: CancellationToken,
        listener: std::net::TcpListener,
        args: ApiServiceArgs,
        state: Arc<ApiState>,
    ) -> Result<(), anyhow::Error> {
        let (prometheus_layer, metric_handle) = Self::build_metrics();
        let api = ApiDoc::openapi();
        // Create a regular axum app.
        let app = Router::<Arc<ApiState>>::new()
            .nest("/api", Self::routes())
            .route("/metrics", get(|| async move { metric_handle.render() }))
            .fallback(interface::handler_404)
            // request trace
            .layer(TraceLayer::new_for_http())
            // request timeout
            .layer(TimeoutLayer::new(Duration::from_secs(args.timeout)))
            // prometheus metric
            .layer(prometheus_layer)
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(interface::handle_error))
                    .timeout(Duration::from_secs(args.timeout)),
            )
            .with_state(state)
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

        let tcp_listener = TcpListener::from_std(listener)?;
        // Run the server with graceful shutdown
        let _ = serve(tcp_listener, app)
            .with_graceful_shutdown(async move {
                select! {
                    _ = token.cancelled() => {
                        info!("received shutdown api service signal");
                    },
                }
            })
            .await;
        Ok(())
    }

    fn build_metrics() -> (
        GenericMetricLayer<'static, PrometheusHandle, Handle>,
        PrometheusHandle,
    ) {
        PrometheusMetricLayerBuilder::new()
            .with_ignore_patterns(&["/metrics"])
            .with_default_metrics()
            .build_pair()
    }

    pub fn stop(&self) -> Result<(), anyhow::Error> {
        info!("Stopping ApiService");
        self.cancel_token.cancel();
        Ok(())
    }
}
