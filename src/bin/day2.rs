// What do we need to do?
// Check if the input (opponent behaviour) matches the right player response
// -> Yiels a score
// Match the played value against its integer value
// -> Yiels a point of 1, 2, 3
// Sum the values in the row
// Sum the rows
//
// A = Rock = 1
// B = Paper = 2
// C = Scissors = 3
//
// X = Loss = 0
// Y = Draw = 3
// Z = Win = 6
#[derive(Debug)]
struct IndividualPlay {
    play: String,
    score: i32
}

fn score_play(play_result: &str) -> i32 {
    match play_result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        &_ => {
            println!("{}", play_result);
            return 1;
        }
    }
}

fn score_player_response(opponent_input: &str, play_result: &str) -> i32 {
    if opponent_input == "A" {
        match play_result{
            "Y" => 1,
            "X" => 3,
            "Z" => 2,
            &_ => {
                println!("{}", opponent_input);
                return 0;
            }
        }
    } else if opponent_input == "B" {
        match play_result {
            "Y" => 2,
            "X" => 1,
            "Z" => 3,
            &_ => {
                println!("{}", opponent_input);
                return 0;
            }
        }
    } else if opponent_input == "C" {
        match play_result {
            "Y" => 3,
            "X" => 2,
            "Z" => 1,
            &_ => {
                println!("{}", opponent_input);
                return 0;
            }
        }
    } else {
        println!("{}", opponent_input);
        return 0;
    }
}

fn score_individual_play(mut play: IndividualPlay) -> IndividualPlay {
    let mut inputs = play.play.split_whitespace();
    let opponent_input = inputs.next().unwrap();
    let play_result = inputs.next().unwrap();

    play.score = score_play(play_result) + score_player_response(opponent_input, play_result);

    return play;
}

fn main() {
    let input = std::fs::read_to_string("./src/bin/day2_input.txt")
        .expect("Should be a string");

    let mut total_score = 0;

    input
        .lines()
        .map(|x| return IndividualPlay {
            play: x.to_string(),
            score: 0
        })
        .map(|play| return score_individual_play(play))
        .for_each(|play| total_score += play.score);

    println!("{:#?}", total_score);
}
