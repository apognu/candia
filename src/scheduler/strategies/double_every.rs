use colored::*;

use std::fmt;

use crate::scheduler::Schedulable;
use crate::util;

#[derive(Debug)]
pub struct DoubleEvery {
  pub upstreams: Vec<String>,
  pub duration: u64,
  pub period: u64,
  pub count: u64,
  pub interval: u64,
}

impl fmt::Display for DoubleEvery {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "  - {} {}", "type:".dimmed(), "DoubleEvery".bold())?;
    writeln!(
      f,
      "    {} requests every {}s for {}s, doubling every {}s",
      self.count.to_string().bold(),
      self.interval.to_string().bold(),
      self.duration.to_string().bold(),
      self.period.to_string().bold()
    )?;

    if !self.upstreams.is_empty() {
      writeln!(f, "    {} {}", "upstreams:".dimmed(), self.upstreams.join(", "))?;
    }

    Ok(())
  }
}

impl Schedulable for DoubleEvery {
  fn schedule(&self, start: f64) -> Option<(u64, u64, Vec<String>)> {
    let elapsed = util::current_epoch() - start;

    if util::elapsed_since(start) >= self.duration {
      None
    } else {
      let laps = 2u64.pow(elapsed as u32 / self.period as u32);

      Some((self.count * laps, self.interval, self.upstreams.clone()))
    }
  }
}
