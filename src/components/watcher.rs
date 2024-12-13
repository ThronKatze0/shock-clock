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

fn get_classes(route: ReadSignal<WatcherRoute>, desired: WatcherRoute) -> String {
    format!(
        "btn join-item flex-1 rounded-l-lg{}",
        if route() == desired {
            " button-primary"
        } else {
            ""
        }
    )
}

#[component]
pub fn Watcher() -> impl IntoView {
    let (route, set_route) = create_signal(WatcherRoute::Blacklist);
    mview! {
        div class="join flex" {
            button class={move || get_classes(route, WatcherRoute::Blacklist)} { "Blacklist" }
            button class={move || get_classes(route, WatcherRoute::Whitelist)} { "Whitelist" }
        }
    }
}
