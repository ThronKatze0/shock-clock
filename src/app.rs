use leptos::*;
use leptos_mview::mview;

#[component]
pub fn App() -> impl IntoView {
    mview! {
        button class="btn btn-primary"("Wuhu")
    }
}
