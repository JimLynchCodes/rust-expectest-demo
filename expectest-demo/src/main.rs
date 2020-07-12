#[cfg(test)]
use expectest::prelude::{be_equal_to, expect};

fn main() {
    println!("calling foo...");
    foo();
}

fn foo() -> u32 {
    42
}

#[test]
fn test_foo() {
    expect(foo()).to(be_equal_to(42));
}
