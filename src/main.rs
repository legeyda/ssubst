

extern crate ssubst;

use std::env::{Args, args};
use std::io::{stdin, stdout};
use std::vec;
use std::iter::Iterator;
use std::process::exit;
	
use ssubst::Subst;


fn print_usage() {
	println!("Usage: ");
	println!("    echo abc | ssubst a x b y c z");
	println!("    which gives xyz");

}

fn main() {

	let mut substs: Vec<Subst> = Vec::new();

	let mut iterator = args();
	iterator.next(); // skip $0
	loop {
		match(iterator.next()) {
			Some(needle) => {
				match(iterator.next()) {
					Some(replace) => {
						substs.push(Subst::new(needle.into_bytes(), replace.into_bytes()));

					},
					None => {
						println!("no replacement for needle {}", needle);
						print_usage();
						exit(2);
					}
				}
			},
			None => {
				break;
			}
		}
	}

	if substs.len()==0 {
		println!("At least one replacement needed");
		print_usage();
		exit(1);		
	}

	ssubst::ssubst(&mut stdin(), &mut stdout(), &substs);



}

// 

