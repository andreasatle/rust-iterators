use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {

    one_to_ten();
    into_iter_vs_iter_vs_iter_mut();
    adapters();
    collect();
    transform_and_collect();
    string_manipulations();
    update_vector_with_iterators();

}

struct OneToTen {
    current: u32,
}

impl OneToTen {
    fn new() -> OneToTen {
        OneToTen {
            current: 0,
        }
    }
}

/// The iterator trait requires:
/// * type Item
/// * fn next(&mut self) -> Option<Self::Item>
impl Iterator for OneToTen {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 10 {
            self.current += 1;
            return Some(self.current);
        }
        None
    }
}


fn one_to_ten() {

    // Use the OneToTen iterator trait defined below.
    for i in OneToTen::new() {
        println!("OneToTen Iterator: {}", i);
    }

    // Range already have the iterator trait.
    for i in 1..11 {
        println!("Range Iterator: {}", i);
    }
}

fn into_iter_vs_iter_vs_iter_mut() {
    let mut v = vec!['a','b','c'];

    // iter will borrow the data immutably.
    for c in v.iter() {
        println!("for c in v.iter: {}", c);
    }

    // iter_mut will borrow the data mutably.
    for c in v.iter_mut() {
        *c = 'q';
    }

    // into_iter consumes the data.
    for c in v.into_iter() {
        println!("for c in v.into_iter: {}", c);
    }

    // Need to re initialize the vector v, since it has been consumed.
    let v = vec!['a','b','c'];

    // c in v implicitly consumes the vector.
    for c in v {
        println!("for c in v: {}", c);
    }

    // This will not work since the vector is consumed.
    // println!("{:?}", v);
}

fn adapters() {
    // std::iter::repeat(1) repeats 1 for ever.
    // .take(5) adapts the iterator to stop after 5 values.
    for i in std::iter::repeat(1).take(5) {
        println!("{}",i);
    }

    // map adapts the iterator.
    let pairs = [(1,2), (3,4), (5,6), (7,8), (9,10), (11,12)].iter();
    for s in pairs.map(|&(a,b)| a+b) {
        println!("sum: {}", s);
    }

    // filter is also an adapter.
    for i in (1..=16).filter(|&x| x&1 == 1) {
        println!("Filter: {}", i);
    }
}

fn collect() {
    // collects makes e.g. a vector out of an iterator.
    // The user has to specify what kind of object the
    // iterator is collected into.
    let v: Vec<_> = (1..=11).into_iter().collect();
    println!("{:?}", v);

    // Convert a vector of chars into a string
    let chrs = ['q','w','e','r','t','y'];
    let s: String = chrs.iter().collect();
    println!("{:?}, {}", chrs, s);

    let hash_map: HashMap<_,_> = vec![("qwe",2),("wer",3),("ert",5),("rty",7)].into_iter().collect();
    println!("HashMap: {:?}", hash_map);

}

// This example requires lifetimes.
// This will be investigated in a different tutorial.
#[derive(Debug)]
struct Customer<'e> {
    email: &'e str,
    owed: f64,
}

impl <'e> Customer<'e> {
    fn new(email: &'e str, owed: f64) -> Customer {
        Customer {email, owed}
    }
}

fn transform_and_collect() {
    let customers = vec![
        Customer::new("andy@stuff.eu", 0.0),
        Customer::new("bernie@stuff.eu", 17.0),
        Customer::new("cindy@stuff.eu", 31.0),
        Customer::new("dandy@stuff.eu", 8.0),
        Customer::new("ernie@stuff.eu", 21.0),
        Customer::new("fergie@stuff.eu", 0.0),
    ];

    // Common pattern: filter -> map -> collect
    let debtor_emails: Vec<_> = customers.iter()
        .filter(|c| c.owed > 10.0)
        .map(|c| c.email)
        .collect();
    println!("{:?}", debtor_emails);

    // We can use find on iterators, result is an Option
    match customers.iter().find(|c| c.email == "bernie@stuff.eu") {
        Some(name) => println!("Found: {:?}", name),
        None => println!("Search failed.")
    }

    match customers.iter().find(|c| c.email == "andrew@stuff.eu") {
        Some(name) => println!("Found: {:?}", name),
        None => println!("Search failed.")
    }

    match customers.iter().find(|c| c.owed == 0.0) {
        Some(name) => println!("Found: {:?}", name),
        None => println!("Search failed.")
    }

    // Case with enumerate, find the index to Dandy.
    match customers.iter().enumerate().find(|(_,c)| c.email == "dandy@stuff.eu") {
        Some((i,_)) => println!("Found: {:?}", i),
        None => println!("Search failed.")
    }

    // Case with enumerate, find the index to Candy.
    match customers.iter().enumerate().find(|(_,c)| c.email == "candy@stuff.eu") {
        Some((i,_)) => println!("Found: {:?}", i),
        None => println!("Search failed.")
    }

}

fn string_manipulations() {
    // String::split give an iterator that can be manipulated.
    let s = "do re mi fa so la si do";
    let rev_s: String = s.split(' ').rev().collect();
    println!("{}", s);
    println!("{}", rev_s);
}

fn update_vector_with_iterators() {
    let n_col = 1000;
    let n_row = 1000;
    let row = |p:usize|p/n_col;
    let col = |p:usize|p%n_col;
    let mut array: Vec<_> = (1..=n_row*n_col).collect();

    // Use the sequential iterator iter_mut from rust
    let now = Instant::now();
    array.iter_mut().enumerate().for_each(|(i,v)|{
        *v = row(i)*col(i);
    });
    println!("Sequential: {:?}",now.elapsed());

    // Use the concurrent iterator par_iter_mut from rayon
    let now = Instant::now();
    array.par_iter_mut().enumerate().for_each(|(i,v)|{
        *v = row(i)*col(i);
    });
    println!("Concurrent: {:?}",now.elapsed());
}