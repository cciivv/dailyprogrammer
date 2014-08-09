/*compute thun-morse sequences
http://www.reddit.com/r/dailyprogrammer/comments/2cld8m/8042014_challenge_174_easy_thuemorse_sequences/
limited to 64 on modern systems due to uint => u64, not wise to actually try much higher than 30 unless
your CPU time isn't valuable.
 */
use std::os;

mod tm_seq {
    pub mod direct {
        /*
           the n'th element of tm(n) can be determined by counting the number of 1's present in the
           binary representation of 'n'. If it is even, tm(n) = 0, if odd, tm(n) = 1
         */
        fn is_odious(n: &uint) -> bool {
            //from John H Conway et al. odius = a number whose binary rep. has an odd number of 1's
            let mut num = *n;
            let mut sum_mod_two = 0u;
            while num != 0 {
                sum_mod_two = ((num & 1) + sum_mod_two) % 2;
                num = num >> 1;
            }
            (sum_mod_two == 1)
        }

        pub fn get_seq(num: uint) {
            assert!(num < 64);
            println!("Thun-Morse({}) =", num);
            for i in range(0u, (1 << num)) {
                let output = match is_odious(&i) {
                    true => 1u,
                    false => 0
                };
                print!("{}", output);
            }
            println!("");
        }
    }
}

fn main() {
    for arg in os::args().iter() {
        let input: Option<uint> = from_str(arg.as_slice());
        if input.is_none() {
            continue;
        }
        let input = input.expect("Not a number");
        tm_seq::direct::get_seq(input);
    }
}

