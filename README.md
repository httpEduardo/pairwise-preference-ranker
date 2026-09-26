# Pairwise Preference Ranker

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

**Turn pairwise choices into a simple ranked list.**

Pairwise Preference Ranker uses a Bradley–Terry model to estimate which options are preferred from winner-and-loser examples. It is useful for small experiments with plans, recommendations, or other alternatives.

## Quick start

```bash
cargo run -- --input matches.json
```

Adjust training with `--iterations`, `--lr`, and `--top`:

```bash
cargo run -- --input matches.json --iterations 200 --lr 0.05 --top 3
```

## Input format

```json
{
  "items": ["Plan A", "Plan B"],
  "preferences": [
    { "winner": "Plan A", "loser": "Plan B", "context": "lower cost" }
  ]
}
```

The output shows the estimated ranking and how often it agrees with the provided preferences. Results depend on the quality and coverage of the examples.

## License

[MIT](LICENSE)
