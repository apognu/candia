use colored::*;

use std::fmt;

use crate::scheduler::Schedulable;
use crate::util;

#[derive(Debug)]
pub struct Pause {
  pub duration: u64,
}

impl fmt::Display for Pause {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "  - {} {}", "type:".dimmed(), "Pause".bold())?;
    writeln!(f, "    pause for {}s", self.duration.to_string().bold(),)?;

    Ok(())
  }
}

impl Schedulable for Pause {
  fn schedule(&self, start: f64) -> Option<(u64, u64, Vec<String>)> {
    if util::elapsed_since(start) >= self.duration {
      None
    } else {
      Some((0, 1, vec![]))
    }
  }
}
