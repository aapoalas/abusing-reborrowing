#![allow(dead_code, unused, clippy::needless_lifetimes)]

mod s1_vec;
mod s2_vec_index;
mod s3_arena_index;
mod s4_arena_index_with_token;
mod s5_pass_arena_index;
mod s6_return_arena_index;
mod s7_zst_token;
mod s8_scoped_rooting;

fn main() {
	if std::env::args().any(|v| v.contains("s=1")) {
    	s1_vec::start();
	} else if std::env::args().any(|v| v.contains("s=2")) {
	    s2_vec_index::start();
	} else if std::env::args().any(|v| v.contains("s=3")) {
	    s3_arena_index::start();
	} else if std::env::args().any(|v| v.contains("s=4")) {
	    s4_arena_index_with_token::start();
	} else if std::env::args().any(|v| v.contains("s=5")) {
	    s5_pass_arena_index::start();
	} else if std::env::args().any(|v| v.contains("s=6")) {
	    s6_return_arena_index::start();
	} else if std::env::args().any(|v| v.contains("s=7")) {
	    s7_zst_token::start();
	} else if std::env::args().any(|v| v.contains("s=8")) {
	    s8_scoped_rooting::start();
	} else {
		unreachable!("Invalid call, pass in a s=N argument");
	}
}
