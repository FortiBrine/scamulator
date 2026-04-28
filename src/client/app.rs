use leptos::{component, view, IntoView};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::client::pages::{HomePage, PageNotFound};

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
