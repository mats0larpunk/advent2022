use crate::input;

fn string_to_vec_of_vec(buf: &str) -> Vec<Vec<i32>> {
    let outer_runs: Vec<&str> = buf.rsplit("\n\n").collect();
    let inner_runs: Vec<Vec<i32>> = outer_runs
        .iter()
        .map(|x| {
            x.rsplit("\n")
                .filter(|x| *x != "")
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    return inner_runs;
}

pub fn solve() {
    let buf = input::read_input("inputs/1");
    let runs = string_to_vec_of_vec(&buf);

    let mut sums: Vec<i32> = runs.iter().map(|x| x.iter().sum::<i32>()).collect();
    sums.sort();
    sums.reverse();
    println!("Solution for 1B: {:?}", &sums[0..3].iter().sum::<i32>());
}
