use leptos::component;
use leptos::IntoView;
use leptos_mview::mview;

#[component]
pub fn Home() -> impl IntoView {
    mview! {
        div class="prose h-screen" {
            h1 class="my-4" { "Shock Clock" }
            div class="flex flex-col h-4/5 my-6" {
                div class="stats" {
                    div class="stat" {
                        span class="stat-title" { "Watcher" }
                    }
                    div class="stat" {
                        span class="stat-title" { "Clock" }
                    }
                }

                div class="flex flex-auto h-6/16 pt-48" {
                    div class="flex-1";
                    button class="btn center text-6xl rounded-full flex-auto h-4/6 w-1/12 border-yellow-500 border-4" {"⚡"}
                    div class="flex-1";
                }
                div class="flex-1 form-control" {
                    label class="label cursor-pointer" {
                        span class="label-text" { "Block Social Media" }
                        input type="checkbox" class="toggle";
                    }
                    label class="label cursor-pointer" {
                        span class="label-text" { "Block Shorts" }
                        input type="checkbox" class="toggle";
                    }
                    label class="label cursor-pointer" {
                        span class="label-text" { "Block Adult Content" }
                        input type="checkbox" class="toggle";
                    }
                }
            }
        }
    }
}

pub mod watcher;

pub use watcher::Watcher;

#[component]
pub fn Games() -> impl IntoView {
    mview! {}
}
