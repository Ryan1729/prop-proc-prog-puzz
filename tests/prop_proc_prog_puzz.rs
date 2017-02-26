#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate prop_proc_prog_puzz;

fn fill_me_in(b: bool) -> bool {
    b
}

#[cfg(test)]
quickcheck! {
    fn prop(input: bool) -> () {
        let expected = fill_me_in(input);
        let recieved = prop_proc_prog_puzz::fill_me_in(input);

        if expected != recieved {
          panic!("given an input of ({input}) \
          we expeceted a result of ({expected}) \
          but we recieved ({recieved}) instead!",
           input=input, expected=expected, recieved=recieved)
        }

    }
}
