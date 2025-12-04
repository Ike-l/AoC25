pub fn find_total(input: &str) -> i64 {
    let mut total: i64 = 50;
    let mut score = 0;

    for line in input.lines() {
        let (dir, mag) = line.split_at(1);
        let mag: i64 = mag.parse().unwrap();

        let r = if dir == "R" { 1 } else { -1 };
        total += r * mag;

        if (total / 100) * 100 == total {
            score += 1;
        }
    }
    score
}

pub fn find_total_part2(input: &str) -> u64 {
    let mut total = 50;
    let mut score = 0;
    
    for line in input.lines() {
        let (dir, mag) = line.split_at(1);
        let mag: i64 = mag.parse().unwrap();

        let r = if dir == "R" { 1 } else { -1 };
        let difference = mag;

        for _ in 0..difference {
            total += r;

            if (total / 100) * 100 == total {
                score += 1;
            }
        }
    }

    score
}

#[test]
fn correct() {
    assert_eq!(find_total(include_str!("input")), 1011);
}

#[test]
fn correct_part2() {
    panic!("Score: {}", find_total_part2(include_str!("input")));
}