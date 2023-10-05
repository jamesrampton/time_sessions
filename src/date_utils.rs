use chrono::{Datelike, NaiveDate, Weekday};

pub struct DateInfo {
    pub today: NaiveDate,
    pub monday: NaiveDate,
    pub sunday: NaiveDate,
    pub start_of_month: NaiveDate,
    pub end_of_month: NaiveDate,
    pub start_of_last_month: NaiveDate,
    pub end_of_last_month: NaiveDate,
}

impl Default for DateInfo {
    fn default() -> Self {
        let now = chrono::offset::Local::now();
        let year = now.year();
        let month = now.month();
        let week = now.iso_week().week();
        let day = now.day();
        let (year_of_last_month, last_month) = match month {
            1 => (year - 1, 12),
            _ => (year, month - 1),
        };
        Self {
            today: NaiveDate::from_ymd(year, month, day),
            monday: NaiveDate::from_isoywd(year, week, Weekday::Mon),
            sunday: NaiveDate::from_isoywd(year, week, Weekday::Sun),
            start_of_month: NaiveDate::from_ymd(year, month, 1),
            end_of_month: NaiveDate::from_ymd(year, month, get_days_in_month(year, month)),
            start_of_last_month: NaiveDate::from_ymd(year_of_last_month, last_month, 1),
            end_of_last_month: NaiveDate::from_ymd(
                year_of_last_month,
                last_month,
                get_days_in_month(year_of_last_month, last_month),
            ),
        }
    }
}

fn get_days_in_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days() as u32
}
