# Smart Contract Audit AI Agent

![Logo](./logo.svg)

If using Shuttle:

```
cargo install cargo-shuttle
shuttle login
shuttle init
shuttle run
shuttle deploy
```

If not using Shuttle:

```console
cd audit-agent
cargo run
```

Environment Variables:

OPENAI_API_KEY

If using Shuttle, create Secrets.toml file in audit-agent folder and add:

```
OPENAI_API_KEY = "<your_api_key_here>"
```

If not using Shuttle, export OPENAI_API_KEY with value as your system environment variable.
