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
fn mul() {
    assert_eq!(test_eval("12*4"), 48);
    assert_eq!(test_eval("10*-5"), -50);
    assert_eq!(test_eval("-10*5"), -50);
    assert_eq!(test_eval("10*10*10"), 1000);
}

#[test]
#[wasm_bindgen_test]
fn div() {
    assert_eq!(test_eval("12/4"), 3);
    assert_eq!(test_eval("12/3/2"), 2);
    assert_eq!(test_eval("10/-5"), -2);
    assert_eq!(test_eval("-10/5"), -2);
}

#[test]
#[wasm_bindgen_test]
fn log_not() {
    assert_eq!(test_eval("!0"), 1);
    assert_eq!(test_eval("!0+!0+!0"), 3);
    assert_eq!(test_eval("!1"), 0);
    assert_eq!(test_eval("!-113"), 0);
}

#[test]
#[wasm_bindgen_test]
fn bitwise_or() {
    assert_eq!(test_eval("1|2"), 3);
    assert_eq!(test_eval("3|6"), 7);
    assert_eq!(test_eval("-1|42"), -1);
}

#[test]
#[wasm_bindgen_test]
fn bitwise_and() {
    assert_eq!(test_eval("1&3"), 1);
    assert_eq!(test_eval("3&6"), 2);
    assert_eq!(test_eval("-1&42"), 42);
}

#[test]
#[wasm_bindgen_test]
fn bitwise_xor() {
    assert_eq!(test_eval("3^2"), 1);
    assert_eq!(test_eval("-1^42"), -43);
}

#[test]
#[wasm_bindgen_test]
fn lsh() {
    assert_eq!(test_eval("1<<2"), 4);
    assert_eq!(test_eval("-1<<3"), -8);
}

#[test]
#[wasm_bindgen_test]
fn rsh() {
    assert_eq!(test_eval("8>>3"), 1);
    assert_eq!(test_eval("24>>3"), 3);
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