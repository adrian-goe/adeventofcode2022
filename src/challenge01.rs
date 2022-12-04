use std::fs;

pub fn run<'a>() -> i32 {
    println!("In file {}", "./src/01_test_input.txt");

    let contents = fs::read_to_string("./src/01_test_input.txt")
        .expect("Should have been able to read the file");

    let split_lines = contents.split("\n").collect::<Vec<&str>>();
    let mut calories_per_elf: Vec<i32> = Vec::new();
    calories_per_elf.push(0);
    for i in 0..split_lines.len() {
        if split_lines[i] == "" {
            calories_per_elf.push(0);
            continue;
        }

        let index = calories_per_elf.len() - 1;
        calories_per_elf[index] = calories_per_elf[index] + split_lines[i].parse::<i32>()
            .unwrap();
    }

    let result = *calories_per_elf.iter().max().unwrap();
    println!("Result = {}", result);
    // part two
    calories_per_elf.sort();
    let length_a = calories_per_elf.len() - 1;
    let length_b = calories_per_elf.len() - 2;
    let length_c = calories_per_elf.len() - 3;
    println!("Result = {},{},{} That is together = {}",
             calories_per_elf[length_a],
             calories_per_elf[length_b],
             calories_per_elf[length_c],
             calories_per_elf[length_a] + calories_per_elf[length_b] + calories_per_elf[length_c]
    );

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(72478, run());
    }
}
