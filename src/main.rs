#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use axum::Router;
#[cfg(feature = "ssr")]
use axum::routing::post;
#[cfg(feature = "ssr")]
use leptos::logging::log;
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use scamulator::client::app::App;
#[cfg(feature = "ssr")]
use scamulator::server::calculate;
#[cfg(feature = "ssr")]
use scamulator::server::shell;
#[cfg(feature = "ssr")]
use scamulator::server::state::AppState;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let server_state = Arc::new(AppState {
        leptos_options: leptos_options.clone()
    });

    let api_router = Router::new()
        .route("/calculate", post(calculate))
        .with_state(server_state);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .nest("/api/v1", api_router)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {

}
