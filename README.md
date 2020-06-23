### WEB_LP_RS

This is a small demo, showing one possible way of using an existing linear programming solver in a rust web service. Thanks to the lp-modeler package, we could also easily swap the solver for another, but for now let's stick with GLPK.

I haven't done any fine tuning, and just pulled across the example problem from lp-modeler.

```
» wrk   http://localhost:8080/api/optimise --latency -t 1  -c 10 -d 2s
Running 2s test @ http://localhost:8080/api/optimise
  1 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    17.52ms    4.64ms  34.67ms   66.92%
    Req/Sec   569.71     47.85   666.00     85.71%
  Latency Distribution
     50%   17.49ms
     75%   20.41ms
     90%   23.73ms
     99%   29.43ms
  1191 requests in 2.10s, 246.57KB read
Requests/sec:    567.08
Transfer/sec:    117.40KB
```

Those figures may or may not be suitable depending on the use case. Also the above does not include any TLS handshake/connection time which you would see in a production environment. There are existing issues open on lp-modeler with suggestions on how performance can be improved in the mid-term.
