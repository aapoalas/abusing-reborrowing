//! Taking indexes into vectors.

// #[allow(empty_line_for_viewing_pleasure)]
fn act(vec: &mut Vec<u32>) {
    let first_index = add(vec, rand::random());			// <-- 	+ '1
    println!("First value: {}", vec[first_index]);		//		|
    let second_index = add(vec, rand::random());		// <-- 	+ '2
    println!("First value: {}", vec[first_index]);		//		|
    println!("Second value: {}", vec[second_index]);	//		|
    clean(vec);											//		- '1, '2
    println!("First value: {}", vec[first_index]);		//		?
    println!("Second value: {}", vec[second_index]);	//		?
}

/// Add a value to vector and return its index.
fn add(vec: &mut Vec<u32>, value: u32) -> usize {
    vec.push(value);
    vec.len() - 1
}









/// Clean the vector of unwanted values.
fn clean(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}