use leptos::*;
use leptos::prelude::*;
use crate::app::components::Header;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <body class="bg-gray-900 overflow-x-hide">
            <div class="w-full max-w-[64rem] mx-auto items-center justify-center align-center text-white">
                <Header />
                "Home Page here"
            </div>
        </body>
    }
}