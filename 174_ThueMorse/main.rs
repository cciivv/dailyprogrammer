/*compute thun-morse sequences
http://www.reddit.com/r/dailyprogrammer/comments/2cld8m/8042014_challenge_174_easy_thuemorse_sequences/
limited to 64 on modern systems due to uint => u64, not wise to actually try much higher than 30 unless
your CPU time isn't valuable.
 */
extern crate test;
use std::os;

mod tm_seq {
    pub mod direct {
        /*
           the n'th element of tm(n) can be determined by counting the number of 1's present in the
           binary representation of 'n'. If it is even, tm(n) = 0, if odd, tm(n) = 1
         */
        pub fn is_odious_check(n: &uint) -> bool {
            //from John H Conway et al. odius = a number whose binary rep. has an odd number of 1's
            let mut num = *n;
            let mut sum = 0u;
            while num != 0 {
                sum = (num & 1) + sum;
                num = num >> 1;
            }
            (sum % 2 == 1)
        }

        pub fn is_odd_check(n: &uint) -> bool {
            let mut num = *n;
            let mut count = 0u;
            while num != 0 {
                num = num  & (num - 1);
                count = count + 1;
            }
            (count % 2 == 1)
        }

        pub fn get_seq(is_odd: fn(&uint)->bool, num: uint) {
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
}

fn main() {
    for arg in os::args().iter() {
        let input: Option<uint> = from_str(arg.as_slice());
        if input.is_none() {
            continue;
        }
        let input = input.expect("Not a number");
        tm_seq::direct::get_seq(tm_seq::direct::is_odd_check, input);
    }
}

#[cfg(test)]
mod testee {
    use test::Bencher;
    use tm_seq::direct;

    static BENCH_SIZE: uint = 20;

#[bench]
    fn bench_odd(b: &mut Bencher) {
        b.iter(|| {for i in range(0u, (1 << BENCH_SIZE)) {direct::is_odd_check(&i);}})
    }

#[bench]
    fn bench_odious(b: &mut Bencher) {
        b.iter(|| {for i in range(0u, (1 << BENCH_SIZE)) {direct::is_odious_check(&i);}})
    }

}
