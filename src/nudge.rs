use std::{rc::Rc, time::Duration};

use dioxus::prelude::*;
use dioxus_desktop::{tao::window::UserAttentionType, use_window, DesktopService};
use tokio::time;

use crate::state::AppState;

/// How often to register moods
static MOOD_FREQ: Duration = Duration::from_secs(60 * 10);

pub fn setup_nudge(cx: Scope) {
    let window = use_window(cx).clone();
    let state = AppState::use_state(cx);
    use_future(cx, (), |_| async move {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            if let Some(elapsed) = state.time_since_last_mood() {
                if elapsed > MOOD_FREQ {
                    tracing::info!("Time since last mood entry: {:?}", elapsed);
                    nudge(&window).await;
                }
            }
            interval.tick().await;
        }
    });
}

/// Causes the macOS dock icon to bounce (if the app is not in focus).
async fn nudge(window: &Rc<DesktopService>) {
    tracing::info!("Nudging!");
    window.request_user_attention(Some(UserAttentionType::Critical));
}
