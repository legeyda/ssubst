

extern crate num;
use std::ops::{Add, Sub};
use self::num::integer::Integer;
use std::str::from_utf8;



pub struct ConfinedQueue<T> {
	len:  usize,
	data: Vec<T>,
	pos:  usize   // first element index
}



impl<T> ConfinedQueue<T> {

	pub fn new(len: usize) -> ConfinedQueue<T> {
		ConfinedQueue{len: len, data: Vec::with_capacity(len), pos: 0}
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn clean(&mut self) {	
		self.data.truncate(0);
		self.pos=0;
	}

	pub fn push(&mut self, val: T) {
		if self.data.len() < self.len {
			self.data.push(val);
		} else {
			self.data[self.pos]=val;
			self.pos = (self.pos + 1) % self.len;
		}
	}

	pub fn full(&self) -> bool {
		return self.data.len()==self.len;
	}

	pub fn get(&self, pos: usize) -> Option<&T> {
		self.data.get((self.pos + pos) % self.len)
	}

	pub fn first(&self) -> Option<&T> {
		self.get(0)
	}
	
	pub fn last(&self) -> Option<&T> {
		self.get(self.data.len()-1)
	}

}




impl<'a, 'b, A: Sized, T> PartialEq<ConfinedQueue<T>> for ConfinedQueue<A> where A: PartialEq<T> {

	fn eq(&self, other: &ConfinedQueue<T>) -> bool {
		if self.data.len()==other.data.len() {
			if self.data.len() < self.len {
				return self.data.eq(&other.data);
			} else {
				for i in 0 .. self.data.len() {
					if self.data.get((self.pos+i)%self.len).unwrap().ne(other.data.get((other.pos+i)%self.len).unwrap()) {
						return false;
					}
				}
				return true;
			}
		}
		return false;
	}

	fn ne(&self, other: &ConfinedQueue<T>) -> bool {
		if self.data.len()==other.data.len() {
			if self.data.len() < self.len {
				return self.data.ne(&other.data);
			} else {
				for i in 0 .. self.data.len() {
					if self.data.get((self.pos+i)%self.len).unwrap().ne(other.data.get((other.pos+i)%self.len).unwrap()) {
						return true;
					}
				}
				return false;
			}
		}
		return true;
	}

}


#[test]
fn test_confined_queue() {

	let mut x: ConfinedQueue<u8> = ConfinedQueue::new(3usize);
	let mut y: ConfinedQueue<u8> = ConfinedQueue::new(3usize);
	assert!(x.eq(&y), "test empty = empty");

	x.push(b'a');
	assert!(x.ne(&y), "test not empty != empty");

	y.push(b'a');
	assert!(x.eq(&y), "test not full == same not full");	

	x.push(b'b'); x.push(b'c'); x.push(b'd');
	y.push(b'b'); y.push(b'c'); y.push(b'd');
	assert!(x.eq(&y), "test full == same full");	

	x.push(b'x');
	y.push(b'y');

	assert!(x.ne(&y), "test full != different full");	

	x.clean();	
	println!("===> x IS [{}] pos {}", from_utf8(&x.data).unwrap(), x.pos);
	//println!("===> y IS [{}] pos {}", from_utf8(&y.data).unwrap(), y.pos);
	//println!("===> z IS [{}] pos {}", from_utf8(&z.data).unwrap(), z.pos);
	let mut z: ConfinedQueue<u8> = ConfinedQueue::new(3usize);
	assert!(x.eq(&z), "test cleaned == empty");		

}








pub struct HashedConfinedQueue {
	value: u32,
	queue: ConfinedQueue<u8>
}

impl HashedConfinedQueue {

	pub fn new(len: usize) -> HashedConfinedQueue {
		HashedConfinedQueue{value: 0, queue: ConfinedQueue::new(len)}
	}

	pub fn len(&self) -> usize {
		self.queue.len()
	}

	pub fn full(&self) -> bool {
		self.queue.full()
	}

	pub fn clean(&mut self) {
		self.queue.clean();
		self.value = 0;
	}

	pub fn push(&mut self, val: u8) {
		if self.queue.full() {
			self.value= self.value - *self.queue.first().unwrap() as u32;
		}
		self.value=self.value+val as u32;
		self.queue.push(val);
	}

	pub fn get(&self, pos: usize) -> Option<&u8> {
		self.queue.get(pos)
	}

	pub fn first(&self) -> Option<&u8> {
		self.queue.first()
	}
	
	pub fn last(&self) -> Option<&u8> {
		self.queue.last()
	}

	pub fn hash_value(&self) -> u32 {
		self.value
	}

}

impl PartialEq<HashedConfinedQueue> for HashedConfinedQueue {

	fn eq(&self, other: &HashedConfinedQueue) -> bool {
		self.hash_value()==other.hash_value() && self.queue.eq(&other.queue)
	}

	fn ne(&self, other: &HashedConfinedQueue) -> bool {
		self.hash_value()!=other.hash_value() || self.queue.ne(&other.queue)
	}

}