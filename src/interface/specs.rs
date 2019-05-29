use colored::*;
use std::collections::HashMap;
use std::fmt;

use crate::datasource::Data;
use crate::scheduler::*;

#[derive(Debug)]
pub enum HttpMethod {
  Unknown,
  Get,
  Post,
}

pub struct Scenario {
  pub options: Options,
  pub start: f64,
  pub upstreams: Vec<Upstream>,
  pub schedulers: Vec<Scheduler>,
  pub datasources: HashMap<String, Data>,
}

pub struct Options {
  pub timeout: u64,
}

impl Default for Options {
  fn default() -> Self {
    Self { timeout: 5 }
  }
}

impl fmt::Display for Scenario {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "{}", "OPTIONS".blue().bold())?;
    writeln!(f, "  {} {}", "timeout:".dimmed(), format!("{}s", self.options.timeout).bold())?;
    writeln!(f)?;

    writeln!(f, "{}", "SCHEDULERS:".blue().bold())?;
    for scheduler in &self.schedulers {
      writeln!(f, "{:#}", scheduler)?;
    }

    writeln!(f, "{}", "UPSTREAMS:".blue().bold())?;
    for upstream in &self.upstreams {
      write!(f, "{:#}", upstream)?;
    }

    writeln!(f, "{}", "DATASOURCES:".blue().bold())?;
    for (name, data) in &self.datasources {
      writeln!(f, "  - {}: {} entries", name.bold(), data.len())?;
    }

    Ok(())
  }
}

#[derive(Debug)]
pub struct Upstream {
  pub method: HttpMethod,
  pub url: String,
  pub headers: HashMap<String, String>,
  pub basic: Option<UpstreamBasicAuth>,
  pub body: Option<String>,
}

impl fmt::Display for Upstream {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "  - {:?} {}", self.method, self.url)?;

    if !self.headers.is_empty() {
      writeln!(f, "    Headers:")?;
      for (key, value) in &self.headers {
        writeln!(f, "      - {} = {}", key.bold(), value)?;
      }
    }

    Ok(())
  }
}

#[derive(Debug)]
pub struct UpstreamBasicAuth {
  pub username: String,
  pub password: String,
}
