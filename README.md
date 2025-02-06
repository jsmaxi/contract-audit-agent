# Smart Contract Audit AI Agent

![Logo](./logo.svg)

AI-Powered Smart Contract Auditing: AI agents analyzing your smart contracts for vulnerabilities.

- Web Application: https://contract-audit-ui-production.up.railway.app/
- Agent code repotory: https://github.com/jsmaxi/contract-audit-agent
- UI code repository: https://github.com/jsmaxi/contract-audit-ui

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
