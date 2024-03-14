# fastingress
Lightning fast Kubernetes Ingress Controller powered by Hyper.

## Getting started
This library currently runs on a proxied Kubernetes cluster on port `8001`. This is not yet configurable, but is being worked on. To proxy your running Kubernetes cluster, run `kubectl proxy`. Then, make sure to create a pod and a service to expose it within the cluster. An example for this can be found in the `kubernetes/nginx-deployment.yml` file.

```bash
kubectl apply -f kubernetes/nginx-deployment.yml
```

Then, run the application. The application will now listen for incoming requests on port `3000`.

```bash
cargo run
```

To create an Ingress route, apply the corresponding Ingress resource, pointing to the just added service. The example for this can be found in the `kubernetes/sample-conf3.yml` file. Make sure to don't stop the running app.

```bash
kubectl apply -f kubernetes/sample-conf3.yml
```

Now, the application can be reached by navigating to `http://localhost:3000/baaas`, which will then proxy to the added service. In this case, that should result in a NGINX welcome page.



## debugging 

See [this link](https://code.visualstudio.com/docs/languages/rust#_debugging). See the `launch.json` example below:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug executable 'fastingress'",
      "cargo": {
        "args": ["build", "--bin=fastingress", "--package=fastingress"],
        "filter": {
          "name": "fastingress",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug unit tests in executable 'fastingress'",
      "cargo": {
        "args": [
          "test",
          "--no-run",
          "--bin=fastingress",
          "--package=fastingress"
        ],
        "filter": {
          "name": "fastingress",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## Development
It's not convinient to always rebuild and do a `cargo run` when introducing new changes. For that, we can use [cargo-watch](https://crates.io/crates/cargo-watch). Install it by `cargo install cargo-watch`.

Now, to run the project, use the following command.
```bash
cargo watch -x run
```

### Linting
Clean code is required. We can assure a part of this by linting. Use the `make lint` command to check on the rules. This is being enforced in the pipeline.
