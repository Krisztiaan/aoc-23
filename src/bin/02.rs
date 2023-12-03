advent_of_code::solution!(2);

const possible: (u8, u8, u8) = (12, 13, 14);

pub fn part_one(input: &str) -> Option<u32> {
    return input
        .lines()
        .filter_map(|line| {
            if let Some((game, draw_text)) = line.split_once(":") {
                let game_num = game.replace("Game ", "").parse::<u32>().unwrap();
                let draws = draw_text
                    .split(|c| c == ',' || c == ';')
                    .map(|v| v.trim_start())
                    .find(|v| {
                        if let Some((count, color)) = v.split_once(" ") {
                            let c = count.parse::<u8>().unwrap();
                            let is_impossible = match color {
                                "red" => c > 12,
                                "green" => c > 13,
                                "blue" => c > 14,
                                _ => false,
                            };
                            return is_impossible;
                        }
                        return false;
                    });
                if let Some(_d) = draws {
                    return None;
                }
                return Some(game_num);
            }
            return None;
        })
        .sum::<u32>()
        .into();
}

pub fn part_two(input: &str) -> Option<u32> {
    return input
        .lines()
        .map(|line| {
            if let Some((game, draw_text)) = line.split_once(":") {
                let game_num = game.replace("Game ", "").parse::<u32>().unwrap();
                let (mut min_red, mut min_green, mut min_blue): (u32, u32, u32) = (0, 0, 0);
                draw_text.split(|c| c == ',' || c == ';').for_each(|v| {
                    if let Some((count, color)) = v.trim_start().split_once(" ") {
                        let c = count.parse::<u32>().unwrap();
                        match color {
                            "red" => {
                                if c > min_red {
                                    min_red = c;
                                }
                            }
                            "green" => {
                                if c > min_green {
                                    min_green = c;
                                }
                            }
                            "blue" => {
                                if c > min_blue {
                                    min_blue = c;
                                }
                            }
                            _ => {}
                        };
                    }
                });
                return min_red * min_blue * min_green;
            }
            return 0;
        })
        .sum::<u32>()
        .into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
