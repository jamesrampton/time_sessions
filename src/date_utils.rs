use chrono::{Datelike, NaiveDate, Weekday};

pub struct DateInfo {
    pub today: NaiveDate,
    pub monday: NaiveDate,
    pub sunday: NaiveDate,
    pub start_of_month: NaiveDate,
    pub end_of_month: NaiveDate,
}

impl DateInfo {
    pub fn new() -> DateInfo {
        let this_year = chrono::offset::Local::now().year();
        let this_month = chrono::offset::Local::now().month();
        let this_week = chrono::offset::Local::now().iso_week().week();
        let this_day = chrono::offset::Local::today().day();
        DateInfo {
            today: NaiveDate::from_ymd(this_year, this_month, this_day),
            monday: NaiveDate::from_isoywd(this_year, this_week, Weekday::Mon),
            sunday: NaiveDate::from_isoywd(this_year, this_week, Weekday::Sun),
            start_of_month: NaiveDate::from_ymd(this_year, this_month, 1),
            end_of_month: NaiveDate::from_ymd(
                this_year,
                this_month,
                get_days_in_month(this_year, this_month),
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
