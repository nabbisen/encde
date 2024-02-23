use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct EncdeArgs<'a> {
    str: &'a str,
}

enum EncdeType {
    Base64,
    Md5,
    // Sha256,
    // Sha512,
    // Url,
    // UrlSafe,
    // Jwt,
}

#[function_component(App)]
pub fn app() -> Html {
    // on mounted
    use_effect_with_deps(
        |_| {
            let input_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector(".select input[type=\"radio\"]")
                .unwrap()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            input_element.click();
        },
        // on mounted only
        Vec::<bool>::new(),
    );

    let encode_ref = use_node_ref();
    let decode_ref = use_node_ref();

    let target_str = use_state(|| String::new());
    let target_direction = use_state(|| false);
    let target_kind = use_state(|| String::new());

    {
        let is_decode = *target_direction;
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

                    let args = to_value(&EncdeArgs { str: &*target_str }).unwrap();
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
    let encode = { onblur(&target_str, &encode_ref, &target_direction.clone(), false) };
    let decode = { onblur(&target_str, &decode_ref, &target_direction.clone(), true) };

    html! {
        <>
            <h1><span style={"margin-right: 0.6rem;"}>{ target_kind.as_str() }</span>{"Encde"}</h1>

            <main class="container">
                { select(&target_kind) }
                <div class="encde encode">
                    <div class="row header">
                        <h2>{"Encode target"}</h2>
                        <button type="button">{"Copy (todo)"}</button>
                    </div>
                    <textarea ref={encode_ref} onblur={encode} placeholder="Enter decode target..." />
                </div>
                <div class="encde decode">
                    <div class="row header">
                        <h2>{"Decode target"}</h2>
                        <button type="button">{"Copy (todo)"}</button>
                    </div>
                    <textarea ref={decode_ref} onblur={decode} placeholder="Enter decode target..." />
                </div>
            </main>
        </>
    }
}

fn onblur(
    target_str: &UseStateHandle<String>,
    input_ref: &NodeRef,
    target_direction: &UseStateHandle<bool>,
    kind: bool,
) -> Callback<FocusEvent> {
    let target_str = target_str.clone();
    let input_ref = input_ref.clone();
    let target_direction = target_direction.clone();
    Callback::from(move |_: FocusEvent| {
        target_str.set(
            input_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value(),
        );
        target_direction.set(kind);
    })
}

fn select(target_kind: &UseStateHandle<String>) -> Html {
    html! {
        <div class="select" style="width: 8em; margin-left: auto; margin-right: auto;">
            <ul>
            {
                vec![
                    EncdeType::Base64,
                    EncdeType::Md5,
                ].iter().map(|t| {
                    {
                        let type_name =match &t {
                            EncdeType::Base64 => "Base64",
                            EncdeType::Md5 => "Md5",
                        };
                        html! {
                            <li>
                                <label>
                                    <input data-encde-kind={type_name} type="radio"
                                        name="encde-type"
                                        onclick={

                                        let target_kind = target_kind.clone();
                                        Callback::from(move |e: MouseEvent| {
                                            let target_kind_id = e.target().expect("todo")
                                                .dyn_into::<HtmlInputElement>().expect("todo")
                                                .dataset().get("encde-kind").expect("todo");
                                            target_kind.set(target_kind_id);
                                        })

                                    }/>
                                    {type_name}
                                </label>
                            </li>
                        }
                    }
                }).collect::<Html>()
            }
            </ul>
        </div>
    }
}
