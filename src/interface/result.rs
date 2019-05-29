use colored::*;
use std::sync::{Arc, Mutex};

use crate::util;

pub enum State {
  Continue,
  Stop,
}

#[derive(Debug)]
pub struct Success {
  pub start: u64,
  pub request: String,
  pub duration: i64,
  pub code: u16,
}

impl Success {
  pub fn new(start: u64, request: String, duration: i64, code: u16) -> Result<Success, Failure> {
    Ok(Success { start, request, duration, code })
  }
}

pub struct Failure {
  pub start: u64,
  pub request: String,
  pub duration: i64,
  pub code: u16,
  pub reason: String,
}

impl Failure {
  pub fn http(start: u64, request: String, duration: i64, code: u16, reason: String) -> Result<Success, Failure> {
    Err(Failure {
      start,
      request,
      duration,
      code,
      reason,
    })
  }

  pub fn global(start: u64, request: String, duration: i64, reason: String) -> Result<Success, Failure> {
    Err(Failure {
      start,
      request,
      duration,
      code: 0,
      reason,
    })
  }
}

pub fn process(results: &Arc<Mutex<Vec<Result<Success, Failure>>>>) {
  let data = results.lock().unwrap();
  let success = data
    .iter()
    .filter(|x| x.is_ok())
    .map(|x| match x {
      Ok(Success { duration, .. }) => *duration,
      _ => 0,
    })
    .collect::<Vec<i64>>();

  println!();
  util::info("done.\n");

  let mut histogram = histogram::Histogram::new();

  for v in &success {
    if *v >= 0 {
      histogram.increment(*v as u64).unwrap();
    }
  }

  if histogram.entries() > 0 {
    let kv = vec![
      ("Requests count", format!("{}", data.len())),
      ("Success count:", format!("{}", success.len())),
      ("Error count:", format!("{}", data.len() - success.len())),
      ("Success rate:", format!("{:.2}%", format_rate(success.len(), data.len()))),
      ("Mean:", format_res_ms(histogram.mean())),
      ("Std. dev.:", format_opt_ms(histogram.stddev())),
      ("90th percentile:", format_res_ms(histogram.percentile(90.0))),
      ("95th percentile:", format_res_ms(histogram.percentile(95.0))),
      ("99th percentile:", format_res_ms(histogram.percentile(99.0))),
    ];

    println!("{}", "STATISTICS:".blue().bold());
    util::print_kv(kv);
  } else {
    util::info("no results");
  }
}

fn format_rate(value: usize, total: usize) -> f64 {
  value as f64 / total as f64 * 100.0
}

fn format_res_ms(value: Result<u64, &'static str>) -> String {
  match value {
    Ok(value) => {
      if value < 1000 {
        format!("{}ms", value)
      } else {
        format!("{}s", value as f64 / 1000.0)
      }
    }

    Err(_) => String::from("N/A"),
  }
}

fn format_opt_ms(value: Option<u64>) -> String {
  match value {
    Some(value) => {
      if value < 1000 {
        format!("{}ms", value)
      } else {
        format!("{}s", value as f64 / 1000.0)
      }
    }

    None => String::from("N/A"),
  }
}
