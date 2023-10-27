//! Application state

use dioxus::prelude::{use_context, Scope};
use dioxus_signals::Signal;
use dirs::home_dir;
use rodio::{Decoder, OutputStream, Sink, Source};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

use crate::mood::Mood;

#[derive(Clone, Copy, Default)]
pub struct AppState {
    pub db_pool: Signal<Option<Pool<Sqlite>>>,
    pub moods: Signal<Vec<Mood>>,
}

impl AppState {
    pub async fn initialize(&self) {
        self.db_pool.set(Some(db_pool().await));
        self.load_moods().await;
        emit_sound().await;
    }

    pub async fn load_moods(&self) {
        let db_pool = &self.get_db_pool();

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
        let moods = sqlx::query_as::<_, Mood>(
            "SELECT datetime, feeling_good FROM mood ORDER BY datetime DESC;",
        )
        .fetch_all(db_pool)
        .await
        .unwrap();
        tracing::info!("Loaded {} mood entries", moods.len());
        self.moods.set(moods);
    }

    pub fn get_db_pool(&self) -> Pool<Sqlite> {
        self.db_pool.read().clone().unwrap()
    }

    pub async fn add_mood(&self, feeling_good: bool) {
        tracing::info!("Adding mood: feeling_good={}", feeling_good);
        let db_pool = &self.get_db_pool();
        sqlx::query("INSERT INTO mood (feeling_good) VALUES (?)")
            .bind(feeling_good)
            .execute(db_pool)
            .await
            .unwrap();
        self.load_moods().await; // TODO: optimize this
        emit_sound().await;
    }

    /// Return the [Duration] since the last mood entry was entered.
    pub fn time_since_last_mood(&self) -> Option<std::time::Duration> {
        self.moods.read().first().map(|mood| {
            chrono::Utc::now()
                .naive_utc()
                .signed_duration_since(mood.datetime)
                .to_std()
                .unwrap()
        })
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

async fn emit_sound() {
    tokio::task::spawn_blocking(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let my_slice = std::io::Cursor::new(include_bytes!("../assets/click.mp3").as_ref());
        let source = Decoder::new(my_slice).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    })
    .await
    .unwrap();
}
