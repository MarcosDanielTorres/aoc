#[derive(Copy, Clone)]
enum Color {
    R,
    G,
    B,
}

#[derive(Copy, Clone)]
struct ColorInfo {
    color: Color,
    value: u32,
}

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;

struct SetDescriptor(Vec<ColorInfo>);

impl From<Vec<&str>> for SetDescriptor {
    fn from(value: Vec<&str>) -> Self {
        let mut descriptors: Vec<ColorInfo> = vec![];

        for st in value {
            let first_num_index = st.find(|c: char| c.is_digit(10)).unwrap();
            let first_alphabetic_index = st.find(|c: char| c.is_alphabetic()).unwrap();

            let value = &st[first_num_index..first_alphabetic_index - 1]
                .parse::<u32>()
                .unwrap();
            let st = &st[first_alphabetic_index..];

            let color_info = match st {
                "red" => ColorInfo {
                    color: Color::R,
                    value: *value,
                },
                "green" => ColorInfo {
                    color: Color::G,
                    value: *value,
                },
                "blue" => ColorInfo {
                    color: Color::B,
                    value: *value,
                },
                _ => unreachable!(),
            };
            descriptors.push(color_info);
        }

        SetDescriptor(descriptors)
    }
}

struct Game {
    id: u32,
    sets: Vec<SetDescriptor>,
}

impl Game {
    pub fn get_id(line: &str) -> u32 {
        let aux = &line.splitn(3, ' ').collect::<Vec<&str>>()[1];
        let id = aux[0..aux.len() - 1].parse().unwrap();
        dbg!(id);
        id
    }


    pub fn calculate_sets(line: &str) -> Vec<SetDescriptor> {
        let mut result = vec![];
        let start = line.find(':').unwrap();
        let sets = line[start + 1..].split(';').collect::<Vec<_>>();
        let sets_2 = sets
            .iter()
            .map(|x| x.split(','))
            .map(|x| x.collect::<Vec<&str>>())
            .collect::<Vec<_>>();

        dbg!(&sets_2);

        for set in sets_2 {
            dbg!(&set);
            let set_descriptor = SetDescriptor::from(set);
            result.push(set_descriptor);
        }
        result
    }

    pub fn is_sum_possible(&self) -> bool {
        for sd in self.sets.iter() {
            for s in sd.0.iter() {
                let mut r_sum = 0;
                let mut g_sum = 0;
                let mut b_sum = 0;
                match s {
                    ColorInfo {
                        color: Color::R,
                        value,
                    } => r_sum += value,
                    ColorInfo {
                        color: Color::G,
                        value,
                    } => g_sum += value,
                    ColorInfo {
                        color: Color::B,
                        value,
                    } => b_sum += value,
                };
                if r_sum > MAX_R || g_sum > MAX_G || b_sum > MAX_B {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn calculate_powers(&self) -> u32 {
        let mut powers = 1;

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        for set in self.sets.iter() {
            for color in set.0.iter(){
                match color {
                    ColorInfo {
                        color: Color::R,
                        value,
                    } => max_r = std::cmp::max(max_r, *value),
                    ColorInfo {
                        color: Color::G,
                        value,
                    } => max_g = std::cmp::max(max_g, *value),
                    ColorInfo {
                        color: Color::B,
                        value,
                    } => max_b = std::cmp::max(max_b, *value),
                };
            }
        }
        powers *= max_r * max_g * max_b;
        powers
    }
}

pub fn part2() {
    // let input = include_str!("test-input.txt").to_string();
    let input = include_str!("input.txt").to_string();
    let mut games: Vec<Game> = vec![];
    let mut total_sum = 0;
    let mut total_powers = 0;

    for line in input.lines() {
        let game = Game {
            id: Game::get_id(&line),
            sets: Game::calculate_sets(&line),
        };

        if game.is_sum_possible() {
            total_sum += game.id;
        }

        total_powers += game.calculate_powers();

        games.push(game); // Just to have a history of games. It's not useful in this exercise
    }
    println!("Total sum: {:?}", total_sum);
    println!("Total powers: {:?}", total_powers);
}
