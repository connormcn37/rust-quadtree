//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate quadtree;
use quadtree::{Point, Rectangle, QuadTree};
/*
extern crate rand;
use rand::{thread_rng, Rng};
use rand::rngs::OsRng;
*/
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn create_qt() -> QuadTree {
    let qt = QuadTree::new(Rectangle::new(0.,0.,100.,100.), Some(4));
    qt
}

#[wasm_bindgen_test]
pub fn test_qt(){
    let mut qt = create_qt();
    //let mut rng = thread_rng(); //fails bc wasm_bindgen
    for i in 1..6 {
        qt.insert(Point { x: i as f64, y: i as f64});
    }

    assert_eq!(qt.is_divided(), true);
}