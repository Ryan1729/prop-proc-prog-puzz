use rand::{Rng, StdRng};

pub fn gen(rng: &mut StdRng, input_type: &PuzzleType, output_type: &PuzzleType) -> String {
    if rng.gen::<bool>() {
        (output_type.example)(rng)
    } else {
        format!("if input == {input_example} {{
            {expr1}
        }} else {{
            {expr2}
        }}",
        input_example = (input_type.example)(rng),
        expr1 = gen(rng, input_type, output_type),
        expr2 = gen(rng, input_type, output_type),
     )
    }
}

pub struct PuzzleType<'a> {
    pub definition: String,
    pub name: String,
    pub arbitrary_impl: String,
    pub is_enum: bool,
    pub built_in: bool,
    pub example: &'a Fn(&mut StdRng) -> String,
}

impl<'a> PuzzleType<'a> {
    pub fn built_in(name: &str, example: &'a Fn(&mut StdRng) -> String) -> Self {
        PuzzleType {
            definition: "".to_string(),
            name: name.to_string(),
            arbitrary_impl: "".to_string(),
            is_enum: false,
            built_in: true,
            example: example,
        }
    }
}

/*

built-in type example:

PuzzleType::built_in("usize")

enum example:

PuzzleType {
    definition: "#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z
}
"
        .to_string(),
    name: "Axis".to_string(),
    arbitrary_impl: "impl Arbitrary for Axis {
    fn arbitrary<G>(g: &mut G) -> Axis
                     where G: Gen
    {
        let r: u8 = g.gen_range(0, 3);
        match  r {
            0 => X,
            1 => Y,
            _ => Z,
        }
    }
}
"
        .to_string(),
    is_enum: true,
    built_in: false,
    example: |mut rng| {
        match rng.gen::<u8>() % 3 {
            0 => "X".to_string(),
            1 => "Y".to_string(),
            _ => "Z".to_string(),
        }
    }
};

//TODO have example closure use Gen
//TODO write macro to reduce redundancy of arbitrary_impl and example closure

*/
