use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct Preference {
    winner: String,
    loser: String,
    context: Option<String>,
}

#[derive(Deserialize)]
struct Dataset {
    items: Vec<String>,
    preferences: Vec<Preference>,
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn parse_arg(args: &[String], key: &str, default: &str) -> String {
    args.iter()
        .position(|arg| arg == key)
        .and_then(|idx| args.get(idx + 1))
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = parse_arg(&args, "--input", "matches.json");
    let iterations: usize = parse_arg(&args, "--iterations", "200")
        .parse()
        .unwrap_or(200);
    let lr: f64 = parse_arg(&args, "--lr", "0.05").parse().unwrap_or(0.05);
    let top: usize = parse_arg(&args, "--top", "0").parse().unwrap_or(0);

    let raw = fs::read_to_string(&input).expect("Failed to read input");
    let dataset: Dataset = serde_json::from_str(&raw).expect("Invalid JSON");

    let mut index = HashMap::new();
    for (i, item) in dataset.items.iter().enumerate() {
        index.insert(item.clone(), i);
    }

    let mut scores = vec![0.0_f64; dataset.items.len()];

    for _ in 0..iterations {
        for pref in &dataset.preferences {
            let winner_idx = *index
                .get(&pref.winner)
                .unwrap_or_else(|| panic!("Unknown item {}", pref.winner));
            let loser_idx = *index
                .get(&pref.loser)
                .unwrap_or_else(|| panic!("Unknown item {}", pref.loser));
            let diff = scores[winner_idx] - scores[loser_idx];
            let prob = sigmoid(diff);
            let grad = 1.0 - prob;
            scores[winner_idx] += lr * grad;
            scores[loser_idx] -= lr * grad;
        }
    }

    let mut correct = 0;
    for pref in &dataset.preferences {
        let winner_idx = index[&pref.winner];
        let loser_idx = index[&pref.loser];
        if scores[winner_idx] >= scores[loser_idx] {
            correct += 1;
        }
    }
    let accuracy = correct as f64 / dataset.preferences.len() as f64;

    let mut ranked: Vec<(String, f64)> = dataset
        .items
        .iter()
        .cloned()
        .zip(scores.into_iter())
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Iterations: {iterations}, lr: {lr}");
    println!("Preference accuracy: {:.1}%", accuracy * 100.0);
    println!("\nRanking:");

    let limit = if top == 0 || top > ranked.len() {
        ranked.len()
    } else {
        top
    };

    for (idx, (item, score)) in ranked.into_iter().take(limit).enumerate() {
        println!("{:>2}. {:<20} {:.3}", idx + 1, item, score);
    }
}
