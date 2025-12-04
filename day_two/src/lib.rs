
pub fn part1(input: &str) -> u64 {
    let mut score = 0;
    for line in input.split(",") {
        let mut split = line.split("-");
        match (split.next(), split.next()) {
            (Some(min), Some(max)) => {
                match (min.parse::<u64>(), max.parse()) {
                    (Ok(min), Ok(max)) => {
                        for id in min..=max {
                            let id_string = id.to_string();
                            let length = id_string.len();
                            if length.is_multiple_of(2) {
                                let (left, right) = id_string.split_at(length / 2);
                                if left == right {
                                    score += id;
                                }
                            }
                        }
                    },
                    _ => unreachable!()        
                }
            },
            _ => unreachable!()
        }
    }

    score
} 

pub fn part2(input: &str) -> u64 {
    let mut score = 0;
    for line in input.split(",") {
        let mut split = line.split("-");
        match (split.next(), split.next()) {
            (Some(min), Some(max)) => {
                match (min.parse::<u64>(), max.parse()) {
                    (Ok(min), Ok(max)) => {
                        for id in min..=max {
                            // println!("Id: {id}");
                            let id_string = id.to_string();
                            let length = id_string.len();
                            let id_string_chars = id_string.chars().collect::<Vec<_>>();
                            let mut substring = String::new();
                            'search: for i in 0..(length/2) {
                                let char = id_string_chars[i];
                                substring.push(char);
                                let count = id_string.matches(&substring).count();
                                if count * substring.len() == length {
                                    // println!("Invalid: {id}");
                                    score += id;
                                    break 'search;
                                }
                            }
                        }
                    },
                    _ => unreachable!()        
                }
            },
            _ => unreachable!()
        }
    }

    score
} 

#[test]
fn foo() {
    let id_string = "123123".to_string();
    let (left, right) = id_string.split_at(3);
    panic!("{left}, {right}", );
}

#[test]
fn bar() {
    let id_string = "123123123".to_string();
    let substring = String::from("123");
    let count = id_string.matches(&substring).count();
    panic!("Count: {count}");
}

#[test]
fn bar2() {
    let id_string = "11112".to_string();
    let substring = String::from("1");
    let count = id_string.matches(&substring).count();
    panic!("Count: {count}");
}

#[test]
fn part1_() {
    panic!("{}", part1(include_str!("input")))
}

#[test]
fn part2_() {
    panic!("{}", part2(include_str!("input")))
}

#[test]
fn part2_test() {
    assert_eq!(part2("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), 4174379265);
}