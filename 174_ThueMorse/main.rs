/*compute thun-morse sequences
http://www.reddit.com/r/dailyprogrammer/comments/2cld8m/8042014_challenge_174_easy_thuemorse_sequences/
limited to 64 on modern systems due to uint => u64, not wise to actually try much higher than 30 unless
your CPU time isn't valuable.
 */
extern crate test;
use std::os;

mod tm_seq {
        fn is_odd(n: &uint) -> bool {
            let mut num = *n;
            let mut count = 0u;
            while num != 0 {
                num = num  & (num - 1);
                count = count + 1;
            }
            (count % 2 == 1)
        }

        pub fn get_seq(num: uint) {
            assert!(num < 64);
            println!("Thun-Morse({}) =", num);
            for i in range(0u, (1 << num)) {
                let output = match is_odd(&i) {
                    true => 1u,
                         false => 0
                };
                print!("{}", output);
            }
            println!("");
        }
}

fn main() {
    let input = from_str(os::args()[1].as_slice()).expect("Not a number");
    tm_seq::get_seq(input);
}

