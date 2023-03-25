# codewars

This repository contains my solutions to the [codewars](https://www.codewars.com/) challenges.
link to my profile: https://www.codewars.com/users/oskarissimus

## Rust
to run rust tests:
```sh
cd rust_solutions
cargo test
```

## Python
to run the tests, run test file with python, for example:
```sh
cd python_solutions
poetry install
poetry run python tests/kyu_8/basic_mathematical_operations_test.py
```

## JavaScript
to run javascript tests run mocha with the path to the file, for example:
```sh
cd js
npm install
npm test ./8-kyu/basic-mathematical-operations.js
```
useful configuration to debug js files
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "node",
            "request": "launch",
            "name": "debug js",
            "skipFiles": [
                "<node_internals>/**"
            ],
            "runtimeExecutable": "npm",
            "program": "${file}",
            "runtimeArgs": [
                "test"
            ],
            "cwd": "${workspaceFolder}/js"
        }
    ]
}
```

## TypeScript
```sh
cd typescript_solutions
npm test
```

