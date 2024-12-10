use shock_clock_ui::components::Home;
use std::fmt::Display;

use icondata as i;
use icondata_core::IconData;
use leptos::*;
use leptos_icons::*;
use leptos_mview::mview;

#[derive(Clone, PartialEq, Copy)]
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
struct Route(RwSignal<SelectedRoute>);

#[component]
pub fn App() -> impl IntoView {
    let selected_route = RwSignal::new(SelectedRoute::Home);
    provide_context(Route(selected_route));

    mview! {
        {move || selected_route().to_string()}
        // {move || match selected_route() {
        //     SelectedRoute::Home => mview! {Home;},
        //     _ => todo!()
        // }}
        div class="btm-nav btm-nav-sm" {
            BtmNavItem route={SelectedRoute::Home} icon={i::AiHomeOutlined}()
            BtmNavItem route={SelectedRoute::Devices} icon={i::BiDevicesRegular}()
            BtmNavItem route={SelectedRoute::Alarm} icon={i::ChClockAlarm}()
        }
    }
}

#[component]
fn BtmNavItem(route: SelectedRoute, icon: &'static IconData) -> impl IntoView {
    let ssr = use_context::<Route>().unwrap().0;
    let moved_route = route.clone(); // no idea why it wants two copies, with normal view! I only
                                     // need one
    mview! {
        button on:click={move |_| ssr.set(moved_route.clone())} class={move || format!("text-primary {}", if moved_route == ssr.get() {"active"} else {""})} {
            Icon icon={icon}()
            span class="btm-nav-label"({route.to_string()})
        }
    }
}
