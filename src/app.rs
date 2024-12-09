use std::fmt::Display;

use icondata as i;
use icondata_core::IconData;
use leptos::*;
use leptos_icons::*;
use leptos_mview::mview;

#[derive(Clone)]
enum SelectedRoute {
    Home,
    Devices,
    Alarm,
}

impl Display for SelectedRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Alarm => "Alarm",
                Self::Devices => "Devices",
                Self::Home => "Home",
            }
        )
    }
}

#[derive(Clone)]
struct RouteSetter(WriteSignal<SelectedRoute>);

#[component]
pub fn App() -> impl IntoView {
    let (selected_route, set_selected_route) = create_signal(SelectedRoute::Home);
    provide_context(RouteSetter(set_selected_route));

    mview! {
        {move || selected_route().to_string()}
        div class="btm-nav" {
            BtmNavItem route={SelectedRoute::Home} icon={i::AiHomeOutlined}()
            BtmNavItem route={SelectedRoute::Devices} icon={i::BiDevicesRegular}()
            BtmNavItem route={SelectedRoute::Alarm} icon={i::ChClockAlarm}()
        }
    }
}

#[component]
fn BtmNavItem(route: SelectedRoute, icon: &'static IconData) -> impl IntoView {
    let ssr = use_context::<RouteSetter>().unwrap();
    let moved_route = route.clone(); // no idea why it wants two copies, with normal view! I only
                                     // need one
    mview! {
        button on:click={move |_| ssr.0(moved_route.clone())} class="text-primary" { Icon icon={icon}() span class="btm-nav-label"({route.to_string()})}
    }
}
