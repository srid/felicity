//! Application state

use std::collections::BTreeMap;

use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use dioxus::prelude::{use_context, Scope};
use dioxus_signals::Signal;

use sqlx::{FromRow, SqlitePool};

#[derive(Clone, Copy, Default)]
pub struct AppState {
    pub moods: Signal<Vec<Mood>>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct Mood {
    pub datetime: chrono::NaiveDateTime,
    pub feeling_good: bool,
}

impl Mood {
    pub fn date(&self) -> chrono::NaiveDate {
        self.local_datetime().date_naive()
    }

    pub fn local_datetime(&self) -> DateTime<Local> {
        Local.from_utc_datetime(&self.datetime)
    }

    pub fn group_by_day(moods: &Vec<Mood>) -> BTreeMap<NaiveDate, Vec<Mood>> {
        // Group moods by date

        let mut moods_by_date: BTreeMap<NaiveDate, Vec<Mood>> = BTreeMap::new();
        for mood in moods {
            moods_by_date
                .entry(mood.date())
                .or_default()
                .push(mood.clone());
        }
        moods_by_date
    }
}

impl AppState {
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

    pub async fn add_mood(&self, feeling_good: bool) {
        let pool = SqlitePool::connect("sqlite:/Users/srid/.dioxus-desktop-template.db")
            .await
            .unwrap();
        sqlx::query("INSERT INTO mood (datetime, feeling_good) VALUES (?, ?)")
            .bind(Utc::now())
            .bind(feeling_good)
            .execute(&pool)
            .await
            .unwrap();
        self.initialize().await; // TODO: optimize this
    }

    pub fn use_state(cx: Scope) -> Self {
        *use_context(cx).unwrap()
    }
}
