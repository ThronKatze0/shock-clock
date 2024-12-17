use leptos::create_effect;
use leptos::SignalUpdate;
use shock_clock_utils::AppBlockData;
use shock_clock_utils::BlockType;
use shock_clock_utils::ShockStrength;
use shock_clock_utils::WebsiteBlockData;
use std::fmt::Display;
use std::fmt::Formatter;
use uuid;

use icondata as i;
use leptos::For;
use leptos::Show;
use leptos_icons::*;

use leptos::component;
use leptos::create_signal;
use leptos::logging;
use leptos::spawn_local;
use leptos::Effect;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::Signal;
use leptos::SignalGet;
use leptos::WriteSignal;
use leptos_mview::mview;
use shock_clock_utils::Block;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    // They need to have different names!
}
async fn update_block_data(blocks: &Vec<Block>) {
    invoke("update_blocklist", to_value(blocks).expect("real bad")).await;
}

#[derive(Clone, Copy, PartialEq)]
enum WatcherRoute {
    Blacklist,
    Whitelist,
}

impl Display for WatcherRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WatcherRoute::Blacklist => write!(f, "Blacklist"),
            WatcherRoute::Whitelist => write!(f, "Whitelist"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BlockTypeRoute {
    All,
    App,
    Website,
    Keyword,
}

impl Display for BlockTypeRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            BlockTypeRoute::All => write!(f, "All"),
            BlockTypeRoute::App => write!(f, "App"),
            BlockTypeRoute::Website => write!(f, "Website"),
            BlockTypeRoute::Keyword => write!(f, "Keyword"),
        }
    }
}

#[component]
pub fn Watcher() -> impl IntoView {
    let (route, set_route) = create_signal(WatcherRoute::Blacklist);
    let (block_type, set_block_type) = create_signal(BlockTypeRoute::All);

    let (blocks, set_blocks) = create_signal(Vec::new());

    let filtered_blocks = move || match block_type() {
        BlockTypeRoute::All => blocks(),
        BlockTypeRoute::App => blocks()
            .into_iter()
            .filter(|block: &Block| match block.block_type {
                BlockType::App(_) => true,
                _ => false,
            })
            .collect(),
        BlockTypeRoute::Website => blocks()
            .into_iter()
            .filter(|block: &Block| match block.block_type {
                BlockType::Website(_) => true,
                _ => false,
            })
            .collect(),
        BlockTypeRoute::Keyword => blocks()
            .into_iter()
            .filter(|block: &Block| match block.block_type {
                BlockType::Keyword => true,
                _ => false,
            })
            .collect(),
    };

    let add_block = move |block: Block| set_blocks.update(|blocks| blocks.push(block));

    let remove_block = move |uuid: uuid::Uuid| {
        set_blocks.update(|blocks| blocks.retain(|block| block.uuid != uuid))
    };

    let change_shock_strength = move |uuid: uuid::Uuid, shock_strength| {
        set_blocks.update(|blocks| {
            blocks
                .iter_mut()
                .find(|block| block.uuid == uuid)
                .expect("m")
                .shock_strength = shock_strength
        })
    };

    add_block(Block {
        uuid: uuid::Uuid::new_v4(),
        name: "Tiktok".to_string(),
        shock_strength: ShockStrength::Normal,
        block_type: BlockType::App(AppBlockData {
            package_name: "com.musically.smth".to_string(),
        }),
    });
    add_block(Block {
        uuid: uuid::Uuid::new_v4(),
        name: "Google".to_string(),
        shock_strength: ShockStrength::Normal,
        block_type: BlockType::Website(WebsiteBlockData {
            url: "www.google.com".to_string(),
        }),
    });
    add_block(Block {
        uuid: uuid::Uuid::new_v4(),
        name: "Halil".to_string(),
        shock_strength: ShockStrength::Normal,
        block_type: BlockType::Keyword,
    });

    // Effect::new(move |_| {
    //     logging::log!("yeah async");
    //     let cloned_blocks = blocks();
    //     spawn_local(async move {
    //         update_block_data(&cloned_blocks).await;
    //     });
    // });

    let log = move || {
        format!(
            "WatcherRoute: {}\nBlockTypeRoute: {}\n\n",
            route(),
            block_type()
        )
    };

    mview! {
        div class="sticky top-0 z-50 bg-base-100 pb-3 pt-3" {
            div class="join flex mx-5" {
                RadioOption value={WatcherRoute::Blacklist} set_signal={set_route} route={route} btn_size="" name="list"()
                RadioOption value={WatcherRoute::Whitelist} set_signal={set_route} route={route} btn_size="" name="list"()
            }

            div class="join flex mx-5 mt-3" {
                RadioOption value={BlockTypeRoute::All} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
                RadioOption value={BlockTypeRoute::App} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
                RadioOption value={BlockTypeRoute::Website} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
                RadioOption value={BlockTypeRoute::Keyword} set_signal={set_block_type} route={block_type} btn_size="btn-sm" name="blockType"()
            }
        }
        p({move || log()})

        button on:click={move |_| {
            add_block(Block {
                uuid: uuid::Uuid::new_v4(),
                name: "App".to_string(),
                shock_strength: ShockStrength::Normal,
                block_type: BlockType::App(AppBlockData {
                    package_name: "com.musically.smth".to_string(),
                }),
            });
        }}("Add smth")

        div class="overflow-y-auto pb-20" {
            ul class="divide-y divide-gray-200" {
                For
                    each={move || filtered_blocks()}
                    key={|block| block.uuid}
                    children={move |block| mview! {
                        BlockElement {block}()
                    }}()
            }
        }

    }
}

#[component]
fn RadioOption<T>(
    value: T,
    set_signal: WriteSignal<T>,
    route: ReadSignal<T>,
    btn_size: &'static str,
    name: &'static str,
) -> impl IntoView
where
    T: Clone + Copy + PartialEq + Display + 'static,
{
    mview! {
        input
            class={move || format!("btn {} join-item flex-1 rounded-l-lg", btn_size)}
            on:click={move |_| set_signal(value)}
            type="radio"
            name={name}
            aria-label={value.to_string()}
            checked={move || route() == value}()
    }
}

#[component]
fn BlockElement(block: Block) -> impl IntoView {
    mview! {
        li class="flex items-center justify-between p-4" {
            div class="flex items-start space-x-3" {
                {match &block.block_type {
                    BlockType::App(_) => mview!{ Icon width="3em" height="3em" icon={i::AiAppstoreOutlined}() },
                    BlockType::Website(_) => mview!{ Icon width="3em" height="3em" icon={i::MdiWeb}() },
                    BlockType::Keyword => mview!{ Icon width="3em" height="3em" icon={i::BsCardText}() }
                }}
                div {
                    span class="text-white text-2xl"({block.name})
                    p class="text-sm text-gray-400"({move || match &block.block_type {
                        BlockType::App(ref app_data) => app_data.package_name.clone(),
                        BlockType::Website(ref website_data) => website_data.url.clone(),
                        _ => "".to_string()
                    }})
                }
            }
            button class="btn btn-secondary" {
                Icon width="2em" height="2em" icon={i::BsTrash}()
            }
        }
    }
}
