# fastingress

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