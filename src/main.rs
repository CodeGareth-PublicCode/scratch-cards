use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug, Default, Clone)]
pub struct ScratchCardGame {
    game_winners: HashSet<String>,
    player_selection: HashSet<String>,
    instances: usize,
}

impl ScratchCardGame {
    fn establish_winning_player_selection(&self) -> Vec<String> {
        let player_selection = self.player_selection.clone();
        let game_winning = self.game_winners.clone();

        let owned_vector = player_selection
            .intersection(&game_winning)
            .into_iter()
            .map(|element| String::from(element))
            .collect();
        owned_vector
    }

    fn increase_number_of_instances_by_1(&mut self) {
        self.instances += 1;
    }
}

fn main() {}

////////////////////////

fn part_1() {
    let file_path: &str = "./src/input.txt";
    let raw_content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let scratch_cards: Vec<ScratchCardGame> =
        raw_content.lines().map(|line| parse_line(line)).collect();

    let total_score = total_winning_score_of_multiple_scratch_cards(&scratch_cards);

    println!("Final score: {}", total_score)
}

////////////////////////

pub fn parse_line(input: &str) -> ScratchCardGame {
    let drop_card_ref_split: Vec<&str> = input.split(":").collect();
    let split: Vec<&str> = drop_card_ref_split[1].split("|").collect();

    let mut container: Vec<HashSet<String>> = vec![];

    for element in split {
        let formatted_split: HashSet<String> = element
            .split(" ")
            .filter(|element| element.chars().any(|char| char.is_numeric()))
            .map(|element| String::from(element))
            .collect();

        container.push(formatted_split)
    }

    ScratchCardGame {
        game_winners: container[0].to_owned(),
        player_selection: container[1].to_owned(),
        instances: 1,
    }
}

pub fn establish_score_based_on_winning_selection(input: &Vec<String>) -> i32 {
    let mut counter = 0;

    for (position, _) in input.iter().enumerate() {
        if position == 0 {
            counter += 1
        } else {
            counter *= 2
        }
    }

    counter
}

pub fn total_winning_score_of_multiple_scratch_cards(input: &Vec<ScratchCardGame>) -> i32 {
    let total_score: i32 = input
        .iter()
        .map(|scratch_card_game: &ScratchCardGame| {
            let winning_player_selection: Vec<String> =
                scratch_card_game.establish_winning_player_selection();
            establish_score_based_on_winning_selection(&winning_player_selection)
        })
        .sum();
    total_score
}

pub fn generate_card_store(input: Vec<&str>) -> HashMap<usize, ScratchCardGame> {
    let mut card_store: HashMap<usize, ScratchCardGame> = HashMap::<usize, ScratchCardGame>::new();

    for (position, &entry) in input.iter().enumerate() {
        card_store
            .entry((position + 1).to_owned())
            .or_insert(parse_line(&entry));
    }

    card_store
}

