//! Application state

use std::collections::BTreeMap;

use chrono::{DateTime, Local, NaiveDate, TimeZone};
use dioxus::prelude::{use_context, Scope};
use dioxus_signals::Signal;
use dirs::home_dir;
use sqlx::{sqlite::SqliteConnectOptions, FromRow, Pool, Sqlite, SqlitePool};

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
        let db_pool = &db_pool().await;
        // Create mood table if it does not exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mood (
                datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
                feeling_good INTEGER NOT NULL CHECK (feeling_good IN (0, 1))
            );",
        )
        .execute(db_pool)
        .await
        .unwrap();
        let moods = sqlx::query_as::<_, Mood>("SELECT datetime, feeling_good FROM mood")
            .fetch_all(db_pool)
            .await
            .unwrap();
        tracing::info!("Loaded {} mood entries", moods.len());
        self.moods.set(moods);
    }

    pub async fn add_mood(&self, feeling_good: bool) {
        tracing::info!("Adding mood: feeling_good={}", feeling_good);
        sqlx::query("INSERT INTO mood (feeling_good) VALUES (?)")
            .bind(feeling_good)
            .execute(&db_pool().await)
            .await
            .unwrap();
        self.initialize().await; // TODO: optimize this
    }

    pub fn use_state(cx: Scope) -> Self {
        *use_context(cx).unwrap()
    }
}

pub async fn db_pool() -> Pool<Sqlite> {
    let opts = db_opts();
    let pool = SqlitePool::connect_with(opts).await.unwrap();
    tracing::info!("Connected to database");
    pool
}

pub fn db_opts() -> SqliteConnectOptions {
    let home_dir = home_dir().expect("Could not find home directory");
    SqliteConnectOptions::default()
        .create_if_missing(true)
        .foreign_keys(true)
        .filename(home_dir.join(".felicity.db"))
}
