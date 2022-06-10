mod args;
mod date_utils;

use args::TimesessionsArgs;
use chrono::{Datelike, NaiveDate, Weekday};
use clap::Parser;
use date_utils::get_days_in_month;
use open::that as open_that;

fn main() {
    let args = TimesessionsArgs::parse();

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

    let (from_date, to_date) = match args.period.as_ref() {
        "day" => (today, today),
        "week" => (monday, sunday),
        "month" => (start_of_month, end_of_month),
        _ => (today, today), // Probably don't need the initial "day" case.
    };

    let url_base = format!(
        "https://{}.codebasehq.com/reports/time_tracking",
        args.account
    );
    let url_base_params = format!("?utf8=✓&time_sessions_filter[criteria][][column]=user&time_sessions_filter[criteria][][operator]=equal&time_sessions_filter[criteria][][data]={}", args.user_id);
    let url_from_date_param = format!("&time_sessions_filter[criteria][][column]=occurred_on&time_sessions_filter[criteria][][operator]=greater-than-equal&time_sessions_filter[criteria][][data]={}", from_date);
    let url_to_date_param = format!("&time_sessions_filter[criteria][][column]=occurred_on&time_sessions_filter[criteria][][operator]=less-than-equal&time_sessions_filter[criteria][][data]={}", to_date);
    let url_suffix_params = "&commit=Filter+Time+Sessions";

    let path = format!(
        "{url_base}\
    {url_base_params}\
    {url_from_date_param}\
    {url_to_date_param}\
    {url_suffix_params}"
    );

    match open_that(path) {
        Ok(()) => (),
        Err(err) => eprintln!("Something went wrong:\n{}", err),
    }
}
