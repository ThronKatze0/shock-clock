use std::fmt::Display;
use std::fmt::Formatter;

use leptos::component;
use leptos::create_signal;
use leptos::IntoView;
use leptos::ReadSignal;
use leptos::SignalGet;
use leptos_mview::mview;

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
            input class="btn join-item flex-1 rounded-l-lg" on:click={move |_| set_block_type(BlockTypeRoute::App)} type="radio" name="blockType" aria-label="App" checked={move || block_type() == BlockTypeRoute::App}()
            input class="btn join-item flex-1 rounded-l-lg" on:click={move |_| set_block_type(BlockTypeRoute::Website)} type="radio" name="blockType" aria-label="Website" checked={move || block_type() == BlockTypeRoute::Website}()
            input class="btn join-item flex-1 rounded-l-lg" on:click={move |_| set_block_type(BlockTypeRoute::Keyword)} type="radio" name="blockType" aria-label="Keyword" checked={move || block_type() == BlockTypeRoute::Keyword}()
        }

        // div class="join flex" {
        //     input class={move || get_classes(route, )} on:click={move |_| set_route(WatcherRoute::Blacklist)} type="radio" name="options" aria-label="Blocklist" checked()
        //     input class={move || get_classes(route, WatcherRoute::Whitelist)} on:click={move |_| set_route(WatcherRoute::Blacklist)} type="radio" name="options" aria-label="Whitelist"()
        // }

        p({move || log()})

    }
}
