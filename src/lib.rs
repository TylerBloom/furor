#![allow(dead_code)]

use std::{collections::HashMap, ops::AddAssign};

use num_bigint::BigUint;

#[derive(Debug)]
pub struct Ans {
    labeling: Vec<char>,
    block_size: usize,
    count_per_block: HashMap<char, usize>,
    count_before_index: Vec<usize>,
    symbol_table: HashMap<char, Vec<usize>>,
    initial_state: BigUint,
}

impl Ans {
    pub fn new(labeling: Vec<char>) -> Self {
        let block_size = labeling.len();
        let mut count_per_block = HashMap::new();
        let mut count_before_index = Vec::new();
        let mut symbol_table = HashMap::new();
        for (i, c) in labeling.iter().copied().enumerate() {
            symbol_table.entry(c).or_insert_with(|| {
                count_per_block.insert(c, 0);
                Vec::new()
            });
            count_before_index.push(*count_per_block.get(&c).unwrap());
            count_per_block.get_mut(&c).unwrap().add_assign(1);
            symbol_table.get_mut(&c).unwrap().push(i);
        }
        let initial_state = match labeling.as_slice() {
            [] => 0,
            [_] => 1,
            [first, rest @ ..] => rest.iter().take_while(|c| *c == first).count(),
        };
        Self {
            labeling,
            block_size,
            count_per_block,
            count_before_index,
            symbol_table,
            initial_state: initial_state.into(),
        }
    }

    /// Returns the `state + 1`th number labeled `symbol`
    fn c(&self, state: BigUint, symbol: char) -> BigUint {
        let state_plus_one = state.clone() + BigUint::new(vec![1]);
        let mut full_blocks = state_plus_one.clone() / self.count_per_block[&symbol];
        let mut symbols_left =
            isize::try_from(state_plus_one % self.count_per_block[&symbol]).unwrap() as usize;
        if symbols_left == 0 {
            full_blocks -= 1usize;
            symbols_left = self.count_per_block[&symbol];
        }

        // Count `symbols_left` symbols within the block
        let index_within_block = self.symbol_table[&symbol][symbols_left - 1];

        // if `block_size` is a power of 2, this multiplication is a bitshift
        full_blocks * self.block_size + index_within_block
    }

    /// Counts the number of numbers labeled `symbol` that are less than `state`
    fn d(&self, state: BigUint) -> (char, BigUint) {
        // if `block_size` is a power of 2, this is `state & (block_size - 1)`
        let index_within_block = isize::try_from(&state % self.block_size).unwrap() as usize;

        let symbol = self.labeling[index_within_block];

        // if `block_size` is a power of 2, this division is a bitshift
        let num_previous_blocks = &state / self.block_size;
        let count_before_block = self.count_per_block[&symbol] * num_previous_blocks;

        (
            symbol,
            count_before_block + self.count_before_index[index_within_block],
        )
    }

    pub fn encode(&self, msg: &str) -> BigUint {
        msg.chars()
            .rev()
            .fold(self.initial_state.clone(), |state, c| self.c(state, c))
    }

    pub fn decode(&self, mut state: BigUint) -> String {
        // TODO: We should be able to gauge what the output length will be by the size of the
        // input.
        let mut message = String::new();
        while state > self.initial_state {
            let (symbol, new_state) = self.d(state);
            state = new_state;
            message.push(symbol);
        }
        message
    }
}
