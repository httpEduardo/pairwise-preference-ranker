# Alignment Atlas

Alignment Atlas turns preference pairs into a ranked list using a lightweight Bradley-Terry loop.

## Quick start

```bash
cargo run -- --input matches.json --iterations 200 --lr 0.05
```

## Input format

```json
{
  "items": ["Plan A", "Plan B"],
  "preferences": [
    {"winner": "Plan A", "loser": "Plan B", "context": "safer"}
  ]
}
```

## Output

The CLI prints ranked scores plus accuracy against the preference pairs.
