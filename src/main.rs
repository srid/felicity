#![allow(non_snake_case)]
mod state;

use std::collections::BTreeMap;

use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_desktop::{LogicalSize, WindowBuilder};
use dioxus_router::prelude::*;
use state::{AppState, Mood};

fn main() {
    // Setup logging using tracing and tracing_subscriber
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Felicity");
    // launch the dioxus app in a webview
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
            .with_window(
                WindowBuilder::new()
                    .with_title("Felicity")
                    .with_inner_size(LogicalSize::new(400.0, 600.0)),
            ),
    );
}

#[derive(Routable, PartialEq, Debug, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
}

fn App(cx: Scope) -> Element {
    use_context_provider(cx, AppState::default);
    let state = AppState::use_state(cx);
    use_future(cx, (), |_| async move {
        state.initialize().await;
    });

    cx.render(rsx! { Router::<Route> {} })
}

fn Wrapper(cx: Scope) -> Element {
    render! {
        div { class: "container text-xl flex flex-col items-center justify-between h-screen",
            div { class: "m-auto p-4", Outlet::<Route> {} }
            footer { class: "mx-auto flex flex-row justify-center items-center w-full p-4 text-sm text-gray-400",
                "Powered by Dioxus "
                img { class: "w-4 h-4 self-center", src: "dioxus.png" }
            }
        }
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    let state = AppState::use_state(cx);
    let moods = state.moods.read();

    // Group moods by date
    // TODO: This should be a signal?
    let moods_by_date = Mood::group_by_day(&moods);

    // Render each date and its associated moods
    render! {
        MoodForm {}
        MoodGoal {}
        ViewMoods { moods_by_date: moods_by_date }
    }
}

#[component]
fn MoodForm(cx: Scope) -> Element {
    let state = AppState::use_state(cx);

    let handler = |mood| {
        move |_event| {
            cx.spawn(async move {
                state.add_mood(mood).await;
            });
        }
    };

    render! {
        div {
            class: "flex items-center justify-center space-x-4 mb-4 bg-blue-100 py-1 border-y-2 border-blue-700",
            title: "How am I experiencing this moment of being alive?",
            EmojiButton { emoji: "😊", handler: handler(true) }
            EmojiButton { emoji: "🥵", handler: handler(false) }
        }
    }
}

#[component]
fn MoodGoal(cx: Scope) -> Element {
    render! {
        div {
            class: "flex items-center justify-center italic text-gray-400 mb-2",
            title: "Current Goal",
            "🥅 achieve days of 100% feeling good 🥅"
        }
    }
}

#[component]
fn EmojiButton<F>(cx: Scope, emoji: &'static str, handler: F) -> Element
where
    F: Fn(Event<MouseData>),
{
    render! {
        button {
            class: "text-4xl hover:scale-110 hover:shadow-lg rounded-lg hover:bg-pink-200 active:bg-black transform transition duration-200 ease-in-out",
            onclick: handler,
            style: "animation: click 0.1s ease-in-out;",
            "{emoji}"
        }
    }
}

#[component]
fn ViewMoods(cx: Scope, moods_by_date: BTreeMap<NaiveDate, Vec<Mood>>) -> Element {
    render! {
        div { class: "flex flex-col space-y-2",
            for (date , moods) in moods_by_date.iter().rev() {
                div { class: "flex flex-row items-center space-x-2",
                    header { class: "font-mono text-xs", "{date}" }
                    MoodSummaryOnDay { moods: moods }
                }
            }
        }
    }
}

#[component]
fn MoodSummaryOnDay<'a>(cx: Scope, moods: &'a Vec<Mood>) -> Element {
    render! {
        div {
            for mood in moods.iter().rev() {
                span { class: "text-sm", title: "{mood.local_datetime()}",
                    if mood.feeling_good { "😊" } else { "🥵" }
                }
            }
        }
    }
}
