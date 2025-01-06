//! Taking indexes into vectors.

fn act(
	vec: &mut Vec<u32>
) {
    let first = add(vec, rand::random());				// ==>	+ '1
    println!("First value: {}", vec[first]);			//		|
    let second = add(vec, rand::random());				// ==>	+ '2
    println!("First value: {}", vec[first]);			//		|
    println!("Second value: {}", vec[second]);			//		|
    gc(vec);											// <==	- '1, '2
    println!("First value: {}", vec[first]);			// ?
    println!("Second value: {}", vec[second]);			// ?
    // These lines intentionally left blank.
    //
    //
    //
    //
    //
    //
}

/// Add value to vector and return its index.
fn add(vec: &mut Vec<u32>, value: u32) -> usize {
    vec.push(value);
    vec.len() - 1
}









/// Clean the vector of unwanted values.
fn gc(vec: &mut Vec<u32>) {
    vec.retain(|_| rand::random::<bool>());
}

pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    act(&mut vec);
}
