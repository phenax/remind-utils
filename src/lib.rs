#![feature(iter_intersperse)]

mod properties;

use std::collections::HashMap;

use anyhow::*;
use chrono::Duration;
use ical::parser::ical::component::{IcalCalendar, IcalTimeZoneTransitionType};

use properties::*;

pub fn ical2rem(cal: IcalCalendar) -> Vec<Result<String>> {
  let timezone_map = cal
    .timezones
    .iter()
    .filter_map(|tz| {
      let tzid = get_property_value(&tz.properties, "TZID");
      let offset = tz.transitions.iter().find_map(|t| {
        if let IcalTimeZoneTransitionType::STANDARD = t.transition {
          get_property_value(&t.properties, "TZOFFSETFROM")
        } else {
          None
        }
      });

      tzid.zip(offset)
    })
    .collect::<HashMap<String, String>>();

  cal
    .events
    .iter()
    .map(|event| {
      let summary = get_property_value(&event.properties, "SUMMARY").unwrap_or_default();
      let meet_link =
        get_property_value(&event.properties, "X-GOOGLE-CONFERENCE").unwrap_or_default();

      let dt_start = get_property_datetime(&event.properties, &timezone_map, "DTSTART")?;
      let dt_end = get_property_datetime(&event.properties, &timezone_map, "DTEND")?;

      let rules = get_property_recurrance(&event.properties);

      let date = if let Some(rule) = rules {
        format!("{rule} FROM {}", dt_start.format("%d %b %Y AT %H:%M"))
      } else {
        dt_start.format("%d %b %Y AT %H:%M").to_string()
      };

      let end_date = if dt_end - dt_start >= Duration::days(1) {
        format!("UNTIL {}", dt_end.format("%d %b %Y"))
      } else {
        format!("DURATION {}", (dt_end - dt_start).num_minutes())
      };

      Ok(
        format!("REM {date} {end_date} MSG {summary} {meet_link}")
          .trim_end()
          .to_string(),
      )
    })
    .collect()
}
