use leptos::on_cleanup;
use leptos::set_interval_with_handle;
use leptos::Signal;
use std::time::Duration;

use icondata as i;
use icondata::Icon;
use leptos::component;
use leptos::create_signal;
use leptos::set_interval;
use leptos::IntoView;
use leptos_icons::Icon;
use leptos_mview::mview;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct ShockArgs {
    duration: u16,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

enum Status {
    Offline,
    Online,
}

#[component]
pub fn Home() -> impl IntoView {
    let shock_test = move |_| {
        spawn_local((async move || {
            let args = to_value(&ShockArgs { duration: 1000 }).unwrap();
            invoke("shock", args).await;
        })());
    };
    let (clock_stat, set_clock_stat) = create_signal(false);
    let handle = set_interval_with_handle(
        move || {
            let stat_clone = set_clock_stat.clone();
            let async_closure = async move || {
                stat_clone(
                    invoke("is_connected", JsValue::undefined())
                        .await
                        .as_bool()
                        .unwrap(),
                );
            };
            spawn_local(async_closure());
        },
        Duration::from_secs(5),
    );
    on_cleanup(move || {
        handle;
    });
    let get_icon = move || {
        if clock_stat() {
            i::AiCheckOutlined
        } else {
            i::AiCloseOutlined
        }
    };
    mview! {
        div class="prose h-screen" {
            h1 class="my-4" { "Shock Clock" }
            div class="flex flex-col h-4/5 my-6" {
                div class="stats" {
                    div class="stat" {
                        span class="stat-title" { "Watcher" }
                    }
                    div class="stat" {
                        span class="stat-title" { "Clock" }
                        Icon icon={Signal::derive(get_icon)};
                    }
                }

                div class="flex flex-auto h-6/16 pt-48" {
                    div class="flex-1";
                    button on:click={shock_test} class="btn center text-6xl rounded-full flex-auto h-4/6 w-1/12 border-yellow-500 border-4" {"⚡"}
                    div class="flex-1";
                }
                div class="flex-1 form-control" {
                    label class="label cursor-pointer" {
                        span class="label-text" { "Block Social Media" }
                        input type="checkbox" class="toggle";
                    }
                    label class="label cursor-pointer" {
                        span class="label-text" { "Block Shorts" }
                        input type="checkbox" class="toggle";
                    }
                    label class="label cursor-pointer" {
                        span class="label-text" { "Block Adult Content" }
                        input type="checkbox" class="toggle";
                    }
                }
            }
        }
    }
}

pub mod watcher;

use wasm_bindgen_futures::spawn_local;
pub use watcher::Watcher;

#[component]
pub fn Games() -> impl IntoView {
    mview! {}
}
