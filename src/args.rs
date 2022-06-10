use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct TimesessionsArgs {
    #[clap(short, long)]
    pub account: String,
    #[clap(short, long)]
    pub user_id: u32,
    #[clap(short, long)]
    pub period: String,
}
