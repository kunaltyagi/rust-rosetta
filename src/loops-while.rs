// Implements http://rosettacode.org/wiki/Loops/While

#[cfg(not(test))]
fn main() {
    let mut i = 1024;
    while i > 0 {
        println!("{}", i);
        i /= 2;
    }
}