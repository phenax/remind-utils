#![feature(iter_intersperse)]

use anyhow::*;
use std::fs::File;
use std::io::BufReader;

use ical2rem::ical2rem;

fn main() -> Result<()> {
  let buf = BufReader::new(File::open("./basic.ics").unwrap());

  let reader = ical::IcalParser::new(buf);

  let events: Vec<Result<String>> = reader
    .flat_map(|calendar| ical2rem(calendar.expect("No cal")))
    .collect();

  let output_format = events
    .into_iter()
    .flatten()
    .intersperse("\n\n".to_string())
    .collect::<String>();

  println!("{output_format}");

  Ok(())
}
