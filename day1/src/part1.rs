pub fn part_1() -> u32 {
    let input = include_str!("input.txt");
    let mut total_sum = 0;

    for line in input.lines() {
        let mut sum = 0;
        for ch in line.chars() {
            if ch.is_numeric() {
                sum += ch.to_digit(10).unwrap() * 10;
                break;
            }
        }

        for ch in line.chars().rev() {
            if ch.is_numeric() {
                sum += ch.to_digit(10).unwrap();
                break;
            }
        }
        total_sum += sum;
    }
    total_sum
}
