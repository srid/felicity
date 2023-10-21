use std::collections::BTreeMap;

use chrono::{DateTime, Local, NaiveDate, TimeZone};

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
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

    // Group moods by date
    pub fn group_by_day(moods: &Vec<Mood>) -> BTreeMap<NaiveDate, Vec<Mood>> {
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
