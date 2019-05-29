use colored::*;

use std::fmt;

use crate::scheduler::{strategies, Schedulable};
use crate::util;

#[derive(Debug)]
pub struct SteppedConstant {
  pub steps: Vec<strategies::Constant>,
}

impl fmt::Display for SteppedConstant {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "  - {} {}", "type:".dimmed(), "SteppedConstant".bold())?;
    writeln!(f, "    {}", "steps:".dimmed())?;

    for (idx, step) in self.steps.iter().enumerate() {
      writeln!(
        f,
        "      {}. {} requests every {}s for {}s",
        idx + 1,
        step.count.to_string().bold(),
        step.interval.to_string().bold(),
        step.duration.to_string().bold()
      )?;
    }

    Ok(())
  }
}

impl Schedulable for SteppedConstant {
  fn schedule(&self, start: f64) -> Option<(u64, u64, Vec<String>)> {
    let elapsed = util::current_epoch() - start;
    let mut offset = 0;

    let threshold = self.steps.iter().find(|s| {
      let found = (elapsed as u64) < (s.duration + offset);

      offset += s.duration;
      found
    });

    match threshold {
      None => None,
      Some(s) => Some((s.count, s.interval, vec![])),
    }
  }
}
