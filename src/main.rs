use clap::{Parser, Subcommand, ValueEnum};
use std::{
    cmp::Ordering,
    error::Error,
    fmt::{Display, Formatter},
    fs::File,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

use tap::Pipe;

#[derive(Parser)]
#[clap(name = "fall out")]
#[clap(author = "uselessgoddess <dodickgod@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Вариант 3(13) Бабочкин Александр")]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    #[clap(long, arg_enum, default_value_t = SortBy::Amount)]
    sort: SortBy,
}

#[derive(Clone, ValueEnum)]
enum SortBy {
    Amount,
    Fallout,
}

#[derive(Subcommand)]
enum Commands {
    /// Find all days with rain
    Rain,

    /// Find all precipitation-free days
    Cleared,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
#[remain::sorted]
enum Fallout {
    Clear,
    Rain,
    Sleet,
    Snow,
}

impl Display for Fallout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Fallout::Clear => write!(f, "clear"),
            Fallout::Rain => write!(f, "rain"),
            Fallout::Sleet => write!(f, "sleet"),
            Fallout::Snow => write!(f, "snow"),
        }
    }
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
    day: u8,
    month: u8,
    amount: f32,
    ty: Fallout,
}

impl DayInfo {
    fn parse<'a>(mut input: impl Iterator<Item = &'a str>) -> Option<Self> {
        Some(Self {
            day: input.next()?.parse().ok()?,
            month: input.next()?.parse().ok()?,
            amount: input.next()?.parse().ok()?,
            ty: input.next()?.parse().ok()?,
        })
    }

    fn cmp(&self, other: &Self) -> Ordering {
        self.amount.total_cmp(&other.amount)
    }

    fn cmp_metadata(&self, other: &Self) -> Ordering {
        self.ty
            .cmp(&other.ty)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut infos: Vec<_> = stdin()
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

    if let SortBy::Amount = args.sort {
        infos.sort_by(DayInfo::cmp)
    } else {
        infos.sort_by(DayInfo::cmp_metadata)
    }

    for DayInfo {
        day,
        month,
        amount,
        ty,
    } in infos
    {
        println!(
            "|день: {day:2} месяц: {month:2} количество осадков: {amount:3} тип осадков: {ty}|"
        );
    }

    Ok(())
}
