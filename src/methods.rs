use crate::{DayInfo, Fallout};

pub fn amount_of(infos: &[DayInfo], month: u8) -> f32 {
    infos
        .iter()
        .filter(|&info| info.month == month)
        .map(|info| info.amount)
        .sum()
}

#[test]
fn basic_amount() {
    let infos = vec![
        DayInfo {
            day: 12,
            month: 1,
            amount: 1.5,
            ty: Fallout::Rain,
        },
        DayInfo {
            day: 11,
            month: 1,
            amount: 1.5,
            ty: Fallout::Rain,
        },
        DayInfo {
            day: 1,
            month: 2,
            amount: 1.5,
            ty: Fallout::Rain,
        },
    ];

    assert_eq!(3.0, amount_of(&infos, 1));
}

#[test]
fn empty_amount() {
    assert_eq!(0.0, amount_of(&[], 4));
}

#[test]
fn empty_month() {
    let infos = vec![
        DayInfo {
            day: 12,
            month: 1,
            amount: 1.5,
            ty: Fallout::Rain,
        },
        DayInfo {
            day: 11,
            month: 2,
            amount: 1.5,
            ty: Fallout::Rain,
        },
        DayInfo {
            day: 1,
            month: 3,
            amount: 1.5,
            ty: Fallout::Rain,
        },
    ];

    assert_eq!(0.0, amount_of(&infos, 4));
}
