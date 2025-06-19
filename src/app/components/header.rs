use leptos::*;
use leptos::prelude::*;
use leptos_router::hooks::*;
use leptos_router::components::*;
use leptos_router::path;

const INPUT_STYLE: &str = "border-b-0 border-[#7734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";
const INPUT_STYLE_SELECTED: &str = "border-b-2 border-[#9734e7] h-8 text-white ml-4 mr-4 hover:border-b-2";

#[component]
pub fn Header() -> impl IntoView {

    let (current_path, set_current_path) = signal(String::new());

    Effect::new(move |_| {

        // Get the current url and save it
        let url = use_url().get();
        let path = url.path();
        match path {
            // "/team" => { set_current_path(String::from(path)) },
            // _ => { set_current_path(String::from("/")) }
            "team" => { "/team" },
            _ => "/"
        };
        //set_current_path.set(path.to_string());
        set_current_path.update(|current_path: &mut String| *current_path = String::from(path));
    });

    view! {

        <div class="flex mx-auto align-center items-center w-full h-12 pt-8 px-20 top-0 fixed">
            <nav class="flex flex-row w-full max-w-[52rem] h-12">
                <div class={move || get_style_from_url(&current_path, "/")}>
                    <A href="/">"Dashboard"</A>
                </div>
                <div class={move || get_style_from_url(&current_path, "/team")}>
                    <A href="/team">"Team"</A>
                </div>
            </nav>
        </div>
    }
}

pub fn get_style_from_url<'a, 'b>(url: &'a ReadSignal<String>, match_url: &'a str) -> &'b str {
    if url.get() == match_url {
        INPUT_STYLE_SELECTED
    } else {
        INPUT_STYLE
    }
}