use crate::input;

fn string_to_vec(buf: &str) -> Vec<&str> {
    return buf.split("\n").filter(|x| *x != "").collect();
}

fn match_result(scenario: &&str) -> i32 {
    let scenario_vec: Vec<&str> = scenario.split(" ").collect();
    match scenario_vec[..] {
        // We choose rock
        [other_hand, "X"] => {
            let initial_point = 1;

            match other_hand {
                "A" => return initial_point + 3,
                "B" => return initial_point + 0,
                "C" => return initial_point + 6,
                _ => todo!("lol"),
            }
        }
        // We choose paper
        [other_hand, "Y"] => {
            let initial_point = 2;

            match other_hand {
                "A" => return initial_point + 6,
                "B" => return initial_point + 3,
                "C" => return initial_point + 0,
                _ => todo!("lol"),
            }
        }
        // We choose scizor
        [other_hand, "Z"] => {
            let initial_point = 3;

            match other_hand {
                "A" => return initial_point + 0,
                "B" => return initial_point + 6,
                "C" => return initial_point + 3,
                _ => todo!("lol"),
            }
        }
        _ => todo!("lol"),
    }
}


const ROCK_SCORE: i32 =  1;
const PAPER_SCORE: i32 = 2;
const SCISSOR_SCORE: i32 =  3;


const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn match_result_b(scenario: &&str) -> i32 {
    let scenario_vec: Vec<&str> = scenario.split(" ").collect();
    match scenario_vec[..] {
        // enemy rock 

        // lose
        ["A","X"] =>  LOSE_SCORE + SCISSOR_SCORE ,
        // draw
        ["A","Y"] =>  DRAW_SCORE + ROCK_SCORE,
        // win
        ["A","Z"] =>  WIN_SCORE + PAPER_SCORE,

        // enemy paper
        // lose
        ["B","X"] =>  LOSE_SCORE + ROCK_SCORE,
        // draw 
        ["B","Y"] =>  DRAW_SCORE + PAPER_SCORE,
        // win
        ["B","Z"] =>  WIN_SCORE + SCISSOR_SCORE,
          
        // enemy sci
        // lose
        ["C","X"] => LOSE_SCORE + PAPER_SCORE,
        // DRAW
        ["C","Y"] =>  DRAW_SCORE + SCISSOR_SCORE,
        // WIN
        ["C","Z"] =>  WIN_SCORE + ROCK_SCORE,
        
        _ => todo!("lol"),
    }
}

pub fn solve() {
    let buf = input::read_input("inputs/2");
    let vec = string_to_vec(&buf);
    println!("{:?}", vec);
    println!("Solution 2A: {:?}", vec.iter().map(match_result).sum::<i32>());
    println!("Solution 2B: {:?}", vec.iter().map(match_result_b).sum::<i32>());
}
