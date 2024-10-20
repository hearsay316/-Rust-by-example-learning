fn main() {
    let strings = vec!["tofu", "98", "18"];
    // let numbers:Vec<_> = strings.into_iter().map(|s|s.parse::<i32>().unwrap()).collect();
    // let numbers:Vec<_> = strings.into_iter()
    //     .filter_map(|s|s.parse::<i32>().ok())
    //     .collect();

    // let numbers:Result<Vec<_>,_> = strings.into_iter()
    //     .map(|s|s.parse::<i32>()).collect();
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Errors {:?}", errors);
    println!("Results {:?}", numbers);
    println!("{:?}", strings);
}
