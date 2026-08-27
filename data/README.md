# Data

This directory is the bot's source of truth. It persists across restarts and deploys — do not treat it as disposable.

| Path | Role |
|------|------|
| `Secrets.toml` | Tokens and runtime config. Copy from `example.Secrets.toml`. Not committed. |
| `rustbot.sqlite` | SQLite database (scores, shop, messages, attachments metadata). Not committed. |
| `migrations/` | sqlx schema migrations. |
| `message_attachments/` | Downloaded Discord attachments. Not committed. |
