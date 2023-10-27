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
        // Reverse each value in the BTreeMap
        for (_, moods) in moods_by_date.iter_mut() {
            moods.reverse();
        }
        moods_by_date
    }

    /// Tag each mood with a flag indicating if the next mood, if any, is the same feeling as this one.
    pub fn tag_consecutive_moods(moods: &[Mood]) -> Vec<(&Mood, bool)> {
        let mut tagged: Vec<(&Mood, bool)> = vec![];
        let mut iter = moods.iter().peekable();
        while let Some(mood) = iter.next() {
            let next_mood = iter.peek();
            tagged.push((
                mood,
                next_mood
                    .map(|next_mood| next_mood.feeling_good == mood.feeling_good)
                    .unwrap_or(false),
            ));
        }
        tagged
    }
}
