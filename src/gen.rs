use std::{io::{BufWriter, Write, self}, fs::File};

use chrono::NaiveDateTime;
use rand::{Rng, rngs::ThreadRng};
use regex::Regex;

#[derive(Debug)]
pub enum GenType {
    Int(i32, i32),
    String(u32, u32),
    Double(f64, f64),
    Date,
    Time,
    DateTime,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRangeError,
    InvalidSyntaxError,
    ParseIntError,
}

/// Create a random CSV given the command line args and the column type list
pub fn generate_random_csv(writer: File, row_count: u32, columns: Vec<GenType>) -> Result<(), io::Error> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            !?";
    let mut buf_writer = BufWriter::new(writer);
    let mut rng = rand::thread_rng();
    let column_count = columns.len();

    for _ in 1..=row_count {
        for (index, column_type) in columns.iter().enumerate() {
            match *column_type {
                GenType::Int(s, e) => write!(buf_writer, "{}", rng.gen_range(s..=e))?,
                GenType::String(s, e) => write!(buf_writer, "{}", (0..=rng.gen_range(s..=e)).map(|_| { 
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect::<String>())?,
                GenType::Double(s, e) => write!(buf_writer, "{}", rng.gen_range(s..=e))?,
                GenType::Date => write!(buf_writer, "{}", generate_random_date(&mut rng).format("%d/%m/%Y"))?,
                GenType::Time => write!(buf_writer, "{}", generate_random_date(&mut rng).format("%H:%M"))?,
                GenType::DateTime => write!(buf_writer, "{}", generate_random_date(&mut rng).format("%d/%m/%Y %H:%M"))?,
            };
            if index + 1 != column_count {
                write!(buf_writer, ",")?;
            }
            
        }
        writeln!(buf_writer)?;
    }
    Ok(())
}

#[inline]
fn generate_random_date(rng: &mut ThreadRng) -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(rng.gen::<u32>() as i64, 0).unwrap()
}

pub fn parse_args(columns: Vec<String>) -> Result<Vec<GenType>, ParseError> {
    let re = Regex::new(r"(d?t|[dils])(?:([+-]?(?:(?:0|[^\D0]\d*)(?:\.\d*)?|\.\d+)):([+-]?(?:(?:0|[^\D0]\d*)(?:\.\d*)?|\.\d+)))?").unwrap();

    let mut built_columns: Vec<GenType> = Vec::new();

    for dtype in columns {
        let Some(parsed) = re.captures(&dtype) else { return Err(ParseError::InvalidSyntaxError); };

        built_columns.push(match &parsed[1] {
            "s" => {
                if let (Some(start), Some(end)) = (parsed.get(2), parsed.get(3)) {
                    let start = start.as_str().parse::<u32>().map_err(|_| ParseError::ParseIntError)?;
                    let end = end.as_str().parse::<u32>().map_err(|_| ParseError::ParseIntError)?;
                    if start > end {
                        return Err(ParseError::InvalidRangeError);
                    }
                    GenType::String(start, end)
                } else {
                    GenType::String(1, 100)
                }
            },
            "i" => {
                if let (Some(start), Some(end)) = (parsed.get(2), parsed.get(3)) {
                    let start = start.as_str().parse::<i32>().unwrap();
                    let end = end.as_str().parse::<i32>().unwrap();
                    if start > end {
                        return Err(ParseError::InvalidRangeError);
                    }
                    GenType::Int(start, end)
                } else {
                    GenType::Int(0, 100)
                }
            },
            "l" => {
                if let (Some(start), Some(end)) = (parsed.get(2), parsed.get(3)) {
                    let start = start.as_str().parse::<f64>().unwrap();
                    let end = end.as_str().parse::<f64>().unwrap();
                    if start > end {
                        return Err(ParseError::InvalidRangeError);
                    }
                    GenType::Double(start, end)
                } else {
                    GenType::Double(0.0, 100.0)
                }
            },
            "d" => GenType::Date,
            "t" => GenType::Time,
            "dt" => GenType::DateTime,
            &_ => return Err(ParseError::InvalidSyntaxError)
        })
    }
    
    Ok(built_columns)
}


