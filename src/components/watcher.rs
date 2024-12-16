use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::Signal;
use leptos::SignalGet;
use leptos_mview::mview;
use serde::{Deserialize, Serialize};
use shock_clock_utils::Block;

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
enum BlockTypeRoute {
    App,
    Website,
    Keyword,
}

impl Display for BlockTypeRoute {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            BlockTypeRoute::App => write!(f, "App"),
            BlockTypeRoute::Website => write!(f, "Website"),
            BlockTypeRoute::Keyword => write!(f, "Keyword"),
        }
    }
}

#[component]
pub fn Watcher() -> impl IntoView {
    let (route, set_route) = create_signal(WatcherRoute::Blacklist);
    let (block_type, set_block_type) = create_signal(BlockTypeRoute::App);

    let blocks: Vec<Block> = Vec::new();

    let update_backend = Signal::derive(|| {});

    let log = move || {
        format!(
            "WatcherRoute: {}\nBlockTypeRoute: {}\n\n",
            route(),
            block_type()
        )
    };

    mview! {
        div class="join flex m-5" {
            input class="btn join-item flex-1 rounded-l-lg" on:click={move |_| set_route(WatcherRoute::Blacklist)} type="radio" name="watcherRoute" aria-label="Blocklist" checked={move || route() == WatcherRoute::Blacklist}()
            input class="btn join-item flex-1 rounded-l-lg" on:click={move |_| set_route(WatcherRoute::Whitelist)} type="radio" name="watcherRoute" aria-label="Whitelist" checked={move || route() == WatcherRoute::Whitelist}()
        }

        div class="join flex m-5" {
            input class="btn btn-sm join-item flex-1 rounded-l-lg" on:click={move |_| set_block_type(BlockTypeRoute::App)} type="radio" name="blockType" aria-label="App" checked={move || block_type() == BlockTypeRoute::App}()
            input class="btn btn-sm join-item flex-1 rounded-l-lg" on:click={move |_| set_block_type(BlockTypeRoute::Website)} type="radio" name="blockType" aria-label="Website" checked={move || block_type() == BlockTypeRoute::Website}()
            input class="btn btn-sm join-item flex-1 rounded-l-lg" on:click={move |_| set_block_type(BlockTypeRoute::Keyword)} type="radio" name="blockType" aria-label="Keyword" checked={move || block_type() == BlockTypeRoute::Keyword}()
        }

        p({move || log()})

        div class="flex flex-col" {

        }

    }
}
//
// #[component]
// fn BlockElement(block: Block) -> impl IntoView {
//     mview! {
//         div class="card" {
//             div class="card-body flex flex-row" {
//                 h2({move || match block.clone() {
//                     Block::App(data) => data.app_name,
//                     Block::Website(data) => data.url,
//                     Block::Keyword(data) => data.name,
//                 }})
//
//                 div class="dropdown dropdown-top" {
//                     div tabindex="0" role="button" class="btn" {
//
//                     }
//                 }
//             }
//         }
//     }
// }
