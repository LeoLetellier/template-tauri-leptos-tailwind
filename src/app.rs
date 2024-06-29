use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="m-0 pt-10 flex-1 justify-center" data-theme="dark">
            <div class="flex place-content-evenly pt-5 pb-20">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="h-60 drop-shadow-tauri-s hover:drop-shadow-tauri-xl" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="h-60 drop-shadow-leptos-s hover:drop-shadow-leptos-xl" alt="Leptos logo"/>
                </a>
            </div>

            <p class="font-sans text-2xl text-center py-6">"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="flex place-content-center py-6 gap-20" on:submit=greet>
                <input
                    class="input text-lg"
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button class="btn text-lg" type="submit">"Greet"</button>
            </form>

            <p class="text-lg text-center"><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
