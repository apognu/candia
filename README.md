# candia

Candia is the longest siege in history. Here, it is a load-testing tool that allows to model your scenario as _initiated requests per interval_.

Candia does not allow you to craft complex funnels, following the browsing pattern of a user, with conditions and complex scenarii. Instead, it gives you tool to control how many requests are initiated for a given period of time. For this, currently, you can use four strategies:

 * **Constant requests** : for every period of time, candia will spawn X requests.
 * **Stepped constant requests** : same as **Constant request**, but allows you to specify several stages, increasing and decreasing the number of spawned requests in each stage.
 * **Double every period** : doubles the number of spawned requests for every period of time.
 * **Ramp up**: Linearly increases the number of spawned requests for a period of time, between two bounds.
 * **Pause**: Do not send any requests for a number of seconds.

For each spawned request, a random request specification is picked from the configured upstreams.

## Configuration

A scenario is represented as the following configuration file:

```
options:
  timeout: 5

schedulers:
  - type: SteppedConstant
    steps:
      - duration: 30
        count: 10
        interval: 2
      - duration: 30
        count: 20
        interval: 2
  - type: Pause
    args:
      duration: 5
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

For now, only ```GET``` and ```POST``` requests are supported.

As well as setting each request's timeout, the ```timeout``` setting defines how long the runner will wait when all schedulers are done to possibly wait for to-be-fulfilled requests. Preferably, we will, in the future, actually check if all requests have been fulfilled.

### Dynamic parameters

You have the possibility, for some configuration attributes, to replace substrings from random seed data taken from data sources. For now, you can interpolate data in **url strings**, **header values** and **bodies**. For retrieving data, there are two strategies for now: from text files (one value per line), and from static arrays defined in the configuration.

Every data source must be declared in a separate ```datasource``` section of the configuration, with a label which will be used in strings, and used as ```{label}```.

The following methods of retrieving data can be used:

 * ```array```: each item in the YAML array is mapped to a value in the pool.
 * ```file```: each line is mapped to a value in the pool.
 * ```directory```: each file in the directory is mapped to a value in the pool.

```
upstreams:
  - method: GET
    url: https://{domain}/user?user={users}
    headers:
      Authorization: {users}
  - method: POST
    url: https://{domain}/user
    bodies: "{bodies}"

datasources:
  bodies:
    type: directory
    source: /tmp/bodies
  domains:
    type: array
    data:
      - example.net
      - api.example.net
  users:
    type: file
    source: /tmp/users    
```

Those interpolators, in the future, will be usable in more locations and more data sources will be implemeted (such as from a directory of files).

## Check the configuration

```
$ candia check config.yml
OPTIONS:
  - timeout: 5s

SCHEDULERS:
  - type: RampUp
    ramp up requests every 1s from 1 to 100 for 15s
  - type: Pause
    pause for 10s
  - type: Constant
    100 requests every 5s for 10s

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
## View and process results

On top of the simple statistics returned by the ```run``` command, each run created at CSV log file, detailing the execution of each request, its name is ```candia-YYYY-MM-DDTHH:MM:SS.log```, and looks like this:

```
$ cat candia-2018-11-08T16:33:13.csv
Start offset,Request,State,Status code,Duration (ms)
1,GET http://127.1.1.1:8080/?user=user1,OK,200,8
1,GET http://127.0.0.1:8080/?user=user1,OK,200,28
1,GET http://127.0.0.1:8080/?user=user1,OK,200,9
2,GET http://127.1.1.1:8080/?user=user1,OK,200,21
2,GET http://127.0.0.1:8080/?user=user2,OK,200,25
2,GET http://127.0.0.1:8080/?user=user1,OK,200,14
2,GET http://127.0.0.1:8080/?user=user1,OK,200,24
3,GET http://127.1.1.1:8080/?user=user3,OK,200,14
3,GET http://127.1.1.1:8080/?user=user3,OK,200,24
```
