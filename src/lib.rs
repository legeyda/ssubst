//extern crate ssubst;

use std::io::{Read, Write};
use std::str::from_utf8;
use std::io::Take;

mod queue;

use queue::HashedConfinedQueue;

pub struct Subst {
	pub needle:  Vec<u8>,
	pub replace: Vec<u8>
}

impl Subst {
	pub fn new(needle: Vec<u8>, replace: Vec<u8>) -> Subst {
		Subst{needle: needle, replace: replace}
	}
}


pub struct SubstDatum<'a> {
	pub needle_hasher: HashedConfinedQueue,
	pub stream_hasher: HashedConfinedQueue,
	pub subst: &'a Subst
}

// data that needed to be stored for each subst
impl<'a> SubstDatum<'a> {
	pub fn new(subst: &Subst) -> SubstDatum {
		let mut result = SubstDatum {
			needle_hasher: HashedConfinedQueue::new(subst.needle.len()), 
			stream_hasher: HashedConfinedQueue::new(subst.needle.len()), 
			subst: subst
		};
		for &byte in subst.needle.iter() {
			result.needle_hasher.push(byte)
		}
		result
	}
}

fn write_output<W: Write>(output: &mut W, byte: &u8) {
	let buf: [u8; 1] = [*byte];
	output.write(&buf[0..1]);
}

// rabin-carp algorithm
pub fn ssubst<R: Read, W: Write>(input: &mut R, output: &mut W, substs: &Vec<Subst>) -> Result<u32, String> {

	// check for empty needles
	for subst in substs {
		if subst.needle.len()==0 {
			return Err("needle cannot be empty".to_owned());
		}
	}

	// init substs data
	let mut subst_data: Vec<SubstDatum> = Vec::with_capacity(substs.len());	
	let mut max_subst_len: usize = 0;
	let mut longest_subst_index: usize = 0;
	for (i, subst) in substs.iter().enumerate() {
		if subst.needle.len()>max_subst_len {
			max_subst_len=subst.needle.len();
			longest_subst_index=i;
		}
		subst_data.push(SubstDatum::new(subst));
	}
	


	// go
	for (i, byte_result) in input.bytes().enumerate() {
		match byte_result {
			Ok(byte) => {
				let mut matched_subst_index: isize = -1;
				for (j, subst_datum) in subst_data.iter_mut().enumerate() {
					if subst_datum.stream_hasher.len() >= subst_datum.needle_hasher.len() {
						if subst_datum.stream_hasher.eq(&subst_datum.needle_hasher) {
							matched_subst_index=j as isize;
							break;
						}
					}
				}
				if matched_subst_index>=0 {
					for &replace_byte in subst_data[matched_subst_index as usize].subst.replace.iter() {
						write_output(output, &replace_byte)
					}
					for subst_datum in subst_data.iter_mut() {
						subst_datum.stream_hasher.clean();	
					}
				}
				if subst_data[longest_subst_index].stream_hasher.full() {
					write_output(output, subst_data[longest_subst_index].stream_hasher.first().unwrap());
				}
				for (j, subst_datum) in subst_data.iter_mut().enumerate() {
					subst_datum.stream_hasher.push(byte);
				}
			}, 
			Err(x) => {				
				break;
			}
		}
	}

	for j in 0 .. subst_data[longest_subst_index].stream_hasher.len() {
		write_output(output, subst_data[longest_subst_index].stream_hasher.get(j).unwrap());
	}

	return Ok(0);
}