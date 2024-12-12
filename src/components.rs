use leptos::component;
use leptos::IntoView;
use leptos_mview::mview;

#[component]
pub fn Home() -> impl IntoView {
    mview! {
        div class="prose" {
            h1 class="my-4" { "Shock Clock" }
            div class="flex flex-col h-screen my-6" {
                div class="stats" {
                    div class="stat" {
                        span class="stat-title" { "Watcher" }
                    }
                    div class="stat" {
                        span class="stat-title" { "Clock" }
                    }
                }

                div class="flex flex-auto h-1/16 py-48" {
                    div class="flex-1";
                    button class="btn center text-6xl rounded-full flex-auto h-4/6 w-1/12 border-yellow-500 border-4" {"⚡"}
                    div class="flex-1";
                }
                div class="flex-1" {
                }
            }
        }
    }
}

#[component]
pub fn Watcher() -> impl IntoView {
    mview! {}
}

#[component]
pub fn Games() -> impl IntoView {
    mview! {}
}
