mod ea;

extern crate dotenv;

use chrono::{DateTime, Duration, Timelike, Utc};
use dotenv::dotenv;

use ea::EAsistent;
use google_calendar3::{
    api::{Event, EventDateTime},
    oauth2, CalendarHub,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Timetable {
    events: Vec<TimetableEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimetableEvent {
    date: String,
    from: String,
    to: String,
    title: String,
    classroom: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut ea = EAsistent::new();

    ea.login(
        env::var("USERNAME").expect("USERNAME not set").as_str(),
        env::var("PASSWORD").expect("PASSWORD not set").as_str(),
    )
    .await?;

    let from = strip_time(Utc::now(), false).unwrap();
    let to = strip_time(Utc::now(), true).unwrap() + Duration::days(14);

    let from_date_string = from.date().format("%Y-%m-%d").to_string();
    let to_date_string = to.date().format("%Y-%m-%d").to_string();

    let timetable = ea.get_timetable(&from_date_string, &to_date_string).await?;

    let service_account_key = oauth2::read_service_account_key("credentials.json").await?;

    let auth = oauth2::ServiceAccountAuthenticator::builder(service_account_key)
        .build()
        .await?;

    let hub = CalendarHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    let calendar_id = env::var("CALENDAR_ID").expect("CALENDAR_ID not set");

    let mut existing_events = hub
        .events()
        .list(calendar_id.as_str())
        .time_min(&from.to_rfc3339())
        .time_max(&to.to_rfc3339())
        .doit()
        .await?
        .1
        .items
        .expect("Failed to fetch existing events from calendar");

    for ea_event in timetable.events {
        let start_time = DateTime::parse_from_rfc3339(
            format!("{}T{}:00+02:00", ea_event.date, ea_event.from).as_str(),
        )
        .expect("Invalid ea event start time");

        if let Some(event_index) = existing_events.iter().position(|e| {
            let ex_title = e.summary.as_ref().unwrap();
            let ex_start_time = DateTime::parse_from_rfc3339(
                e.start
                    .as_ref()
                    .unwrap()
                    .date_time
                    .as_ref()
                    .unwrap()
                    .as_str(),
            )
            .expect("Invalid existing event start time");
            ex_start_time == start_time && ex_title.as_str() == ea_event.title
        }) {
            existing_events.remove(event_index);
            continue;
        }

        let event = Event {
            summary: Some(ea_event.title.clone()),
            location: Some(ea_event.classroom),
            start: Some(EventDateTime {
                date_time: Some(format!("{}T{}:00+02:00", ea_event.date, ea_event.from)),
                ..EventDateTime::default()
            }),
            end: Some(EventDateTime {
                date_time: Some(format!("{}T{}:00+02:00", ea_event.date, ea_event.to)),
                ..EventDateTime::default()
            }),
            ..Event::default()
        };

        hub.events()
            .insert(event, calendar_id.as_str())
            .doit()
            .await?;

        println!("Added {}", ea_event.title);
    }

    for ex_event in existing_events {
        hub.events()
            .delete(calendar_id.as_str(), ex_event.id.as_ref().unwrap())
            .doit()
            .await?;

        println!(
            "Removed {}",
            ex_event.summary.unwrap_or("unnamed event".to_string())
        );
    }

    Ok(())
}

fn strip_time(date: DateTime<Utc>, end: bool) -> Option<DateTime<Utc>> {
    if !end {
        date.with_hour(0)?.with_minute(0)?.with_second(0)
    } else {
        date.with_hour(23)?.with_minute(59)?.with_second(59)
    }
}
