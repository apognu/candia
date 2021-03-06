use colored::*;

use std::fmt;

use crate::scheduler::Schedulable;
use crate::util;

#[derive(Debug)]
pub struct Constant {
  pub upstreams: Vec<String>,
  pub duration: u64,
  pub count: u64,
  pub interval: u64,
}

impl fmt::Display for Constant {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "  - {} {}", "type:".dimmed(), "Constant".bold())?;
    writeln!(
      f,
      "    {} requests every {}s for {}s",
      self.count.to_string().bold(),
      self.interval.to_string().bold(),
      self.duration.to_string().bold()
    )?;

    if !self.upstreams.is_empty() {
      writeln!(f, "    {} {}", "upstreams:".dimmed(), self.upstreams.join(", "))?;
    }

    Ok(())
  }
}

impl Schedulable for Constant {
  fn schedule(&self, start: f64) -> Option<(u64, u64, Vec<String>)> {
    if util::elapsed_since(start) >= self.duration {
      None
    } else {
      Some((self.count, self.interval, self.upstreams.clone()))
    }
  }
}
