use leptos::*;
use leptos_mview::mview;
use leptos_icons::*;
use icondata as i;
use icondata_core::IconData;

#[derive(Clone)]
enum SelectedRoute {
    Home,
    Devices,
    Alarm
}

#[derive(Clone)]
struct RouteSetter(WriteSignal<SelectedRoute>);

#[component]
pub fn App() -> impl IntoView {
    let (selected_route, set_selected_route) = create_signal(SelectedRoute::Home);
    provide_context(RouteSetter(set_selected_route));

    mview! {
        {match selected_route() {
            SelectedRoute::Home => mview! {"Home"},
            SelectedRoute::Devices => mview! {"Devices"},
            SelectedRoute::Alarm => mview! {"Alarm"},
        }}
        div class="btm-nav" {
            BtmNavItem text={"Home"} icon={i::AiHomeOutlined}()
            BtmNavItem text={"Devices"} icon={i::BiDevicesRegular}()
            BtmNavItem text={"Alarm"} icon={i::ChClockAlarm}()
        }
    }
}

#[component]
fn BtmNavItem(text: &'static str, icon: &'static IconData) -> impl IntoView {
    let ssr = use_context::<RouteSetter>().unwrap();
    mview! {
        button onclick=[ssr.0] class="text-primary" { Icon icon={icon}() span class="btm-nav-label"({text})}
    }
}
