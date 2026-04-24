use num_sys::BiCompNum;

fn main() {
    let n = BiCompNum::new_i(0, 0, 0, 1);
    println!("{}", n.exp())
}

