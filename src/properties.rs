use anyhow::*;
use chrono::{
  naive::{NaiveDate, NaiveDateTime},
  FixedOffset, TimeZone,
};
use ical::property::Property;
use std::{collections::HashMap, str::FromStr};

pub fn get_property<T>(
  properties: &[Property],
  name: &str,
  handler: impl Fn(Property) -> Option<T>,
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

pub fn add_timezone(datetime: NaiveDateTime, tz: &str) -> Result<NaiveDateTime> {
  if tz.is_empty() {
    return Ok(datetime);
  }

  let dt = FixedOffset::from_str(tz)
    .map_err(|e| anyhow!(e))
    .and_then(|tz| {
      tz.from_local_datetime(&datetime)
        .earliest()
        .ok_or(Error::msg("foobar"))
    });

  // TODO: Use machine timezone
  let ist = FixedOffset::from_str("+0530").unwrap();
  let dt_local = dt.map(|d| d.with_timezone(&ist).naive_local());

  Ok(dt_local.unwrap_or(datetime))
}

pub fn get_property_datetime(
  properties: &[Property],
  tzmap: &HashMap<String, String>,
  name: &str,
) -> Result<NaiveDateTime> {
  get_property(properties, name, |p| {
    let params = p
      .params
      .unwrap_or_default()
      .into_iter()
      .collect::<HashMap<String, Vec<String>>>();

    let timezone_name = params.get("TZID").unwrap_or(&vec![]).concat();
    let timezone = tzmap.get(&timezone_name).cloned().unwrap_or_default();
    // println!("{timezone} {:?}", p.value);

    Some(match &p.value {
      Some(d) if d.len() == 16 => NaiveDateTime::parse_from_str(d, "%Y%m%dT%H%M%SZ")
        .map_err(|e| anyhow!(e))
        .and_then(|d| add_timezone(d, &timezone)),
      Some(d) if d.len() == 15 => NaiveDateTime::parse_from_str(d, "%Y%m%dT%H%M%S")
        .map_err(|e| anyhow!(e))
        .and_then(|d| add_timezone(d, &timezone)),
      Some(d) if d.len() == 8 => NaiveDate::parse_from_str(d, "%Y%m%d")
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        .map_err(|e| anyhow!(e))
        .and_then(|d| add_timezone(d, &timezone)),
      _ => Err(Error::msg("Invalid date format")),
    })
  })
  .unwrap_or(Err(Error::msg("Unable to find property")))
}

pub fn get_property_recurrance(properties: &[Property]) -> Option<String> {
  get_property(properties, "RRULE", |p| parse_recurrance_rule(p.value))
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

pub fn parse_recurrance_rule(rule: Option<String>) -> Option<String> {
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
