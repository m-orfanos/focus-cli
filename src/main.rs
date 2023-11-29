use std::fmt::Write;
use std::thread;

use chrono::{DateTime, Duration, Local};
use clap::{arg, command};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

fn main() {
    // setup cli parser
    let cli = command!()
        .arg(arg!([title] "The name of the task to track").required(true))
        .get_matches();

    let title = cli.get_one::<String>("title").unwrap().to_string();

    // setup view
    let duration: Duration = Duration::seconds(15);
    let start: DateTime<Local> = Local::now();
    let end = start + duration;

    let pb = build_view(start, duration, title);

    // main loop
    let mut now = start;
    while now < end {
        let elapsed = now - start;
        pb.set_position(elapsed.num_seconds().try_into().unwrap());
        thread::sleep(std::time::Duration::from_secs(1)); // FIXME: change to at least 60 seconds
        now = Local::now();
    }
    pb.finish();
}

fn build_view(start: DateTime<Local>, duration: Duration, title: String) -> ProgressBar {
    let pb = ProgressBar::new(duration.num_seconds().try_into().unwrap());
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {start} {title} {duration} [{wide_bar:.cyan/blue}] ({eta})",
        )
        .unwrap()
        .with_key("start", move |_state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{}", start.format("%Y-%m-%dT%H:%M").to_string()).unwrap()
        })
        .with_key("title", move |_state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{}", title).unwrap()
        })
        // FIXME: do not hardcode duration to seconds
        .with_key(
            "duration",
            move |_state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{}s", duration.num_seconds()).unwrap()
            },
        )
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    pb
}
