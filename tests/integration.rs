use std::collections::BTreeMap;

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
