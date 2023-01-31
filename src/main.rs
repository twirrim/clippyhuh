use rayon::prelude::*;

fn main() {
    (0..128).into_par_iter().for_each(|number| {
        println!("Number {}", number);
    });
}
