mod dispatcher;
pub mod strategies;

use std::fmt;

pub use self::dispatcher::*;

pub trait Schedulable {
  fn schedule(&self, start: f64) -> Option<(u64, u64, Vec<String>)>;
}

#[derive(Debug)]
pub enum Scheduler {
  Constant(strategies::Constant),
  SteppedConstant(strategies::SteppedConstant),
  DoubleEvery(strategies::DoubleEvery),
  RampUp(strategies::RampUp),
  Pause(strategies::Pause),
}

impl fmt::Display for Scheduler {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Scheduler::Constant(s) => write!(f, "{:#}", s),
      Scheduler::SteppedConstant(s) => write!(f, "{:#}", s),
      Scheduler::DoubleEvery(s) => write!(f, "{:#}", s),
      Scheduler::RampUp(s) => write!(f, "{:#}", s),
      Scheduler::Pause(s) => write!(f, "{:#}", s),
    }
  }
}
