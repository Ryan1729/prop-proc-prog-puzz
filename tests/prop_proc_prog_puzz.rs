extern crate prop_proc_prog_puzz;

#[test]
fn test_true() {
    assert_eq!(false, prop_proc_prog_puzz::fill_me_in(true));
}
#[test]
fn test_false() {
    assert_eq!(true, prop_proc_prog_puzz::fill_me_in(false));
}
