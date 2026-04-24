use num_sys::BiCompNum;

fn main() {
    let mut n = BiCompNum::new_i(1, 1, 1, 1);
    let mut buf = String::new();
    loop {
        println!("{}", n.abs());
        n = n.exp();
        std::io::stdin().read_line(&mut buf).expect("Unable to read stdin");
    }
}

