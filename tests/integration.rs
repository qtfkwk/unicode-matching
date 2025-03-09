#[test]
fn simple_close() {
    let close = unicode_matching::close();

    assert_eq!(*close.get("(").unwrap(), ")");
    assert_eq!(*close.get("[").unwrap(), "]");
    assert_eq!(*close.get("<").unwrap(), ">");
    assert_eq!(*close.get("{").unwrap(), "}");
    assert_eq!(*close.get("'").unwrap(), "'");
    assert_eq!(*close.get("\"").unwrap(), "\"");
}

#[test]
fn simple_open() {
    let open = unicode_matching::open();

    assert_eq!(*open.get(")").unwrap(), "(");
    assert_eq!(*open.get("]").unwrap(), "[");
    assert_eq!(*open.get(">").unwrap(), "<");
    assert_eq!(*open.get("}").unwrap(), "{");
    assert_eq!(*open.get("'").unwrap(), "'");
    assert_eq!(*open.get("\"").unwrap(), "\"");
}

#[test]
fn simple_match() {
    let matching = unicode_matching::matching();

    assert_eq!(*matching.get("(").unwrap(), ")");
    assert_eq!(*matching.get("[").unwrap(), "]");
    assert_eq!(*matching.get("<").unwrap(), ">");
    assert_eq!(*matching.get("{").unwrap(), "}");
    assert_eq!(*matching.get(")").unwrap(), "(");
    assert_eq!(*matching.get("]").unwrap(), "[");
    assert_eq!(*matching.get(">").unwrap(), "<");
    assert_eq!(*matching.get("}").unwrap(), "{");
    assert_eq!(*matching.get("'").unwrap(), "'");
    assert_eq!(*matching.get("\"").unwrap(), "\"");
}
