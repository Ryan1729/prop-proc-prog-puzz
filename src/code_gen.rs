use rand::{Rng, StdRng};

pub fn gen(rng: &mut StdRng, input_type: &PuzzleType, output_type: &PuzzleType) -> String {
    let mut generator = CodeGenerator {
        rng: rng,
        input_type: input_type,
        output_type: output_type,
        count: 0,
        maximum: 8,
    };

    generator.gen()
}

struct CodeGenerator<'a> {
    rng: &'a mut StdRng,
    input_type: &'a PuzzleType<'a>,
    output_type: &'a PuzzleType<'a>,
    count: u8,
    maximum: u8,
}

impl<'a> CodeGenerator<'a> {
    pub fn gen(&mut self) -> String {
        if self.input_type.is_integer {
            self.int_input_gen()
        } else if self.input_type.is_partial_eq {
            self.partial_eq_gen()
        } else {
            type_expr(self.rng, self.output_type)
        }
    }


    fn int_input_gen(&mut self) -> String {
        if self.count >= self.maximum {
            return type_expr(self.rng, self.output_type);
        }

        self.count += 1;

        match self.rng.gen::<u8>() % 8 {
            0 => type_expr(self.rng, self.output_type),
            1 => {
                format!("if input == {input_type_expr} {{
                    {expr1}
                }} else {{
                    {expr2}
                }}",
                input_type_expr = type_expr(self.rng, self.input_type),
                expr1 = self.int_input_gen(),
                expr2 = self.int_input_gen(),
            )
            }
            2 => {
                format!("if input >= {input_type_expr} {{
                    {expr1}
                }} else {{
                    {expr2}
                }}",
                input_type_expr = type_expr(self.rng, self.input_type),
                expr1 = self.int_input_gen(),
                expr2 = self.int_input_gen(),
            )
            }
            3 => {
                format!("if input > {input_type_expr} {{
                    {expr1}
                }} else {{
                    {expr2}
                }}",
                input_type_expr = type_expr(self.rng, self.input_type),
                expr1 = self.int_input_gen(),
                expr2 = self.int_input_gen(),
            )
            }
            4 => {
                format!("if input <= {input_type_expr} {{
                    {expr1}
                }} else {{
                    {expr2}
                }}",
                input_type_expr = type_expr(self.rng, self.input_type),
                expr1 = self.int_input_gen(),
                expr2 = self.int_input_gen(),
            )
            }
            5 => {
                format!("if input < {input_type_expr} {{
                    {expr1}
                }} else {{
                    {expr2}
                }}",
                input_type_expr = type_expr(self.rng, self.input_type),
                expr1 = self.int_input_gen(),
                expr2 = self.int_input_gen(),
            )
            }
            6 => {
                format!("if input != {input_type_expr} {{
                        {expr1}
                    }} else {{
                        {expr2}
                    }}",
                    input_type_expr = type_expr(self.rng, self.input_type),
                    expr1 = self.int_input_gen(),
                    expr2 = self.int_input_gen(),
                    )
            }
            //TODO expression that involves `input`
            //TODO boolean expressions
            _ => type_expr(self.rng, self.output_type),

        }
    }



    fn partial_eq_gen(&mut self) -> String {
        if self.count >= self.maximum {
            return type_expr(self.rng, self.output_type);
        }

        self.count += 1;

        match self.rng.gen::<u8>() % 3 {
            0 => type_expr(self.rng, self.output_type),
            1 => {
                format!("if input == {input_example} {{
                {expr1}
            }} else {{
                {expr2}
            }}",
            input_example =  type_expr(self.rng, self.input_type),
            expr1 = self.partial_eq_gen(),
            expr2 = self.partial_eq_gen(),
        )
            }
            _ => {
                format!("if input != {input_example} {{
                {expr1}
            }} else {{
                {expr2}
            }}",
            input_example =  type_expr(self.rng, self.input_type),
            expr1 = self.partial_eq_gen(),
            expr2 = self.partial_eq_gen(),
        )
            }
        }

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


fn type_expr(rng: &mut StdRng, t: &PuzzleType) -> String {
    if t.is_integer {
        int_expr(rng, t)
    } else {
        (t.example)(rng)
    }
}

pub struct PuzzleType<'a> {
    pub definition: String,
    pub name: String,
    pub arbitrary_impl: String,
    pub is_partial_eq: bool,
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
            is_partial_eq: PuzzleType::is_partial_eq(name),
            is_enum: false,
            built_in: true,
            example: example,
            is_integer: PuzzleType::is_integer(name),
        }
    }

    fn is_partial_eq(name: &str) -> bool {
        if PuzzleType::is_integer(name) {
            return true;
        }

        match name {
            "str" | "&str" | "String" | "char" | "f32" | "f64" => true,
            _ => false,
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
    is_partial_eq: true,
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
