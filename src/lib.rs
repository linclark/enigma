#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_rust_function']"]
#[link_args = "-s DEMANGLE_SUPPORT=1"]
extern {}
extern crate ultra;

use ultra::Enigma;
use ultra::decrypt;

#[no_mangle]
pub fn rust_function(n:i32) -> i32 {
    let mut enigma = Enigma::new("123", "DOG", "CAT", 'A', "");
    let msg = enigma.encrypt("WHY DON'T WE TRY ANOTHER SENTENCE AND SEE IF THE RESULT IS BETTER");
    println!("{}", msg);

    let msg2 = decrypt("YCJ'K AXR GPPACDD IKUO JBF TBYHPHJ ZTQQGGYN");
    println!("{}", msg2.0);

    n + 42
}

fn main() {
    /* Intentionally left blank */
}