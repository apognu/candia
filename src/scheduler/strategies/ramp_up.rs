use colored::*;

use std::fmt;

use crate::scheduler::Schedulable;
use crate::util;

#[derive(Debug)]
pub struct RampUp {
  pub upstreams: Vec<String>,
  pub duration: u64,
  pub interval: u64,
  pub from: u64,
  pub to: u64,
}

impl fmt::Display for RampUp {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "  - {} {}", "type:".dimmed(), "RampUp".bold())?;
    writeln!(
      f,
      "    ramp up requests every {}s from {} to {} for {}s",
      self.interval.to_string().bold(),
      self.from.to_string().bold(),
      self.to.to_string().bold(),
      self.duration.to_string().bold()
    )?;

    Ok(())
  }
}

impl Schedulable for RampUp {
  fn schedule(&self, start: f64) -> Option<(u64, u64, Vec<String>)> {
    let elapsed = util::current_epoch() - start;

    if util::elapsed_since(start) >= self.duration {
      None
    } else {
      let laps = ((self.to - self.from) as f64 * (elapsed / self.duration as f64)) as u64 + self.from;

      Some((laps, self.interval, self.upstreams.clone()))
    }
  }
}
