mod date;

use clap::{Parser, Subcommand, ValueEnum};
use std::{
    cmp::Ordering,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

// use anyhow::Result;
use crate::date::{Day, Month};
use tap::Pipe;

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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
#[remain::sorted]
enum Fallout {
    Clear,
    Rain,
    Sleet,
    Snow,
}

impl FromStr for Fallout {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as ValueEnum>::from_str(s, true)
    }
}

const CLEAR_RANK: f32 = 1.5;

#[derive(Debug)]
struct DayInfo {
    day: Day,
    month: Month,
    amount: f32,
    ty: Fallout,
}

impl DayInfo {
    fn parse<'a>(mut input: impl Iterator<Item = &'a str>) -> Option<Self> {
        let mut input = input.inspect(|x| println!("{x}"));
        Some(Self {
            day: input.next()?.parse::<u8>().ok()?.pipe(Day::from_u8)?,
            month: input.next()?.parse::<u8>().ok()?.pipe(Month::from_u8)?,
            amount: input.next()?.parse().ok()?,
            ty: input.next()?.parse().ok()?,
        })
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.total_cmp(&other.amount)
    }

    fn cmp_metadata(&self, other: &Self) -> Ordering {
        self.ty
            .cmp(&self.ty)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let buf: BufReader<_> = File::open(args.name)?.pipe(BufReader::new);

    let sort = &match &args.command {
        Commands::Rain { sort } => sort.clone(),
        Commands::Cleared { sort } => sort.clone(),
    };

    let mut infos: Vec<_> = buf
        .lines()
        .map(|line| -> Option<_> { DayInfo::parse(line.ok()?.split_whitespace()) })
        .map(Option::unwrap) // none is unreachable
        .filter(|info| {
            if let Commands::Rain { .. } = &args.command {
                info.ty == Fallout::Rain
            } else {
                info.amount <= CLEAR_RANK
            }
        })
        .collect();

    if let SortBy::Rainfall = sort {
        infos.sort_by(|a, b| a.amount.total_cmp(&b.amount))
    } else {
        infos.sort_by(DayInfo::cmp)
    }

    Ok(())
}
