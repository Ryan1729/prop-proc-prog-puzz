use rand::{Rng, StdRng};

pub fn gen(rng: &mut StdRng, input_type: &PuzzleType, output_type: &PuzzleType) -> String {
    match rng.gen::<u8>() % 4 {
        0 => (output_type.example)(rng),
        1 => {
            format!("if input == {input_example} {{
            {expr1}
        }} else {{
            {expr2}
        }}",
        input_example = (input_type.example)(rng),
        expr1 = enum_gen(rng, input_type, output_type),
        expr2 = enum_gen(rng, input_type, output_type),
    )
        }
        2 => {
            format!("if input == {input_type_expr} {{
            {expr1}
        }} else {{
            {expr2}
        }}",
        input_type_expr = int_expr(rng, input_type),
        expr1 = enum_gen(rng, input_type, output_type),
        expr2 = enum_gen(rng, input_type, output_type),
    )
        }
        //TODO expression that involves `input`
        // 3 =>
        _ => int_expr(rng, input_type),

    }
}

fn int_expr(rng: &mut StdRng, t: &PuzzleType) -> String {
    match rng.gen::<u8>() % 7 {
        0 => {
            format!("({} as u8).wrapping_add({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        1 => {
            format!("({} as u8).wrapping_sub({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        2 => {
            format!("({} as u8).wrapping_mul({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        3 => {
            format!("({} as u8).wrapping_div({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        4 => {
            format!("({} as u8).wrapping_rem({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        5 => {
            format!("({} as u8).wrapping_shl({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        _ => {
            format!("({} as u8).wrapping_shr({})",
                    (t.example)(rng),
                    (t.example)(rng))
        }
        //TODO subexpressions
    }

}

fn enum_gen(rng: &mut StdRng, input_type: &PuzzleType, output_type: &PuzzleType) -> String {
    if rng.gen::<bool>() {
        (output_type.example)(rng)
    } else {
        format!("if input == {input_example} {{
            {expr1}
        }} else {{
            {expr2}
        }}",
        input_example = (input_type.example)(rng),
        expr1 = enum_gen(rng, input_type, output_type),
        expr2 = enum_gen(rng, input_type, output_type),
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
    pub is_integer: bool,
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
            is_integer: PuzzleType::is_integer(name),
        }
    }

    fn is_integer(name: &str) -> bool {
        match name {
            "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => true,
            _ => false,
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
