use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::Signal;
use leptos::SignalGet;
use leptos::WriteSignal;
use leptos_mview::mview;
use serde::{Deserialize, Serialize};
use shock_clock_utils::Block;

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
            RadioOption value={WatcherRoute::Blacklist} set_signal={set_route} route={route} btn_size=""()
            RadioOption value={WatcherRoute::Whitelist} set_signal={set_route} route={route} btn_size=""()
        }

        div class="join flex m-5" {
            RadioOption value={BlockTypeRoute::App} set_signal={set_block_type} route={block_type} btn_size="btn-sm"()
            RadioOption value={BlockTypeRoute::Website} set_signal={set_block_type} route={block_type} btn_size="btn-sm"()
            RadioOption value={BlockTypeRoute::Keyword} set_signal={set_block_type} route={block_type} btn_size="btn-sm"()
        }

        p({move || log()})

        div class="flex flex-col" {

        }

    }
}

#[component]
fn RadioOption<T>(
    value: T,
    set_signal: WriteSignal<T>,
    route: ReadSignal<T>,
    btn_size: &'static str,
) -> impl IntoView
where
    T: Clone + Copy + PartialEq + Display + 'static,
{
    mview! {
        input class={move || format!("btn {} join-item flex-1 rounded-l-lg", btn_size)} on:click={move |_| set_signal(value)} type="radio" name="watcherRoute" aria-label={move || value.to_string()} checked={move || route() == value}()
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
