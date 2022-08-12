pub fn run(){
    println!("{}",lint(5));
}

fn lint<'a>(num: u32) -> u32{
    num*2
}

// warning: this lifetime isn't used in the function definition
//  --> src/extra_unused_lifetimes.rs:5:9
//   |
// 5 | fn lint<'a>(num: u32){
//   |         ^^
//   |
//   = note: `#[warn(clippy::extra_unused_lifetimes)]` on by default
//   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#extra_unused_lifetimes

// warning: `lifetime-lints` (bin "lifetime-lints") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 0.23s