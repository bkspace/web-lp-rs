# WEB_LP_RS

This is a small demo, showing one possible way of using an existing linear programming solver in a rust web service. Thanks to the lp-modeler package, we could also easily swap the solver for another, but for now let's stick with GLPK.

## Running

To run this example you need to have GLPK installed. You can validate this by:

```glpsol --version```

If not, you can install with brew: ```brew install glpk```

Then run ```cargo build``` and execute the binary.

Or simply use the included Dockerfile:

```docker build -t web-lp-rs .```
```docker run -p 8080:8080 web-lp-rs```

HTTP GET `http://localhost:8080/api/optimise` will return a set of results if the problem is solveable:

```
{
    results: {
        B_E: 1,
        B_F: 0,
        A_E: 0,
        C_E: 0,
        A_D: 0,
        B_D: 0,
        C_F: 0,
        C_D: 1,
        A_F: 1
    }
}
```

