fn main() {
    println!("Hello, world!");
}

pub fn parse_line(input: &str) -> Vec<Vec<&str>> {
    let drop_card_ref_split: Vec<&str> = input.split(":").collect();
    let split: Vec<&str> = drop_card_ref_split[1].split("|").collect();

    let mut container: Vec<Vec<&str>> = vec![];

    for element in split {
        let formatted_split: Vec<&str> = element
            .split(" ")
            .filter(|element| element.chars().any(|char| char.is_numeric()))
            .collect::<Vec<&str>>();
        container.push(formatted_split);
    };

    container

}

// pub fn calculate_points(input) {
//
// }

// pub fn find_union(part_a: &Vec<String>, part_b: &Vec<String>){
//     pass
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_line_into_two_workable_vectors() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let parsed_line = parse_line(test_input);

        let expected_value: Vec<Vec<&str>> = vec![
            vec!["41", "48", "83", "86", "17"],
            vec!["83", "86", "6", "31", "17", "9", "48", "53"],
        ];

        assert_eq!(expected_value, parsed_line);
    }
}
