// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

pub(crate) fn snake_to_pascal_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    // Start with capitalization
    let mut capitalize_next_char = true;
    for c in s.chars() {
        match (capitalize_next_char, c) {
            (_, '_') => {
                capitalize_next_char = true;
            }
            (true, c @ 'a'..='z') => {
                out.push((c as u8 - b'a' + b'A') as char);
                capitalize_next_char = false;
            }
            (_, c) => {
                out.push(c);
                capitalize_next_char = false;
            }
        }
    }
    // Weird corner case: `s` was only composed of underscores
    if out.is_empty() {
        out.push('_');
    }
    out
}

#[cfg(test)]
#[test]
fn test_snake_to_pascal_case() {
    assert_eq!(snake_to_pascal_case("a"), "A");
    assert_eq!(snake_to_pascal_case("#a!"), "#a!");
    assert_eq!(snake_to_pascal_case("a_!b"), "A!b");
    assert_eq!(snake_to_pascal_case("aa"), "Aa");
    assert_eq!(snake_to_pascal_case("a_b"), "AB");
    assert_eq!(snake_to_pascal_case("aa_bb_cCc"), "AaBbCCc");
    assert_eq!(snake_to_pascal_case("_a"), "A");
    assert_eq!(snake_to_pascal_case("_1_2b_3C"), "12b3C");
    assert_eq!(snake_to_pascal_case("_"), "_");
    assert_eq!(snake_to_pascal_case("___"), "_");
    assert_eq!(snake_to_pascal_case("__a__b__"), "AB");
    assert_eq!(snake_to_pascal_case("a_b_"), "AB");
}
