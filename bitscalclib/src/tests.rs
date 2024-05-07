use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use crate::evaluate;

#[test]
#[wasm_bindgen_test]
fn add() {
    assert_eq!(test_eval("1+2"), 3);
    assert_eq!(test_eval("1+2+3"), 6);
    assert_eq!(test_eval("1+-3"), -2);
}

#[test]
#[wasm_bindgen_test]
fn sub() {
    assert_eq!(test_eval("3-1"), 2);
    assert_eq!(test_eval("1-5"), -4);
    assert_eq!(test_eval("1---1"), 0);
}


#[test]
#[wasm_bindgen_test]
fn parens() {
    assert_eq!(test_eval("(12 / 2) + 2"), 8);
    assert_eq!(test_eval("(((40))+2)"), 42);
}

fn test_eval(expr: &'static str) -> i64 {
    evaluate(expr).result.unwrap()
}