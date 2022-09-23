# ea-sync

## Tool for syncing eAsistent timetable with google calendar

More CLI options comming soon. Currently it syncs 2 weeks from current day.

### Setup

- Copy `.env.example` to `.env`
- Enter your easistent `USERNAME` and `PASSWORD`
- Create a new google calendar and copy its id from settings to `CALENDAR_ID`
- Go to <https://console.cloud.google.com> and create a new project
- Enable google calendar API
- Create a new service account and add a new JSON key
- Save downloaded json file to `credentials.json` in the same folder as the executable file
- Copy the service account email and share the previously created calendar with it, allow adding new events
- Build the program with `cargo build --release`
- Run the program from `.env` and `credentials.json` directory `./target/release/ea-async`
