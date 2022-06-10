use chrono::{Datelike, NaiveDate, Weekday};
use open::that as open_that;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO this is a terrible way to parse command line arguments. At the moment, both arguments
    // are optional and have defaults, but if argument 1 is omitted, argument 2 becomes argument 1.
    // This probably is better handled with a Config struct or something.
    let period = match args.len() {
        2 => &args[1],
        3 => &args[1],
        _ => "all", // This is a fake value; it will trigger the catchall for `period` matching.
    };
    let subdomain = match args.len() {
        3 => format!("{}.", &args[2]),
        _ => String::from("www."),
    };
    let user_id = env::var("TS_USER_ID").expect("Please set a user id environment variable");

    let this_year = chrono::offset::Local::now().year();
    let this_week = chrono::offset::Local::now().iso_week().week();
    let this_month = chrono::offset::Local::now().month();
    let this_day = chrono::offset::Local::today().day();

    let today = NaiveDate::from_ymd(this_year, this_month, this_day);
    let monday = NaiveDate::from_isoywd(this_year, this_week, Weekday::Mon);
    let sunday = NaiveDate::from_isoywd(this_year, this_week, Weekday::Sun);
    let start_of_month = NaiveDate::from_ymd(this_year, this_month, 1);
    let end_of_month = NaiveDate::from_ymd(
        this_year,
        this_month,
        get_days_in_month(this_year, this_month),
    );

    let (from_date, to_date) = match period.as_ref() {
        "day" => (today, today),
        "week" => (monday, sunday),
        "month" => (start_of_month, end_of_month),
        _ => (today, today), // Probably don't need the initial "day" case.
    };

    let url_base = format!("https://{subdomain}codebasehq.com/reports/time_tracking");
    let url_base_params = "?utf8=✓&time_sessions_filter[criteria][][column]=user&time_sessions_filter[criteria][][operator]=equal&time_sessions_filter[criteria][][data]=";
    let url_from_date_param = "&time_sessions_filter[criteria][][column]=occurred_on&time_sessions_filter[criteria][][operator]=greater-than-equal&time_sessions_filter[criteria][][data]=";
    let url_to_date_param = "&time_sessions_filter[criteria][][column]=occurred_on&time_sessions_filter[criteria][][operator]=less-than-equal&time_sessions_filter[criteria][][data]=";
    let url_suffix_params = "&commit=Filter+Time+Sessions";

    let path = format!(
        "{url_base}\
    {url_base_params}\
    {user_id}\
    {url_from_date_param}\
    {from_date}\
    {url_to_date_param}\
    {to_date}\
    {url_suffix_params}"
    );

    match open_that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("Something went wrong:\n{}", err),
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
