use chrono::{DateTime, Duration, Timelike, Utc};
use ical::{generator::*, *};

use crate::{ea::EAsistent, Cli};

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long, env, default_value = "8080")]
    port: i32,
}

pub async fn execute(cli: &Cli, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("Serving ical on port {}", args.port);

    let mut ea = EAsistent::new();

    ea.login(cli.username.clone(), cli.password.clone()).await?;

    let from = strip_time(Utc::now(), false).unwrap();
    let to = strip_time(Utc::now(), true).unwrap() + Duration::days(30);

    let timetable = ea.get_timetable(from, to).await?;

    let mut cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//eAsistent//v1//SI")
        .build();

    for ea_event in timetable.events {
        let event = IcalEventBuilder::tzid("Europe/Ljubljana")
            .uid(&ea_event.title)
            .changed(chrono::Local::now().format("%Y%m%dT%H%M%S").to_string())
            .start(ea_event.from)
            .end(ea_event.to)
            .set(ical_property!("SUMMARY", &ea_event.title))
            .build();

        cal.events.push(event);
    }

    print!("{}", cal.generate());

    Ok(())
}

fn strip_time(date: DateTime<Utc>, end: bool) -> Option<DateTime<Utc>> {
    if !end {
        date.with_hour(0)?.with_minute(0)?.with_second(0)
    } else {
        date.with_hour(23)?.with_minute(59)?.with_second(59)
    }
}
