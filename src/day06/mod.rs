use std::{fs, path::Path};
use itertools::Itertools;


const START_OF_MESSAGE_LENGTH: usize = 14;

pub fn run_part_1<P: AsRef<Path>>(filename: P) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    get_start_of_packet_marker_position(&input).unwrap() as u32
}


pub fn run_part_2<P: AsRef<Path>>(filename: P) -> u32 {
    let input = fs::read_to_string(filename).unwrap();
    get_start_of_message_marker_position(&input).unwrap() as u32
}


fn get_start_of_packet_marker_position(input: &str) -> Option<usize> {
    if let Some((i, (_, _, _, _))) = input.chars()
            .tuple_windows::<(_, _, _, _)>()
            .enumerate()
            .find(|(_, (a, b, c, d))| {
        a != b && a != c && a != d && b != c && b != d && c != d
    }) {
        Some(i + 4)
    } else {
        None
    }
}

// This function assumes that input contains only ASCII characters (one byte per char)
fn get_start_of_message_marker_position(input: &str) -> Option<usize> {
    let input = input.as_bytes();
    let mut position = 0;
    while (position + START_OF_MESSAGE_LENGTH) < input.len() {
        let chunk_to_test = &input[position..position+14];
        if let Some((i, _)) = find_repeated_characters(&chunk_to_test) {
            position += i + 1;
        } else {
            return Some(position + START_OF_MESSAGE_LENGTH);
        }
    }
    None
}


fn find_repeated_characters(input: &[u8]) -> Option<(usize, usize)> {
    assert!(input.len() == START_OF_MESSAGE_LENGTH);
    for i in 0..input.len() - 1 {
        for j in i+1..input.len() {
            if input[i] == input[j] {
                return Some((i, j));
            }
        }
    }
    None
}


#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        assert_eq!(super::get_start_of_packet_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(super::get_start_of_packet_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(super::get_start_of_packet_marker_position("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(super::get_start_of_packet_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(super::get_start_of_packet_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::get_start_of_message_marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(super::get_start_of_message_marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(super::get_start_of_message_marker_position("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(super::get_start_of_message_marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(super::get_start_of_message_marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}

