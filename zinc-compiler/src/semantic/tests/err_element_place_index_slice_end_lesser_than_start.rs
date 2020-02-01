//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::PlaceError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = array[2 .. 1];
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 22),
        ElementError::Place(PlaceError::IndexSliceEndLesserThanStart(
            "1".to_owned(),
            "2".to_owned(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}