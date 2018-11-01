# candia

Candia is the longest siege in history. Here, it is a load-testing tool that allows to model your scenario as _initiated requests per interval_.

Candia does not allow you to craft complex funnels, following the browsing pattern of a user, with conditions and complex scenarii. Instead, it gives you tool to control how many requests are initiated for a given period of time. For this, currently, you can use four strategies:

 * **Constant requests** : for every period of time, candia will spawn X requests.
 * **Stepped constant requests** : same as **Constant request**, but allows you to specify several stages, increasing and decreasing the number of spawned requests in each stage.
 * **Double every period** : doubles the number of spawned requests for every period of time.
 * **Ramp up**: Linearly increases the number of spawned requests for a period of time, between two bounds.

For each spawned request, a random request specification is picked from the configured upstreams.

## Configuration

A scenario is represented as the following configuration file:

```
schedulers:
  - type: SteppedConstant
    steps:
      - duration: 30
        count: 10
        interval: 2
      - duration: 30
        count: 20
        interval: 2
  - type: RampUp
    args:
      duration: 60
      from: 10
      to: 100
      interval: 5

upstreams:
  - method: GET
    url: https://example.com/api/users/1
    headers:
      Accept: application/json
      Authorization: Bearer abcdefghijklmnopqrstuvwxyz
```

For now, only ```GET``` requests with fixed headers are supported. In the future, I may implement other methods, dynamic string params, headers and bodies.

## Check the configuration

```
$ candia check config.yml
SCHEDULERS:
 - type: RampUp
   ramp up requests every 1s from 1 to 100 for 15s

UPSTREAMS:
  - Get http://127.0.0.1:8080/
```

## Run the scenario

```
$ candia run config.yml
INFO: running batch with 1 requests over 1 seconds...
INFO: running batch with 7 requests over 1 seconds...
INFO: running batch with 14 requests over 1 seconds...
[...]
INFO: running batch with 86 requests over 1 seconds...
INFO: running batch with 93 requests over 1 seconds...
INFO: running batch with 100 requests over 1 seconds...

INFO: done.

STATISTICS:
  Requests count 801
  Success count: 801
    Error count: 0
   Success rate: 100.00%
           Mean: 17ms
      Std. dev.: 14ms
90th percentile: 34ms
95th percentile: 46ms
99th percentile: 69ms
```
