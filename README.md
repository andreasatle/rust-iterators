# Rust Iterators
## Disclaimer
I have followed a blog by [Tomasz Kurcz](https://www.newline.co/@uint/rust-iterators-a-guide--80e35528).

## The Iterator Trait
The Iterator trait defines a type ```Item``` and a method next(), that returns an ```Option<Item>```. We can define our own iterator ```OneToTen``` as
```rust
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

Option<Self::Item>
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

```
that we can use in e.g. a for-loop
```rust
for i in OneToTen::new() {
    println!("OneToTen Iterator: {}", i);
}
```

## iter vs iter_mut vs into_iter
We illustrate by an example code:
```rust
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
v = vec!['a','b','c'];

// c in v implicitly consumes the vector.
for c in v {
    println!("for c in v: {}", c);
}

// This will not work since the vector is consumed.
// println!("{:?}", v);
```

## Adapters
```rust
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
```

## Collect
```rust
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
```

## Transform and collect
```rust
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

// Successful Find Costumer with given email.
match customers.iter().find(|c| c.email == "bernie@stuff.eu") {
    Some(name) => println!("Found: {:?}", name),
    None => println!("Search failed.")
}

// Failed Find Costumer with given email.
match customers.iter().find(|c| c.email == "andrew@stuff.eu") {
    Some(name) => println!("Found: {:?}", name),
    None => println!("Search failed.")
}

// Find (first) costumer with owed == 0.0.
match customers.iter().find(|c| c.owed == 0.0) {
    Some(name) => println!("Found: {:?}", name),
    None => println!("Search failed.")
}

// Successful Case with enumerate, find the index to Dandy.
match customers.iter().enumerate().find(|(_,c)| c.email == "dandy@stuff.eu") {
    Some((i,_)) => println!("Found: {:?}", i),
    None => println!("Search failed.")
}

// Failed Case with enumerate, find the index to Candy.
match customers.iter().enumerate().find(|(_,c)| c.email == "candy@stuff.eu") {
    Some((i,_)) => println!("Found: {:?}", i),
    None => println!("Search failed.")
}
```

## Introduction to ```rayon```
```rust
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
```

The last code given output:
```
Sequential: 43.019101ms
Concurrent: 7.452054ms
```

It is evident that the concurrent loop using [```rayon```](https://docs.rs/rayon/latest/rayon/) is about 6 times faster. It is quite a bit of overhead before the concurrent run is faster, but once the threshold is reach, we see a significant improvement.