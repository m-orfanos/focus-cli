use std::fmt::Write;
use std::thread;

use chrono::{DateTime, Duration, Local};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn main() {
    // configuration
    const SLEEP_TIME_IN_SECONDS: u64 = 1; // FIXME: change to at least 60 seconds

    let duration_in_seconds: Duration = Duration::seconds(15);
    let start: DateTime<Local> = Local::now();
    let end = start + duration_in_seconds;

    // build view
    let pb = ProgressBar::new(duration_in_seconds.num_seconds().try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {start} {title} {duration} [{wide_bar:.cyan/blue}] ({eta})",
        )
        .unwrap()
        .with_key("start", move |_state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{}", start.format("%Y-%m-%dT%H:%M").to_string()).unwrap()
        })
        // FIXME: do not hardcode title
        .with_key("title", |_state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{}", "Hello World").unwrap()
        })
        // FIXME: do not hardcode duration to seconds
        .with_key(
            "duration",
            move |_state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{}s", duration_in_seconds.num_seconds()).unwrap()
            },
        )
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    // main loop
    let mut now = start;
    while now < end {
        let elapsed = now - start;
        pb.set_position(elapsed.num_seconds().try_into().unwrap());
        thread::sleep(std::time::Duration::from_secs(SLEEP_TIME_IN_SECONDS));
        now = Local::now();
    }

    // end
    pb.finish();
}
