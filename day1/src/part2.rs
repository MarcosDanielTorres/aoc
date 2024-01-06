fn to_num(st: &str) -> &'static str {
    match st {
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => unreachable!(),
    }
}

pub fn part_2() -> u32 {
    let mut total_sum: u32 = 0;
    let test_input: String = include_str!("input.txt").to_string();

    #[rustfmt::skip]
    let list_of_nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    for line in test_input.lines() {
        let mut min: i32 = line.len() as i32;
        let mut max: i32 = -1;
        let mut left_num = "0";
        let mut right_num = "0";
        for num in list_of_nums {
            let matches: Vec<_> = line.match_indices(num).collect();
            for (index, _) in matches.iter() {
                // dbg!(
                //     "there is a num {:?} at index: {:?} in this line: {:?}",
                //     num, index, line
                // );

                if (*index as i32) > max {
                    max = *index as i32;
                    right_num = num;
                }

                if (*index as i32) < min {
                    min = *index as i32;
                    left_num = num;
                }
            }
        }
        // dbg!("max {:?} right_num {:?}", max, right_num);
        // dbg!("min {:?} left_num {:?}", min, left_num);

        let max = max as usize;
        let min = min as usize;
        assert!(max >= min);

        let num = to_num(&left_num).parse::<u32>().unwrap() * 10
            + to_num(&right_num).parse::<u32>().unwrap();
        // dbg!("num is: {:?}", num);

        total_sum += num;
    }
    total_sum
}
