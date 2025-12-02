[![Build Docker and Deploy](https://github.com/TimHillier/RustBot/actions/workflows/docker-build-push.yml/badge.svg)](https://github.com/TimHillier/RustBot/actions/workflows/docker-build-push.yml)

# RustBot

A Discord bot written in Rust that implements a community scoring and trading system. Users can earn points through reactions (+2/-2), trade points with each other, and purchase items from a shop using their accumulated points.

## Description

RustBot is a feature-rich Discord bot that gamifies community interaction through a scoring system. Users receive points when others react to their messages with +2 emojis, and lose points with -2 emojis. The bot tracks scores in a SQLite database and provides various commands for viewing leaderboards, trading points, and managing a virtual economy through a shop system.

## Libraries Used

- [**serenity** (0.12.4)](https://crates.io/crates/serenity) - Discord API library for Rust, providing client, gateway, and framework functionality
- [**poise** (0.6.1)](https://crates.io/crates/poise) - Discord command framework built on top of Serenity

## Commands

All commands use the `!` prefix.

### Scoring Commands

- **`!score`** - Shows the current score of the command caller. Can also be used as a reply to another user's message to see their score.
- **`!top`** - Displays the top scoring user.
- **`!leader`** (aliases: `!board`, `!leaderboard`, `!lb`) - Displays the top 10 scoring users.
- **`!wallet`** (aliases: `!balance`, `!bank`) - Shows the user's current balance of +2 emojis.

### Trading Commands

- **`!trade <user> <amount>`** - Trades the specified amount of +2 emojis from the command caller to the specified user. The trade affects both users' scores (giver loses 2 points per +2, receiver gains 2 points per +2). Validates that the caller has sufficient balance before executing.

### Shop Commands

- **`!shop`** (alias: `!store`) - Displays the shop with all available items, their prices, and descriptions.
  - **`!shop buy <symbol>`** - Purchases an item from the shop using the item's symbol. Validates that the user has enough +2 emojis to make the purchase.
- **`!item_count <symbol>`** (aliases: `!count`, `!getCount`) - Shows the current count of a specific shop item.

### Fun Commands

- **`!smash`** - Returns a random "Smash" or "Pass" response. Used as a reply to messages.
- **`!judge`** - Judges a post by adding a random +2 or -2 reaction. Must be used as a reply to another message. Prevents duplicate judgments on the same post. _Note: Marked for rework._

### Utility Commands

- **`!help [command]`** - Displays the help menu. If a command name is provided, shows detailed help for that specific command.

## Features

- **Reaction-based Scoring**: Automatically tracks scores when users react with +2 or -2 emojis
- **Database Persistence**: Uses SQLite to store user scores, trade logs, and shop data
- **Trade System**: Users can trade +2 emojis with each other, affecting their scores
- **Shop System**: Virtual economy where users can purchase items using accumulated +2 emojis
- **Leaderboards**: Track top performers in the community
- **Migration Support**: Database migrations managed through sqlx-cli

## Development

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality before commits. To set up:

1. Install pre-commit (requires Python):

   ```bash
   pip install pre-commit
   ```

2. Install the git hooks:

   ```bash
   pre-commit install
   ```

3. The hooks will now run automatically on every commit, checking:
   - **rustfmt**: Code formatting with `cargo fmt`
   - **clippy**: Linting with `cargo clippy`

To manually run the hooks:

```bash
pre-commit run --all-files
```

To skip hooks for a single commit (not recommended):

```bash
git commit --no-verify
```
