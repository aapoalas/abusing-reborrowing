//! Index newtype into vectors with separate marker lifetime.

#[allow(clippy::needless_lifetimes)]
fn act<'a>(vec: &mut Vec<u32>, token: &'a mut Token) {	// <--				+ 'a lifetime
    let first_index = add(vec, rand::random(), token);	// <--	+ '1		|
    println!("First value: {}", vec[first_index]);		//		|			|
    let second_index = add(vec, rand::random(), token);	// <--	+ '2		|
    println!("First value: {}", vec[first_index]);		//		|			|
    println!("Second value: {}", vec[second_index]);	//		|			|
    clean(vec, token);									//		- '1, '2	|
    // println!("First value: {}", vec[first_index]);	//					|
    // println!("Second value: {}", vec[second_index]);	//					|
}														//	<--				-

/// Marker ZST, exclusive access to the Token is required to clean the
/// Vec<u32>.
struct Token();

/// Add a value to vector and return its index as VecIndex, bound to a shared
/// borrow of Token.
fn add<'a>(vec: &mut Vec<u32>, value: u32, _: &'a Token) -> VecIndex<'a> {
    vec.push(value);
    VecIndex(vec.len() - 1, PhantomData)
}

/// Clean the vector of unwanted values, requiring exclusive access to Token.
fn clean(vec: &mut Vec<u32>, _: &mut Token) {
    vec.retain(|_| rand::random::<bool>());
}










pub(crate) fn start() {
    let mut vec = vec![0, 1, 2, 3, 4, 5];
    let mut token = Token();
    act(&mut vec, &mut token);
}

#[derive(Clone, Copy)]
struct VecIndex<'a>(usize, PhantomData<&'a u32>);

impl Index<VecIndex<'_>> for Vec<u32> {
    type Output = u32;

    fn index(&self, index: VecIndex<'_>) -> &Self::Output {
        self.index(index.0)
    }
}

impl VecIndex<'_> {
    #[allow(dead_code)]
    fn unbind(self) -> VecIndex<'static> {
        unsafe { std::mem::transmute::<VecIndex, VecIndex<'static>>(self) }
    }
}

use std::{marker::PhantomData, ops::Index};

