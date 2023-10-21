use anyhow::*;
use chrono::naive::{NaiveDate, NaiveDateTime};
use ical::property::Property;
use std::collections::HashMap;

pub fn get_property<T>(
  properties: &[Property],
  name: &str,
  handler: fn(p: Property) -> Option<T>,
) -> Option<T> {
  properties.iter().find_map(|p| {
    if p.name == name.to_uppercase() {
      handler(p.to_owned())
    } else {
      None
    }
  })
}

pub fn get_property_value(properties: &[Property], name: &str) -> Option<String> {
  get_property(properties, name, |p| p.value)
}

pub fn get_property_datetime(properties: &[Property], name: &str) -> Result<NaiveDateTime> {
  get_property(properties, name, |p| {
    // TODO: Use TZID from params
    Some(match &p.value {
      Some(d) if d.len() == 16 => {
        NaiveDateTime::parse_from_str(d, "%Y%m%dT%H%M%SZ").map_err(|e| anyhow!(e))
      }
      Some(d) if d.len() == 15 => {
        NaiveDateTime::parse_from_str(d, "%Y%m%dT%H%M%S").map_err(|e| anyhow!(e))
      }
      Some(d) if d.len() == 8 => NaiveDate::parse_from_str(d, "%Y%m%d")
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        .map_err(|e| anyhow!(e)),
      _ => Err(Error::msg("Hello worl")),
    })
  })
  .unwrap_or(Err(Error::msg("Unable to find property")))
}

pub fn get_property_recurrance(properties: &[Property]) -> Option<String> {
  get_property(properties, "RRULE", |p| parse_rule(p.value))
}

pub fn to_day(d: &str) -> &str {
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
