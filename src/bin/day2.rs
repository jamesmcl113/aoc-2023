fn main() {
    let limit = Game {
        id: 0,
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum_of_powers: u32 = std::fs::read_to_string("../input/day2.txt")
        .expect("Error opening puzzle input")
        .lines()
        .map(|line| get_power_set(parse_game_data(line)))
        .sum();

    println!("{sum_of_powers}");
}

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn extract_cube_amount(cube_data: &str, color: &str) -> Option<u32> {
    if cube_data.contains(color) {
        let (amount, _) = cube_data
            .split_once(" ")
            .expect("Cube data should be in form '[amount] [color]'");
        amount.parse().ok()
    } else {
        None
    }
}

fn parse_game_data(line: &str) -> Game {
    let (game_id, cube_sets) = line
        .split_once(": ")
        .expect("Each game should start with Game [id]: ");

    let (_, id) = game_id
        .split_once(" ")
        .expect("Id should be of format Game [id]");

    let id: u32 = id.parse().expect("Id should be a u32");

    let mut game = Game {
        id,
        red: 0,
        green: 0,
        blue: 0,
    };

    let handfuls = cube_sets.split("; ");
    for handful in handfuls {
        let cubes = handful.split(", ");
        for cube_data in cubes {
            if let Some(amount) = extract_cube_amount(cube_data, "red") {
                game.red = u32::max(game.red, amount);
            } else if let Some(amount) = extract_cube_amount(cube_data, "green") {
                game.green = u32::max(game.green, amount);
            } else if let Some(amount) = extract_cube_amount(cube_data, "blue") {
                game.blue = u32::max(game.blue, amount);
            }
        }
    }

    game
}

fn is_possible_game(limit: &Game, game: Game) -> Option<u32> {
    if game.red <= limit.red && game.green <= limit.green && game.blue <= limit.blue {
        Some(game.id)
    } else {
        None
    }
}

fn get_power_set(game: Game) -> u32 {
    game.red * game.green * game.blue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let game = parse_game_data("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 6);
    }
}
