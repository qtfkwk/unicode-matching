use std::collections::BTreeMap;

use unicode_matching::FindMatching;

fn add_extra_close<'a>(m: BTreeMap<&'a str, &'a str>) -> BTreeMap<&'a str, &'a str> {
    m.into_iter()
        .chain([("<", ">"), ("'", "'"), ("\"", "\"")].into_iter())
        .collect::<BTreeMap<_, _>>()
}

fn add_extra_open<'a>(m: BTreeMap<&'a str, &'a str>) -> BTreeMap<&'a str, &'a str> {
    m.into_iter()
        .chain([(">", "<"), ("'", "'"), ("\"", "\"")].into_iter())
        .collect::<BTreeMap<_, _>>()
}

fn add_extra_match<'a>(m: BTreeMap<&'a str, &'a str>) -> BTreeMap<&'a str, &'a str> {
    add_extra_close(add_extra_open(m))
}

#[test]
fn simple_close() {
    let close = unicode_matching::close();

    assert_eq!(*close.get("(").unwrap(), ")");
    assert_eq!(*close.get("[").unwrap(), "]");
    assert_eq!(*close.get("{").unwrap(), "}");

    let close = add_extra_close(close);

    assert_eq!(*close.get("<").unwrap(), ">");
    assert_eq!(*close.get("'").unwrap(), "'");
    assert_eq!(*close.get("\"").unwrap(), "\"");
}

#[test]
fn simple_open() {
    let open = unicode_matching::open();

    assert_eq!(*open.get(")").unwrap(), "(");
    assert_eq!(*open.get("]").unwrap(), "[");
    assert_eq!(*open.get("}").unwrap(), "{");

    let open = add_extra_open(open);

    assert_eq!(*open.get(">").unwrap(), "<");
    assert_eq!(*open.get("'").unwrap(), "'");
    assert_eq!(*open.get("\"").unwrap(), "\"");
}

#[test]
fn simple_match() {
    let matching = unicode_matching::matching();

    assert_eq!(*matching.get("(").unwrap(), ")");
    assert_eq!(*matching.get("[").unwrap(), "]");
    assert_eq!(*matching.get("{").unwrap(), "}");
    assert_eq!(*matching.get(")").unwrap(), "(");
    assert_eq!(*matching.get("]").unwrap(), "[");
    assert_eq!(*matching.get("}").unwrap(), "{");

    let matching = add_extra_match(matching);

    assert_eq!(*matching.get("<").unwrap(), ">");
    assert_eq!(*matching.get(">").unwrap(), "<");
    assert_eq!(*matching.get("'").unwrap(), "'");
    assert_eq!(*matching.get("\"").unwrap(), "\"");
}

#[test]
fn find_matching() {
    let s = "fn main() {\n    println!(\"Hello!\");\n}";
    //       000000000011 11111111222222 2222333 333 33
    //       012345678901 23456789012345 6789012 345 67

    let s_matches = [(7, 8), (10, 36), (24, 33)]
        .into_iter()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect::<BTreeMap<_, _>>();

    let close = unicode_matching::close();
    let open = unicode_matching::open();

    for position in 0..=(s.len() + 1) {
        if let Some(other) = s_matches.get(&position) {
            assert_eq!(s.find_matching(position, &close, &open), *other);
        } else {
            assert_eq!(s.find_matching(position, &close, &open), position);
        }
    }
}
