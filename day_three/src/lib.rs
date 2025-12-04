pub fn part1(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let mut maximum_digit = 0;
        let mut index = 0;
        let chars = line.chars().collect::<Vec<_>>();
        for i in 0..chars.len() {
            let char = chars[i];
            let num = char.to_digit(10).unwrap();
            if num > maximum_digit {
                maximum_digit = num;
                index = i;
            }
        }

        let result = if index == (chars.len() - 1) {
            let mut new_maximum = 0;
            for i in 0..(chars.len() - 1) {
                let char = chars[i];
                let num = char.to_digit(10).unwrap();
                if num > new_maximum {
                    new_maximum = num;
                }
            }

            new_maximum * 10 + maximum_digit
        } else {
            let mut second_maximum = 0;
            for i in (index + 1)..chars.len() {
                let char = chars[i];
                let num = char.to_digit(10).unwrap();
                if num > second_maximum {
                    second_maximum = num;
                }
            }

            maximum_digit * 10 + second_maximum
        };

        score += result;
    }

    score
}

#[test]
fn part1_() {
    panic!("{}", part1(include_str!("input")));
}

#[test]
fn part1_test() {
    assert_eq!(part1("987654321111111\n811111111111119\n234234234234278\n818181911112111"), 357);
}