pub fn replicate_card_instance(input: &Vec<ScratchCardGame>) -> Vec<ScratchCardGame> {
    let mut card_instances: Vec<ScratchCardGame> = input.to_vec();

    if card_instances.len() < 1 {
        card_instances.extend_from_within(..card_instances.len());
    } else {
        card_instances.extend_from_within(..card_instances.len() - 1);
    };

    card_instances
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeInclusive;

    #[test]
    fn test_parse_line_into_scratch_card() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratch_card = parse_line(test_input);

        let winnings: HashSet<String> = HashSet::from([
            String::from("41"),
            String::from("48"),
            String::from("83"),
            String::from("86"),
            String::from("17"),
        ]);

        let entries: HashSet<String> = HashSet::from([
            String::from("83"),
            String::from("86"),
            String::from("6"),
            String::from("31"),
            String::from("17"),
            String::from("9"),
            String::from("48"),
            String::from("53"),
        ]);

        let expected_value: ScratchCardGame = ScratchCardGame {
            game_winners: winnings,
            player_selection: entries,
            instances: 1,
        };

        assert_eq!(expected_value, scratch_card);
    }

    #[test]
    fn test_find_intersection_between_two_elements() {
        let test_input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratch_card: ScratchCardGame = parse_line(test_input);
        let mut winning_player_selection: Vec<String> =
            scratch_card.establish_winning_player_selection();
        let mut expected = ["17", "48", "83", "86"];

        assert_eq!(winning_player_selection.sort(), expected.sort());
    }

    #[test]
    fn test_establish_doubling_score_pattern_for_multiple_winning_numbers() {
        let test_input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratch_card: ScratchCardGame = parse_line(test_input);
        let winning_player_selection: Vec<String> =
            scratch_card.establish_winning_player_selection();

        let score = establish_score_based_on_winning_selection(&winning_player_selection);

        assert_eq!(score, 8)
    }

    #[test]
    fn test_play_with_multiple_scratch_cards_and_establish_score() {
        let multiple_entries: Vec<&str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let scratch_cards: Vec<ScratchCardGame> = multiple_entries
            .iter()
            .map(|&entry| parse_line(&entry))
            .collect();

        let total_score = total_winning_score_of_multiple_scratch_cards(&scratch_cards);

        assert_eq!(total_score, 13)
    }

    #[test]
    fn test_parse_input_file_into_correct_shape() {
        let file_path: &str = "./src/input.txt";
        let raw_content: String =
            std::fs::read_to_string(file_path).expect("should read from file");
        let content = raw_content.lines().collect::<Vec<&str>>();
        assert_eq!(content.len(), 205);
    }

    #[test]
    fn test_increasing_instances_of_other_cards() {
        let multiple_entries: Vec<&str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let mut card_store = generate_card_store(multiple_entries);

        let mut tracker = 1;

        while tracker < 6 {
            let offset_counter: usize = tracker + 1;
            dbg!(offset_counter);
            let mut scratch_card_game = card_store
                .get_mut(&offset_counter)
                .unwrap()
                .clone()
                .to_owned();
            scratch_card_game.increase_number_of_instances_by_1();
            card_store.insert(offset_counter, scratch_card_game.to_owned());
            tracker += 1
        }

        assert_eq!(card_store.get(&1).unwrap().instances, 1);
        assert_eq!(card_store.get(&2).unwrap().instances, 2);
        assert_eq!(card_store.get(&3).unwrap().instances, 2);
        assert_eq!(card_store.get(&4).unwrap().instances, 2);
        assert_eq!(card_store.get(&5).unwrap().instances, 2);
        assert_eq!(card_store.get(&6).unwrap().instances, 2);
    }

    #[test]
    fn test_increasing_instances_of_other_cards_based_on_wins() {
        let multiple_entries: Vec<&str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        // Build a nice storage element so you can reference by game
        let mut card_store = generate_card_store(multiple_entries);

        // Iterate through in range to navigate multiple usages against card store
        for game_number in 1..=card_store.len() {
            // You need to establish the number of winners to figure out how many
            // cards ahead need to increase their instance number by 1
            let game = card_store.get(&game_number).unwrap();
            let number_of_winners = game.establish_winning_player_selection().len();

            // For any pre-existing instance occurrences, this drives the repeat process
            // for future cards
            for instances in 1..=game.instances {
                // Look ahead of the card you're at
                for game_card_ref in game_number + 1..=game_number + number_of_winners {
                    // Greedy pull the card to own it
                    let mut scratch_card_game = card_store
                        .get_mut(&game_card_ref)
                        .unwrap()
                        .clone()
                        .to_owned();

                    // Bump instances by 1, meaning next pass you will loop victories this much
                    scratch_card_game.increase_number_of_instances_by_1();

                    // Insert back into hashmap for later reference
                    card_store.insert(game_card_ref, scratch_card_game.to_owned());
                }
            }
        }

        assert_eq!(card_store.get(&1).unwrap().instances, 1);
        assert_eq!(card_store.get(&2).unwrap().instances, 2);
        assert_eq!(card_store.get(&3).unwrap().instances, 4);
        assert_eq!(card_store.get(&4).unwrap().instances, 8);
        assert_eq!(card_store.get(&5).unwrap().instances, 14);
        assert_eq!(card_store.get(&6).unwrap().instances, 1);
    }
}
