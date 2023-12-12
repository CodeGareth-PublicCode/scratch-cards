use std::collections::hash_map::RandomState;
use std::collections::hash_set::Intersection;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Default)]
pub struct ScratchCard {
    game_winners: HashSet<String>,
    player_selection: HashSet<String>,
}

fn main() {
    println!("Hello, world!");
}

pub fn parse_line(input: &str) -> ScratchCard {
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

    ScratchCard {
        game_winners: container[0].to_owned(),
        player_selection: container[1].to_owned(),
    }
}

pub fn establish_winning_player_selection(input: &ScratchCard) -> Vec<String> {
    let player_selection = input.player_selection.clone();
    let game_winning = input.game_winners.clone();

    let owned_vector = player_selection
        .intersection(&game_winning)
        .into_iter()
        .map(|element| String::from(element))
        .collect();
    owned_vector
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::RandomState;

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

        let expected_value: ScratchCard = ScratchCard {
            game_winners: winnings,
            player_selection: entries,
        };

        assert_eq!(expected_value, scratch_card);
    }

    #[test]
    fn test_find_union_between_two_elements() {
        let test_input: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratch_card: ScratchCard = parse_line(test_input);

        let mut winning_player_selection: Vec<String> =
            establish_winning_player_selection(&scratch_card);
        let mut expected = ["17", "48", "83", "86"];

        assert_eq!(winning_player_selection.sort(), expected.sort());
    }
}
