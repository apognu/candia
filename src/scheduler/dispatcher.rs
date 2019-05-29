use std::sync::{mpsc::Sender, Arc};
use std::thread;
use std::time::Duration;

use rand::{random, thread_rng, Rng};
use reqwest::Client;

use crate::result::{Failure, State, Success};
use crate::specs::{HttpMethod::*, Upstream};
use crate::{config, result, scheduler::*, specs, util};

pub fn tick<'a>(options: &'a Arc<config::Options>, scenario: &Arc<specs::Scenario>, scheduler: &Scheduler, start: f64, tx: &Sender<Result<Success, Failure>>) -> result::State {
  // How much time passed since this scheduler was created?
  let elapsed = util::current_epoch() - start;
  let mut threads = vec![];

  // According to the current scheduler, how many requests should be spacned for the curent tick?
  // Each schedulers must return (count, interval) to defined this.
  let threshold = match scheduler {
    Scheduler::Constant(s) => s.schedule(start),
    Scheduler::SteppedConstant(s) => s.schedule(start),
    Scheduler::DoubleEvery(s) => s.schedule(start),
    Scheduler::RampUp(s) => s.schedule(start),
    Scheduler::Pause(s) => s.schedule(start),
  };

  // If requests must be spawned
  if let Some((count, interval, upstreams)) = threshold {
    if interval > 0 && (elapsed as u64) % interval == 0 {
      if options.verbose {
        println!();
      }

      util::info(&format!("running batch with {} requests over {} seconds...", count, interval));

      let upstreams = Arc::new(upstreams);

      // Spawn a thread for each request to be sent
      for _ in 0..count {
        let scenario = Arc::clone(&scenario);
        let tx = Sender::clone(&tx);
        let options = Arc::clone(&options);
        let upstreams = Arc::clone(&upstreams);

        let thread = thread::spawn(move || {
          // Sleep for a random period of the current interval to distribute the requests
          thread::sleep(Duration::from_millis(random::<u64>() % (interval * 1000)));

          let upstreams: Vec<&Upstream> = if upstreams.len() > 0 {
            scenario.upstreams.iter().filter(|u| upstreams.contains(&u.name)).collect()
          } else {
            scenario.upstreams.iter().collect()
          };

          // Pick a random request to be sent
          if let Some(req) = thread_rng().choose(&upstreams) {
            tx.send(request(&options, &scenario, req)).unwrap();
          }
        });

        threads.push(thread);
      }
    }

    return State::Continue;
  }

  // If there are no other requests to be sent for this scheduler, stop it
  State::Stop
}

pub fn request(options: &Arc<config::Options>, scenario: &Arc<specs::Scenario>, req: &specs::Upstream) -> Result<Success, Failure> {
  let duration = util::current_epoch_ms();
  let offset = util::elapsed_since(scenario.start);

  let client = Client::builder().timeout(Duration::from_secs(scenario.options.timeout)).build().unwrap();
  let url = util::interpolate(&req.url, &scenario.datasources);

  // TODO: add more methods
  let request = match req.method {
    Get => client.get(&url),
    Post => client.post(&url),
    _ => return Failure::global(offset, String::new(), 0, String::new()),
  };

  // Add headers
  let request = req
    .headers
    .iter()
    .fold(request, |r, (key, value)| r.header::<&str, &str>(&key, &util::interpolate(value, &scenario.datasources)));

  // Add Basic authentication
  let request = match req.basic {
    None => request,
    Some(ref basic) => request.basic_auth(&basic.username, Some(&basic.password)),
  };

  let request = match &req.body {
    None => request,
    Some(body) => request.body(util::interpolate(&body, &scenario.datasources)),
  };

  let request = request.build().unwrap();
  let request_desc = format!("{} {}", request.method(), request.url());

  match client.execute(request) {
    Ok(response) => {
      if options.verbose {
        util::write_flush("·");
      }

      let duration = util::current_epoch_ms() - duration;

      match response.status().as_u16() {
        200...399 => Success::new(offset, request_desc, duration, response.status().as_u16()),
        code => Failure::http(offset, request_desc, duration, code, String::new()),
      }
    }

    Err(err) => {
      if options.verbose {
        util::write_flush("!");
      }

      let duration = util::current_epoch_ms() - duration;

      Failure::global(offset, request_desc, duration, err.to_string())
    }
  }
}
