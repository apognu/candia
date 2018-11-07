use std::collections::HashMap;
use std::io::{self, Write};
use std::process;

use chrono::prelude::*;
use colored::*;
use rand::{self, Rng};
use regex::Regex;

use datasource::Data;
use result::{Failure, Success};

pub fn current_epoch() -> f64 {
  Utc::now().timestamp() as f64
}

pub fn current_epoch_with_ms() -> f64 {
  let utc: DateTime<Utc> = Utc::now();

  let secs = utc.timestamp() as f64;
  let millis = f64::from(utc.timestamp_subsec_millis()) / 1000.0;

  secs + millis
}

pub fn current_epoch_ms() -> i64 {
  Utc::now().timestamp_millis()
}

pub fn write_flush(msg: &str) {
  print!("{}", msg);
  io::stdout().flush().unwrap();
}

pub fn elapsed_since(epoch: f64) -> u64 {
  let now = current_epoch() as u64;

  now - (epoch as u64)
}

pub fn print_kv(values: Vec<(&str, String)>) {
  let mut keys_length = 0;
  for (key, _) in &values {
    if key.len() > keys_length {
      keys_length = key.len();
    }
  }

  for (key, value) in values {
    println!("{key:>width$} {value}", key = key.bold(), width = keys_length, value = value);
  }
}

pub fn fatal(msg: &str) {
  eprintln!("{} {}", "FATAL:".red().bold(), msg);
  process::exit(1);
}

pub fn info(msg: &str) {
  println!("{} {}", "INFO:".blue().bold(), msg);
}

pub fn log_header() -> String {
  "Start offset,Request,State,Status code,Duration (ms)\n".to_owned()
}

pub fn log(result: &Result<Success, Failure>) -> String {
  match result {
    Ok(s) => format!("{},{},{},{},{}\n", s.start, s.request, "OK", s.code, s.duration),
    Err(f) => format!("{},{},{},{},{}\n", f.start, f.request, "KO", f.code, f.duration),
  }
}

pub fn interpolate(base: &str, datasources: &HashMap<String, Data>) -> String {
  let rgx = Regex::new(r"\{(?P<label>[a-z_]+)\}").unwrap();

  rgx.captures_iter(base).fold(String::from(base), |result, capture| {
    let pattern = capture.get(0).unwrap().as_str();
    let label = capture.name("label").unwrap().as_str();

    let value = match datasources.get(label) {
      Some(vec) => rand::thread_rng().choose(&vec),
      None => None,
    };

    match value {
      Some(value) => result.replacen(pattern, value, 1),
      None => result,
    }
  })
}
