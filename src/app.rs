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
    let encode_ref = use_node_ref();
    let decode_ref = use_node_ref();

    let target_str = use_state(|| String::new());
    let target_kind = use_state(|| false);

    {
        let is_decode = *target_kind;
        let processed = if is_decode {
            encode_ref.clone()
        } else {
            decode_ref.clone()
        };
        let target_str = target_str.clone();
        let target_str2 = target_str.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if target_str.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { str: &*target_str }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let cmd = if is_decode {
                        "decode_base64"
                    } else {
                        "encode_base64"
                    };
                    let ret = invoke(cmd, args).await;
                    if !ret.is_null() {
                        processed
                            .cast::<web_sys::HtmlInputElement>()
                            .unwrap()
                            .set_value(&ret.as_string().expect("todo"));
                    }
                });
            },
            target_str2,
        );
    }
    let encode = { onblur(&target_str, &encode_ref, &target_kind.clone(), false) };
    let decode = { onblur(&target_str, &decode_ref, &target_kind.clone(), true) };

    html! {
        <>
            <h1>{"Encde"}</h1>

            <main class="container">
                <div class="row">
                    <h2>{"Encode target"}</h2>
                    <input id="encode-input" ref={encode_ref} onblur={encode} placeholder="Enter decode target..." />
                    <button type="button">{"Copy (todo)"}</button>
                </div>
                <div class={"select"} style={"width: 8em; margin-left: auto; margin-right: auto;"}>
                    <ul><li>{"kind (todo)"}</li></ul>
                </div>
                <div class="row">
                    <h2>{"Decode target"}</h2>
                    <input id="decode-input" ref={decode_ref} onblur={decode} placeholder="Enter decode target..." />
                    <button type="button">{"Copy (todo)"}</button>
                </div>
            </main>
        </>
    }
}

fn onblur(
    target_str: &UseStateHandle<String>,
    input_ref: &NodeRef,
    target_kind: &UseStateHandle<bool>,
    kind: bool,
) -> Callback<FocusEvent> {
    let target_str = target_str.clone();
    let input_ref = input_ref.clone();
    let target_kind = target_kind.clone();
    Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        target_str.set(
            input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value(),
        );
        target_kind.set(kind);
    })
}
