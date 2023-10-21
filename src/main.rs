use anyhow::*;
use ical::parser::ical::component::IcalCalendar;
use ical::property::Property;
use std::fs::File;
use std::io::BufReader;

fn get_property(properties: &[Property], name: &str) -> String {
  properties
    .iter()
    .find_map(|p| {
      if p.name == name.to_uppercase() {
        p.value.to_owned()
      } else {
        None
      }
    })
    .unwrap_or_else(|| "".to_string())
}

fn ical2rem(cal: IcalCalendar) -> Result<()> {
  for event in cal.events {
    let summary = get_property(&event.properties, "summary");
    // let description = get_property(&event.properties, "description");
    let rules = get_property(&event.properties, "rrule");
    println!("{}: {}", summary, rules);
  }

  Ok(())
}

fn main() -> Result<()> {
  let buf = BufReader::new(File::open("./basic.ics").unwrap());

  let reader = ical::IcalParser::new(buf);

  for calendar in reader {
    ical2rem(calendar?)?;
  }

  Ok(())
}
