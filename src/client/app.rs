use leptos::config::LeptosOptions;
use leptos::{component, view, IntoView};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::client::pages::{HomePage, PageNotFound};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/scamulator.css"/>

        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| PageNotFound()>
                    <Route path=path!("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
