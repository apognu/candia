extern crate serde_yaml;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use datasource::{self, DataSource};
use interface::specs::{self, HttpMethod::*};
use scheduler::{strategies, *};
use util;

#[derive(Deserialize)]
pub struct Config {
  schedulers: Vec<ConfigScheduler>,
  upstreams: Vec<ConfigUpstream>,
  datasources: HashMap<String, ConfigDatasource>,
}

#[derive(Deserialize)]
struct ConfigScheduler {
  kind: String,
  #[serde(default)]
  args: HashMap<String, u64>,
  #[serde(default)]
  steps: Vec<HashMap<String, u64>>,
}

#[derive(Deserialize)]
struct ConfigUpstream {
  method: String,
  url: String,
  #[serde(default)]
  headers: HashMap<String, String>,
  #[serde(default)]
  basic: Option<ConfigUpstreamBasicAuth>,
  #[serde(default)]
  body: Option<String>,
}

#[derive(Deserialize)]
struct ConfigUpstreamBasicAuth {
  username: String,
  password: String,
}

#[derive(Deserialize)]
struct ConfigDatasource {
  kind: String,
  source: Option<String>,
  data: Option<Vec<String>>,
}

impl Config {
  pub fn read(file: &str) -> Result<Config, Box<dyn Error>> {
    let mut content = String::new();
    let mut file = File::open(file)?;

    file.read_to_string(&mut content)?;

    Ok(serde_yaml::from_str(&content)?)
  }

  pub fn create_scenario(&self) -> specs::Scenario {
    let mut scenario = specs::Scenario {
      start: util::current_epoch_with_ms(),
      schedulers: vec![],
      upstreams: vec![],
      datasources: HashMap::new(),
    };

    scenario.schedulers = self
      .schedulers
      .iter()
      .map(|scheduler| match scheduler.kind.as_ref() {
        "Constant" => Some(Scheduler::Constant(strategies::Constant {
          duration: *scheduler.args.get("duration").unwrap_or(&0),
          count: *scheduler.args.get("count").unwrap_or(&0),
          interval: *scheduler.args.get("interval").unwrap_or(&0),
        })),

        "SteppedConstant" => Some(Scheduler::SteppedConstant(strategies::SteppedConstant {
          steps: scheduler
            .steps
            .iter()
            .map(|step| strategies::Constant {
              duration: *step.get("duration").unwrap_or(&0),
              count: *step.get("count").unwrap_or(&0),
              interval: *step.get("interval").unwrap_or(&0),
            }).collect(),
        })),

        "DoubleEvery" => Some(Scheduler::DoubleEvery(strategies::DoubleEvery {
          duration: *scheduler.args.get("duration").unwrap_or(&0),
          period: *scheduler.args.get("period").unwrap_or(&0),
          count: *scheduler.args.get("count").unwrap_or(&0),
          interval: *scheduler.args.get("interval").unwrap_or(&0),
        })),

        "RampUp" => Some(Scheduler::RampUp(strategies::RampUp {
          duration: *scheduler.args.get("duration").unwrap_or(&0),
          interval: *scheduler.args.get("interval").unwrap_or(&0),
          from: *scheduler.args.get("from").unwrap_or(&0),
          to: *scheduler.args.get("to").unwrap_or(&0),
        })),
        unknown => {
          util::fatal(format!("unknown scheduler '{}'", unknown).as_ref());
          None
        }
      }).filter_map(|scheduler| scheduler)
      .collect();

    scenario.upstreams = self
      .upstreams
      .iter()
      .map(|upstream| specs::Upstream {
        method: match upstream.method.as_ref() {
          "GET" => Get,
          "POST" => Post,
          unknown => {
            util::fatal(format!("unknown HTTP method: {}", unknown).as_ref());
            Unknown
          }
        },
        url: upstream.url.to_owned(),
        headers: upstream.headers.to_owned(),
        basic: match upstream.basic {
          None => None,
          Some(ref basic) => Some(specs::UpstreamBasicAuth {
            username: basic.username.to_owned(),
            password: basic.password.to_owned(),
          }),
        },
        body: upstream.body.to_owned(),
      }).collect();

    scenario.datasources = self
      .datasources
      .iter()
      .map(|(name, datasource)| {
        let plugin: Vec<String> = match datasource {
          ConfigDatasource { ref kind, source: Some(source), .. } if kind == "file" => datasource::File::new(source).iter(),
          ConfigDatasource { ref kind, source: Some(source), .. } if kind == "directory" => datasource::Directory::new(source).iter(),
          ConfigDatasource { ref kind, data: Some(data), .. } if kind == "array" => datasource::Array::new(data).iter(),
          _ => vec![],
        };

        (name.to_owned(), plugin)
      }).collect();

    scenario
  }
}

#[derive(Debug)]
pub struct Options {
  pub verbose: bool,
  pub log: bool,
}

impl Options {
  fn new() -> Options {
    Options { verbose: false, log: true }
  }
}

pub fn parse_options(app: &clap::ArgMatches) -> Options {
  app.args.iter().fold(Options::new(), |mut options, arg| match arg {
    (&"verbose", _) => {
      options.verbose = true;
      options
    }

    (&"nolog", _) => {
      options.log = false;
      options
    }

    _ => options,
  })
}
