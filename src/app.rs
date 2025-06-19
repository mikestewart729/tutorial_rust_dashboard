pub mod components;
pub mod db;
pub mod models;
pub mod server_functions;
pub mod pages;
use pages::{HomePage, TeamPage};

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_router::components::{Router, Route, Routes};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dashboard-app.css"/>
        <link data-trunk rel="tailwind-css" href="/style/input.css "/>

        // sets the document title
        <Title text="Full-Stack Dashboard App"/>

        // content for this welcome page
        <Router>
            <main>
                <body class="bg-gray-900 overflow-x-hide" />
                <Routes fallback=|| "Not found."> // Might not need this fallback lambda?
                    <Route path="/" view=move || {
                        view! {
                            <HomePage />
                        }
                    }/>
                    <Route path="/team" view=move || {
                        view! {
                            <TeamPage />
                        }
                    }/>
                    <Route path=path!("/*any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
