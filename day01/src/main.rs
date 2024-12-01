use std::{collections::HashMap, fs, ops::Mul, path::Path};

fn parse_in_column(data: String) -> (Vec<i64>, Vec<i64>) {
    data.lines()
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .unzip()
}

fn find_total_distance(column_one: &Vec<i64>, column_two: &Vec<i64>) -> i64 {
    let c_one = column_one.clone();
    let c_two = column_two.clone();

    c_one
        .into_iter()
        .zip(c_two)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn build_frequency_table(column_two: &Vec<i64>) -> HashMap<i64, i64> {
    let mut frequency_table: HashMap<i64, i64> = HashMap::new();
    column_two.iter().for_each(|value| {
        *frequency_table.entry(*value).or_insert(0) += 1;
    });
    frequency_table
}

fn calculate_similarity_score(column_one: &Vec<i64>, frequency_table: HashMap<i64, i64>) -> i64 {
    column_one
        .into_iter()
        .map(|value| frequency_table.get(value).copied().unwrap_or(0).mul(value))
        .sum()
}

fn main() {
    let data = match fs::read_to_string(Path::new("src/input1.txt")) {
        Ok(filecontent) => filecontent,
        Err(e) => panic!("Error while opening file: {e:?}"),
    };
    let (column_one, column_two) = parse_in_column(data);
    let total_distance = find_total_distance(&column_one, &column_two);
    println!("The total distance is {}", total_distance);
    let frequency_table = build_frequency_table(&column_two);
    let total_frequency = calculate_similarity_score(&column_one, frequency_table);
    println!("The total frequency is {}", total_frequency);
}
