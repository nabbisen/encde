use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    str: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let encode_input_ref = use_node_ref();
    let decode_input_ref = use_node_ref();

    let encode_target = use_state(|| String::new());
    let decode_target = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let encode_target = encode_target.clone();
        let encode_target2 = encode_target.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if encode_target.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { str: &*encode_target }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("encode_base64", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            encode_target2,
        );
    }
    {
        let greet_msg = greet_msg.clone();
        let decode_target = decode_target.clone();
        let decode_target2 = decode_target.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if decode_target.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { str: &*decode_target }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("decode_base64", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
            decode_target2,
        );
    }
    let encode = {
        let encode_target = encode_target.clone();
        let encode_input_ref = encode_input_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            encode_target.set(
                encode_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };
    let decode = {
        let decode_target = decode_target.clone();
        let decode_input_ref = decode_input_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            decode_target.set(
                decode_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <>
            <h1>{"Encde"}</h1>

            <main class="container">
                <div class="row">
                    <input id="encode-input" ref={encode_input_ref} placeholder="Enter decode target..." />
                    <button type="button" onclick={encode}>{"Encode"}</button>
                </div>
                <div class="row">
                    <input id="decode-input" ref={decode_input_ref} placeholder="Enter decode target..." />
                    <button type="button" onclick={decode}>{"Decode"}</button>
                </div>
                
                <p><b>{ &*greet_msg }</b></p>
            </main>
        </>
    }
}
