fn main() {
    let input: String  = std::fs::read_to_string("./src/bin/day1_input.txt")
        .expect("Should have been able to read this file");
    
    let mut parsed_input: Vec<i32> = input.split("\n\n")
        .map(|x| x.split("\n")
            .flat_map(|x| x.parse::<i32>())
            .sum())
        .collect::<Vec<i32>>();

    parsed_input.sort_by(|a, b| b.cmp(a));

    println!("{0}", parsed_input[0] + parsed_input[1] + parsed_input[2]);
    
}
