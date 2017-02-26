extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

static mut RNG: Option<StdRng> = None;

use std::process::Command;
use std::fs::File;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let seed: usize = 42;

    println!("seed: {}", seed);

    let mut rng;
    unsafe {
        if RNG.is_none() {
            let seed: &[_] = &[seed];
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

    let puzzle_name: String = "puzzle_".to_string() + seed.to_string().as_ref();

    {
        let output = Command::new("cargo")
            .arg("new")
            .arg(&puzzle_name)
            .output()
            .expect("cargo does not seem to be installed and on the PATH");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    cd(&puzzle_name);

    Command::new("mkdir")
        .arg("-p")
        .arg("tests")
        .output()
        .expect("mkdir does not seem to be installed and on the PATH");

    cd("tests");

    let test_filename = puzzle_name.to_string() + ".rs";

    let mut test_file = File::create(&test_filename).unwrap();

    write!(test_file,
           r##"
    #[cfg(test)]
    #[macro_use]
    extern crate quickcheck;
    extern crate {puzzle_name};

    {func_str}

    #[cfg(test)]
    quickcheck! {{
        fn prop(input: bool) -> () {{
            let expected = fill_me_in(input);
            let recieved = {puzzle_name}::fill_me_in(input);

            if expected != recieved {{
              panic!("given an input of ({{input}}) \
              we expeceted a result of ({{expected}}) \
              but we recieved ({{recieved}}) instead!",
               input=input, expected=expected, recieved=recieved)
           }}

       }}
   }}
    "##,
           puzzle_name = puzzle_name,
           func_str = func_str)
        .unwrap();

    cd("../src");

    let mut template_file = File::create("lib.rs").unwrap();

    write!(template_file, "{}", template("unimplemented!();")).unwrap();

    cd("..");

    let mut toml = OpenOptions::new().append(true).open("Cargo.toml").unwrap();

    write!(toml,
           "{}",
           r#"

    [dev-dependencies]
    quickcheck = "0.3"
    "#)
        .unwrap();

    println!("wrote {}", &puzzle_name);
}

fn template(code: &str) -> String {
    format!(r"
    pub fn fill_me_in (b : bool) -> bool {{
        {}
    }}
    ",
            code)
}

fn cd(path: &str) {
    env::set_current_dir(Path::new(path)).unwrap();
}

fn _ls() {
    let output = Command::new("ls")
        .output()
        .expect("ls does not seem to be installed and on the PATH");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
