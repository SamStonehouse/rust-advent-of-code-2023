use std::collections::HashMap;

use crate::inputs;

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    fn new(id: u32, winning_numbers: Vec<u32>, card_numbers: Vec<u32>) -> Card {
        Card {
            id,
            winning_numbers,
            card_numbers,
        }
    }

    fn from_line(line: &str) -> Card {
        let (id_str, game_str) = line.split_once(':').unwrap();
        let id_as_str = id_str.split_whitespace().last().unwrap();
        let id = id_as_str.parse::<u32>().unwrap();

        let (winning_numbers_str, card_numbers_str) = game_str.split_once('|').unwrap();
        let winning_numbers: Vec<u32> = winning_numbers_str
            .trim()
            .split_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect();

        let card_numbers: Vec<u32> = card_numbers_str
            .trim()
            .split_whitespace()
            .map(|num_str| num_str.trim().parse::<u32>().unwrap())
            .collect();

        Card {
            id,
            winning_numbers,
            card_numbers,
        }
    }

    fn get_winning_number_count(self: &Self) -> u32 {
        u32::try_from(
            self.card_numbers
                .iter()
                .filter(|num| self.winning_numbers.contains(num))
                .count(),
        )
        .unwrap()
    }

    fn get_score(self: &Self) -> u32 {
        let winning_number_cnt = self.get_winning_number_count();

        if winning_number_cnt == 0 {
            return 0;
        }

        let base: u32 = 2;
        base.pow(winning_number_cnt - 1)
    }
}

struct CardCopySet {
    cards: HashMap<u32, Card>,
    card_counts: HashMap<u32, u32>,
}

impl CardCopySet {
    fn create_card_map(cards: &Vec<Card>) -> HashMap<u32, Card> {
        let mut card_map = HashMap::new();
        cards.iter().for_each(|card| {
            card_map.insert(card.id, card.clone());
        });

        card_map
    }
    fn create_card_counts(cards: &Vec<Card>) -> HashMap<u32, u32> {
        let mut card_map = HashMap::new();
        cards.iter().for_each(|card| {
            card_map.insert(card.id, 1);
        });

        card_map
    }

    fn from_cards(cards: Vec<Card>) -> CardCopySet {
        let card_map = Self::create_card_map(&cards);
        let mut card_counts = Self::create_card_counts(&cards);

        let mut card_ids = card_map.keys().collect::<Vec<&u32>>();
        card_ids.sort();

        card_ids.iter().for_each(|id| {
            let current_card_count = *card_counts.get(id).unwrap();
            // For each winning number, iterate through the next set of cards and add the the current winning numbers to them
            for i in 0..card_map.get(id).unwrap().get_winning_number_count() {
                let key = **id + 1 + i;
                let val = *card_counts.get(&key).unwrap();
                card_counts.insert(key, val + current_card_count);
            }
        });

        CardCopySet {
            cards: card_map,
            card_counts,
        }
    }
}

pub(crate) fn part_one() {
    println!("Day Four, Part One");
    let lines = inputs::read_inputs_from_file("./inputs/day_four.txt").unwrap();
    let total_score: u32 = lines
        .iter()
        .map(|line| Card::from_line(line).get_score())
        .sum();
    println!("Total score for all cards: {}", total_score);
}

pub(crate) fn part_two() {
    println!("Day Four, Part Two");
    let lines = inputs::read_inputs_from_file("./inputs/day_four.txt").unwrap();
    let cards: Vec<Card> = lines.iter().map(|line| Card::from_line(line)).collect();
    let card_copy_set = CardCopySet::from_cards(cards);

    println!(
        "Total number of copies: {}",
        card_copy_set.card_counts.into_values().sum::<u32>()
    );
}
