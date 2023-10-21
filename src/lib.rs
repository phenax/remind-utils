use anyhow::*;
use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::Duration;
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

fn get_date_property(properties: &[Property], name: &str) -> Result<NaiveDateTime> {
  properties
    .iter()
    .find_map(|p| {
      if p.name == name.to_uppercase() {
        Some(match &p.value {
          Some(d) if d.len() == 16 => {
            NaiveDateTime::parse_from_str(d, "%Y%m%dT%H%M%SZ").map_err(|e| anyhow!(e))
          }
          Some(d) if d.len() == 8 => NaiveDate::parse_from_str(d, "%Y%m%d")
            .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
            .map_err(|e| anyhow!(e)),
          _ => Err(Error::msg("Hello worl")),
        })
      } else {
        None
      }
    })
    .ok_or(Error::msg("Helele"))?
}

pub fn ical2rem(cal: IcalCalendar) -> Vec<Result<String>> {
  cal
    .events
    .iter()
    .map(|event| {
      let summary = get_property(&event.properties, "SUMMARY");
      let meet_link = get_property(&event.properties, "X-GOOGLE-CONFERENCE");
      // let rules = get_property(&event.properties, "rrule");

      let dt_start = get_date_property(&event.properties, "DTSTART")?;
      let dt_end = get_date_property(&event.properties, "DTEND")?;

      let delta = if dt_end - dt_start >= Duration::days(1) {
        format!("UNTIL {}", dt_end.format("%d %b %Y"))
      } else {
        format!("DURATION {}:00", (dt_end - dt_start).num_minutes())
      };

      let date = dt_start.format("%d %b %Y AT %H:%M");

      Ok(
        format!("REM {date} {delta} MSG {summary} {meet_link}")
          .trim_end()
          .to_string(),
      )
    })
    .collect()
}
