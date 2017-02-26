extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

static mut RNG: Option<StdRng> = None;

use std::process::Command;

fn main() {
    let mut rng;
    unsafe {
        if RNG.is_none() {
            let seed: &[_] = &[42];
            RNG = Some(SeedableRng::from_seed(seed));
        }

        rng = RNG.as_mut().unwrap();
    }

    let func_str = match rng.gen::<u8>() % 4 {
        0 => template("false"),
        1 => template("!b"),
        2 => template("b"),
        _ => template("true"),
    };

    println!("{}", func_str);

    let output = Command::new("ls")
        .output()
        .expect("ls command failed to start");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn template(code: &str) -> String {
    format!(r"
    pub fn fill_me_in (b : bool) -> bool {{
        {}
    }}
    ",
            code)
}
