mod args;
mod date_utils;

use args::TimesessionsArgs;
use clap::Parser;
use date_utils::DateInfo;
use open::that as open_that;

fn main() {
    let args = TimesessionsArgs::parse();

    let date_info = DateInfo::default();

    let (from_date, to_date) = match args.period.as_ref() {
        "day" => (date_info.today, date_info.today),
        "week" => (date_info.monday, date_info.sunday),
        "month" => (date_info.start_of_month, date_info.end_of_month),
        "last_month" => (date_info.start_of_last_month, date_info.end_of_last_month),
        _ => (date_info.today, date_info.today), // Probably don't need the initial "day" case.
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
