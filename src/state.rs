//! Application state

use dioxus::prelude::{use_context, Scope};
use dioxus_signals::Signal;

use sqlx::{sqlite, FromRow, SqlitePool};

#[derive(Clone, Copy)]
pub struct AppState {
    pub moods: Signal<Vec<Mood>>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Mood {
    pub datetime: String,
    pub feeling_good: bool,
}

impl AppState {
    pub fn new() -> Self {
        let name = std::env::var("USER").unwrap_or("world".to_string());
        Self {
            moods: Signal::new(vec![]),
        }
    }

    pub async fn initialize(&self) {
        let pool = SqlitePool::connect("sqlite:/Users/srid/.dioxus-desktop-template.db")
            .await
            .unwrap();
        let moods = sqlx::query_as::<_, Mood>("SELECT datetime, feeling_good FROM mood")
            .fetch_all(&pool)
            .await
            .unwrap();
        for mood in &moods {
            println!("{:?}", mood);
        }
        self.moods.set(moods);
    }

    pub fn use_state(cx: Scope) -> Self {
        *use_context(cx).unwrap()
    }
}
