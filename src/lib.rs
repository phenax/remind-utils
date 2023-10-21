#![feature(iter_intersperse)]

use anyhow::*;
use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::Duration;
use ical::parser::ical::component::IcalCalendar;
use ical::property::Property;
use std::collections::HashMap;

fn get_property(properties: &[Property], name: &str) -> Option<String> {
  properties.iter().find_map(|p| {
    if p.name == name.to_uppercase() {
      p.value.to_owned()
    } else {
      None
    }
  })
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

fn to_day(d: &str) -> &str {
  match d {
    "MO" => "Mon",
    "TU" => "Tue",
    "WE" => "Wed",
    "TH" => "Thu",
    "FR" => "Fri",
    "SA" => "Sat",
    "SU" => "Sun",
    _ => "",
  }
}

pub fn parse_rule(rule: Option<String>) -> Option<String> {
  if let Some(rule) = rule {
    let rules = rule
      .split(';')
      .filter_map(|v| v.split_once('=').map(|(k, v)| (k, v.to_string())))
      .collect::<HashMap<&str, String>>();

    match rules.get("FREQ").map(|v| v.as_str()) {
      Some("WEEKLY") => Some(
        rules
          .get("BYDAY")
          .map(|v| v.to_string())
          .unwrap_or_default()
          .split(',')
          .map(to_day)
          .intersperse(" ")
          .collect::<String>(),
      ),
      Some("DAILY") => None,
      _ => None,
    }
  } else {
    None
  }
}

pub fn ical2rem(cal: IcalCalendar) -> Vec<Result<String>> {
  cal
    .events
    .iter()
    .map(|event| {
      let summary = get_property(&event.properties, "SUMMARY").unwrap_or_default();
      let meet_link = get_property(&event.properties, "X-GOOGLE-CONFERENCE").unwrap_or_default();
      let reccurance_rules = get_property(&event.properties, "RRULE");

      let dt_start = get_date_property(&event.properties, "DTSTART")?;
      let dt_end = get_date_property(&event.properties, "DTEND")?;

      let delta = if dt_end - dt_start >= Duration::days(1) {
        format!("UNTIL {}", dt_end.format("%d %b %Y"))
      } else {
        format!("DURATION {}", (dt_end - dt_start).num_minutes())
      };

      let rules = parse_rule(reccurance_rules);
      let date = if let Some(rule) = rules {
        format!("{rule} FROM {}", dt_start.format("%d %b %Y AT %H:%M"))
      } else {
        dt_start.format("%d %b %Y AT %H:%M").to_string()
      };

      Ok(
        format!("REM {date} {delta} MSG {summary} {meet_link}")
          .trim_end()
          .to_string(),
      )
    })
    .collect()
}
