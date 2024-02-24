# fastingress
Lightning fast Kubernetes Ingress Controller powered by Hyper.


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
