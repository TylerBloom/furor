use std::collections::{HashMap, HashSet};

use furor::Ans;
use itertools::Itertools;
use rand::random;

fn gen_probs(symbols: &str) -> HashMap<char, f64> {
    symbols.chars().map(|c| (c, random())).collect()
}

fn spread(freqs: &HashMap<char, f64>, length: usize) -> String {
    let mut labeling = String::new();
    let total = freqs.values().sum::<f64>();
    let mut probs = freqs
        .iter()
        .map(|(c, freq)| (*c, total / freq))
        .collect::<HashMap<_, _>>();
    let mut charset = probs.keys().copied().collect::<HashSet<_>>();
    while labeling.len() < (length - charset.len()) {
        let min_c = *probs.iter().min_by(|a, b| a.1.total_cmp(b.1)).unwrap().0;
        labeling.push(min_c);
        if charset.contains(&min_c) {
            charset.remove(&min_c);
        }
        *probs.get_mut(&min_c).unwrap() += total / freqs[&min_c];
    }
    labeling.extend(charset);
    labeling
}

fn choices(source: &HashMap<char, f64>) -> impl Iterator<Item = char> {
    let mut running_total = 0f64;
    let mut weights = Vec::with_capacity(source.len());
    for (c, freq) in source {
        running_total += freq;
        weights.push((running_total, *c));
    }
    let last = *weights.last().unwrap();
    std::iter::repeat_with(move || {
        let r: f64 = random();
        weights.iter().find(|(w, _)| *w > r).unwrap_or(&last).1
    })
}

fn main() {
    let mut probs = gen_probs("abc");
    let total = probs.values().sum::<f64>();
    for p in probs.values_mut() {
        *p /= total;
    }

    let labeling = spread(&probs, 32);
    println!("Given probs: {:#?}", probs);
    println!("Generated labeling: {:?}", labeling);
    println!("Labeling probs: ");
    for c in labeling.chars().unique() {
        println!(
            "{c}: {}",
            labeling.chars().filter(|s| *s == c).count() as f64 / labeling.len() as f64
        );
    }

    let ans = Ans::new(labeling.chars().collect());
    let message: String = choices(&probs).take(40_000).collect();

    let information_content = -message.chars().map(|c| probs[&c].log2()).sum::<f64>();

    println!("Init encoder: {ans:#?}");

    let state = ans.encode(&message);
    let rough_len = state.iter_u32_digits().count() * 32;
    // Counts the number of zeros at the start of the integer
    let extra_len = state.iter_u32_digits().last().unwrap().reverse_bits().trailing_zeros() as usize;
    let ans_len = rough_len - extra_len;

    println!("Information content: {information_content}");
    println!("Code length: {ans_len}");

    let decoded = ans.decode(state);
    assert_eq!(decoded.len(), message.len(), "Mismatching lenghts! Expected: {}, Found: {}", message.len(), decoded.len());
    assert_eq!(decoded, message);
}
