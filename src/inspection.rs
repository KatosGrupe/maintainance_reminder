use chrono::prelude::*;
use chrono::Date;
use std::str::FromStr;

#[derive(Debug)]
pub struct InspectionDate {
    pub id: i64,
    pub repetition: Repetition,
    pub date: DateTime<FixedOffset>,
}

impl InspectionDate {
    pub fn next_time() -> Date<Utc> {
        todo!()
    }
}

#[derive(Debug)]
pub enum NextTimePivotPoint {
    Completion,
    DueDate,
}

#[derive(Debug)]
pub struct Repetition {
    next_time_pivot_point: NextTimePivotPoint,
    seconds: i32,
}

#[derive(Debug)]
pub enum ParseError {
    IncorrectRepetitionPivotChoice,
    IncorrectTimeCount,
    IncorrectTimeMultiplier,
    RegexNotMatched,
}

impl Repetition {
    pub fn from_string(value: &str) -> Result<Repetition, ParseError> {
        use regex::Regex;
        let re = Regex::new("([+]+)([0-9]+)([D|M|Y])").unwrap();

        match re.captures(value) {
            Some(cap) => {
                let count: i32 = match FromStr::from_str(&cap[2]) {
                    Ok(result) => result,
                    Err(_) => {
                        error!("({}) could not be converted into number, the program might not track correctly",
                               &cap[2]);
                        return Err(ParseError::IncorrectTimeCount);
                    }
                };
                let repetition = Repetition {
                    next_time_pivot_point: match &cap[1] {
                        "+" => NextTimePivotPoint::Completion,
                        "++" => NextTimePivotPoint::DueDate,
                        _ => {
                            error!(
                                "({}) could not be parsed, program might not track correctly",
                                &cap[1]
                            );
                            return Err(ParseError::IncorrectRepetitionPivotChoice);
                        }
                    },
                    seconds: match &cap[3] {
                        "Y" => count * 60 * 60 * 24 * 365,
                        "M" => count * 60 * 60 * 24 * 30,
                        "W" => count * 60 * 60 * 24 * 7,
                        "D" => count * 60 * 60 * 24,
                        "h" => count * 60 * 60,
                        "m" => count * 60,
                        "s" => count,
                        other => {
                            error!(
                                "({}) could not be parsed, acceptable values (Y, M, D, W, h, m, s)",
                                &cap[3]
                            );
                            return Err(ParseError::IncorrectTimeMultiplier);
                        }
                    },
                };
                return Ok(repetition);
            }
            None => {
                error!(
                    "({}) could not be parsed, program might not track correctly",
                    value
                );
                return Err(ParseError::RegexNotMatched);
            }
        }
    }
}
