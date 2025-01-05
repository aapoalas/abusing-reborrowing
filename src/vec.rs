pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}

fn act(vec: &mut Vec<u32>) {
    let first_value = add(vec, rand::random());
    println!("First value: {}", first_value);
    let second_value = add(vec, rand::random());
    // println!("First value: {}", first_value);
    println!("Second value: {}", second_value);
    clean(vec);
    // println!("First value: {}", first_value);
    // println!("Second value: {}", second_value);
}

fn add(vec: &mut Vec<u32>, value: u32) -> &u32 {
    vec.push(value);
    vec.last().unwrap()
}

fn clean(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}
