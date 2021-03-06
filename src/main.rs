#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate histogram;
extern crate rand;
extern crate reqwest;

mod config;
mod datasource;
mod interface;
mod scheduler;
mod util;

use chrono::prelude::*;
use clap::App;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{process, thread};

use crate::interface::result::{self, Failure, State, Success};
use crate::interface::specs;

fn main() {
  if let Err(error) = parse_cli() {
    println!("ERROR: {:#}", error);
  }
}

fn parse_cli() -> Result<(), Box<dyn Error>> {
  let yml = load_yaml!("cli.yml");
  let mut app = App::from_yaml(yml).version(crate_version!());

  let matches = { app.clone().get_matches() };
  let options = config::parse_options(&matches);

  match matches.subcommand() {
    ("run", Some(args)) => run(options, args),
    ("check", Some(args)) => check(&options, args),
    _ => usage(&mut app),
  }
}

fn run(options: config::Options, args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let config = config::Config::read(args.value_of("config").unwrap())?;
  let scenario = Arc::new(config.create_scenario());
  let timeout = scenario.options.timeout;
  let options = Arc::new(options);
  let results = Arc::new(Mutex::new(vec![]));
  let do_log = args.occurrences_of("disable_logging") == 0;

  let (tx, rx) = mpsc::channel::<Result<Success, Failure>>();

  // Spawn a receiver thread to compile the requests results
  {
    let results = results.clone();

    thread::spawn(move || {
      let log_name = Utc::now().format("candia-%Y-%m-%dT%H:%M:%S.csv").to_string();
      let mut log_file = if do_log {
        Some(File::create(log_name).expect("could not open log file for writing"))
      } else {
        None
      };

      if let Some(log_file) = &mut log_file {
        log_file.write_all(util::log_header().as_bytes()).unwrap();
      }

      for result in rx {
        if let Some(log_file) = &mut log_file {
          let log = util::log(&result);

          log_file.write_all(log.as_bytes()).unwrap();
        }

        results.lock().unwrap().push(result);
      }
    });
  }

  let duration = (scenario.schedulers.iter().map(|s| s.duration()).sum::<u64>()) + scenario.options.timeout;
  let mut step = 0;
  let pb = Arc::new(ProgressBar::new(duration));

  pb.enable_steady_tick(100);
  pb.set_style(
    ProgressStyle::default_bar()
      .template("{spinner:.green} [{elapsed_precise}] [{bar:50.white/blue}] {prefix} {msg}")
      .progress_chars("#> "),
  );

  let pbclone = Arc::clone(&pb);

  // Main loop iterating over the different configured schedulers
  let main = thread::spawn(move || {
    // Start running the curent scheduler, if there are no schedulers left, the scenario is finished
    for scheduler in &scenario.schedulers {
      step += 1;
      let start = util::current_epoch();

      // Send a tick every second, each schedulers will determine if requests have to be sent for that tick
      loop {
        pbclone.set_prefix(&format!("Step {}:", step));
        pbclone.set_position((util::current_epoch() - scenario.start) as u64);
        let result = scheduler::tick(&Arc::clone(&options), &Arc::clone(&scenario), scheduler, start, &Sender::clone(&tx), &pbclone);

        // A scheduler can tell us if it is finished or not, if it is, we skip to the next scheduler in line
        if let State::Stop = result {
          break;
        }

        // We send a tick every second for the schedule to be able to schedule its requests
        thread::sleep(Duration::from_secs(1));
      }
    }
  });

  main.join().unwrap();

  pb.set_prefix("Finalizing:");
  pb.set_message("waiting for timeout to expire");

  for _ in 0..timeout {
    pb.inc(1);
    thread::sleep(Duration::from_secs(1));
  }

  pb.finish_with_message("done");

  result::process(&results);

  Ok(())
}

fn check(_options: &config::Options, args: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
  let config = config::Config::read(args.value_of("config").unwrap())?;
  let scenario = config.create_scenario();

  print!("{:#}", scenario);

  Ok(())
}

fn usage(app: &mut App) -> ! {
  let _ = app.print_help();
  process::exit(1)
}
