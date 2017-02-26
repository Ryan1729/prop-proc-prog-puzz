extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

static mut RNG: Option<StdRng> = None;

use std::process::Command;
use std::fs::File;
use std::env;
use std::path::Path;
use std::io::Write;

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

    println!("{}", func_str);

    let puzzle_name: String = "puzzle_".to_string() + seed.to_string().as_ref();

    {
        let output = Command::new("cargo")
            .arg("new")
            .arg(&puzzle_name)
            .output()
            .expect("cargo does not seem to be installed and on the PATH");

        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    ls();

    cd(&puzzle_name);

    ls();

    {
        let output = Command::new("mkdir")
            .arg("-p")
            .arg("tests")
            .output()
            .expect("mkdir does not seem to be installed and on the PATH");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    cd("tests");

    let test_filename = puzzle_name.to_string() + ".rs";

    let mut test_file = File::create(&test_filename).unwrap();

    write!(test_file,
           r"
    extern crate {puzzle_name};

    {func_str}

    #[test]
    fn test_true() {{
        assert_eq!(fill_me_in(true), {puzzle_name}::fill_me_in(true));
    }}
    #[test]
    fn test_false() {{
        assert_eq!(fill_me_in(false), {puzzle_name}::fill_me_in(false));
    }}
    ",
           puzzle_name = puzzle_name,
           func_str = func_str)
        .unwrap();


    {
        let output = Command::new("cat")
            .arg(&test_filename)
            .output()
            .expect("cat does not seem to be installed and on the PATH");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    cd("..");

    ls();

    cd("src");

    let mut template_file = File::create("lib.rs").unwrap();

    write!(template_file, "{}", template("unimplemented!();")).unwrap();
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

fn ls() {
    let output = Command::new("ls")
        .output()
        .expect("ls does not seem to be installed and on the PATH");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
