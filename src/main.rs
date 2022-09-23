use clap::{Parser, Subcommand, ValueEnum};
use clap_derive::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[clap(name = "fall out")]
#[clap(author = "uselessgoddess <dodickgod@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Вариант 3(13) Бабочкин Александр")]
struct Args {
    /// Name of file with data
    name: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
#[remain::sorted]
enum Precipitation {
    Rain,
    Sleet,
    Snow,
}

#[derive(Clone, ValueEnum)]
enum SortBy {
    Rainfall,
    Precipitation,
}

#[derive(Subcommand)]
enum Commands {
    /// Find all days with rain
    Rain {
        #[clap(arg_enum)]
        sort: SortBy,
    },

    /// Find all precipitation-free days
    Cleared {
        #[clap(arg_enum)]
        sort: SortBy,
    },
}

const CLEAR_RANK: f32 = 1.5;

fn main() {
    let args = Args::parse();
}
