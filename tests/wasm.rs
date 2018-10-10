#![cfg(target_arch = "wasm32")]

use wp_issue_398;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(wp_issue_398::two(), 2);
}
