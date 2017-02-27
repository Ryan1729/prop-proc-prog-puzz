extern crate rand;

use rand::{Rng, SeedableRng, StdRng};

static mut RNG: Option<StdRng> = None;

use std::process::Command;
use std::fs::File;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;

mod code_gen;

use code_gen::PuzzleType;

fn main() {
    let input_example: &Fn(&mut StdRng) -> String =
        &|rng: &mut StdRng| rng.gen::<bool>().to_string();

    let input_type = &PuzzleType::built_in("bool", input_example);

    let output_type = &PuzzleType {
        definition: "#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Colour {
    Red,
    Green,
    Blue
}
"
            .to_string(),
        name: "Colour".to_string(),
        arbitrary_impl: "impl Arbitrary for Colour {
    fn arbitrary<G>(g: &mut G) -> Colour
                     where G: Gen
    {
        let r: u8 = g.gen_range(0, 3);
        match r {
            0 => Red,
            1 => Green,
            _ => Blue,
        }
    }
}
"
            .to_string(),
        is_enum: true,
        built_in: false,
        example: &|mut rng| match rng.gen::<u8>() % 3 {
            0 => "Red".to_string(),
            1 => "Green".to_string(),
            _ => "Blue".to_string(),
        },
    };

    let seed: usize = 42;
    println!("seed: {}", seed);

    let rng;
    unsafe {
        if RNG.is_none() {
            let seed: &[_] = &[seed];
            RNG = Some(SeedableRng::from_seed(seed));
        }

        rng = RNG.as_mut().unwrap();
    }

    let puzzle_name: String = "puzzle_".to_string() + seed.to_string().as_ref();

    let code = code_gen::gen(rng, input_type, output_type);

    let func_str = template(&input_type.name, &output_type.name, &code);

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
    {input_import}
    {output_import}

    {enum_star}
    {func_str}

    #[cfg(test)]
    quickcheck! {{
        fn prop(input: {input_name}) -> () {{
            let expected = fill_me_in(input);
            let recieved = {puzzle_name}::fill_me_in(input);

            if expected != recieved {{
              panic!("given an input of ({{input:?}}) \
              we expeceted a result of ({{expected:?}}) \
              but we recieved ({{recieved:?}}) instead!",
               input=input, expected=expected, recieved=recieved)
           }}

       }}
   }}
    "##,
           puzzle_name = puzzle_name,
           input_name = input_type.name,
           func_str = func_str,
           input_import = import(&puzzle_name, &input_type),
           output_import = import(&puzzle_name, &output_type),
           enum_star=get_enum_star(&input_type, &output_type),
           )
        .unwrap();

    cd("../src");

    let mut template_file = File::create("lib.rs").unwrap();

    write!(template_file,
           "
{}
{enum_star}
{extra_input_definition}
{extra_output_definition}
extern crate quickcheck;

use quickcheck::{{Arbitrary,Gen}};
{input_arbitrary_impl}
{output_arbitrary_impl}
",
           template(&input_type.name, &output_type.name, "unimplemented!();"),
           extra_input_definition = input_type.definition,
           extra_output_definition = output_type.definition,
           enum_star=get_enum_star(&input_type, &output_type),
           input_arbitrary_impl = &input_type.arbitrary_impl,
           output_arbitrary_impl = &output_type.arbitrary_impl,)
        .unwrap();

    cd("..");

    let mut toml = OpenOptions::new().append(true).open("Cargo.toml").unwrap();

    write!(toml,
           "{}",
           r#"
    quickcheck = "0.3"
    "#)
        .unwrap();

    println!("wrote {}", &puzzle_name);
}

fn template(input_name: &str, output_name: &str, code: &str) -> String {
    format!(r"
#[allow(unused_variables)]
pub fn fill_me_in (input : {input_name}) -> {output_name} {{
    {}
}}
    ",
            code,
            input_name = input_name,
            output_name = output_name,
        )
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

fn import(module: &str, t: &PuzzleType) -> String {
    if t.built_in {
        "".to_string()
    } else {
        format!("use {}::{};", module, t.name)
    }
}

fn get_enum_star(t1: &PuzzleType, t2: &PuzzleType) -> String {
    let mut result = "".to_string();

    if t1.is_enum {
        result += format!("use {}::*;\n", t1.name).as_ref();
    }

    if t2.is_enum {
        result += format!("use {}::*;\n", t2.name).as_ref();
    }

    result
}
