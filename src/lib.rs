use std::collections::BTreeMap;

/// Full set of matching brackets/quotes
pub const MATCHING: &[(&str, &str)] = &[
    ("(", ")"),
    ("[", "]"),
    ("{", "}"),
    ("«", "»"),
    ("‘", "’"),
    ("“", "”"),
    ("‹", "›"),
    ("⁅", "⁆"),
    ("⁽", "⁾"),
    ("₍", "₎"),
    ("⌈", "⌉"),
    ("⌊", "⌋"),
    ("〈", "〉"),
    ("❨", "❩"),
    ("❪", "❫"),
    ("❬", "❭"),
    ("❮", "❯"),
    ("❰", "❱"),
    ("❲", "❳"),
    ("❴", "❵"),
    ("⟅", "⟆"),
    ("⟦", "⟧"),
    ("⟨", "⟩"),
    ("⟪", "⟫"),
    ("⟬", "⟭"),
    ("⟮", "⟯"),
    ("⦃", "⦄"),
    ("⦅", "⦆"),
    ("⦇", "⦈"),
    ("⦉", "⦊"),
    ("⦋", "⦌"),
    ("⦍", "⦎"),
    ("⦏", "⦐"),
    ("⦑", "⦒"),
    ("⦓", "⦔"),
    ("⦕", "⦖"),
    ("⦗", "⦘"),
    ("⧘", "⧙"),
    ("⧚", "⧛"),
    ("⧼", "⧽"),
    ("⸂", "⸃"),
    ("⸄", "⸅"),
    ("⸉", "⸊"),
    ("⸌", "⸍"),
    ("⸜", "⸝"),
    ("⸠", "⸡"),
    ("⸢", "⸣"),
    ("⸤", "⸥"),
    ("⸦", "⸧"),
    ("⸨", "⸩"),
    ("⹕", "⹖"),
    ("⹗", "⹘"),
    ("⹙", "⹚"),
    ("⹛", "⹜"),
    ("〈", "〉"),
    ("《", "》"),
    ("「", "」"),
    ("『", "』"),
    ("【", "】"),
    ("〔", "〕"),
    ("〖", "〗"),
    ("〘", "〙"),
    ("〚", "〛"),
    ("﴾", "﴿"),
    ("︗", "︘"),
    ("︵", "︶"),
    ("︷", "︸"),
    ("︹", "︺"),
    ("︻", "︼"),
    ("︽", "︾"),
    ("︿", "﹀"),
    ("﹁", "﹂"),
    ("﹃", "﹄"),
    ("﹇", "﹈"),
    ("﹙", "﹚"),
    ("﹛", "﹜"),
    ("﹝", "﹞"),
    ("（", "）"),
    ("［", "］"),
    ("｛", "｝"),
    ("｟", "｠"),
    ("｢", "｣"),
    // Extra:
    ("<", ">"),
    ("'", "'"),
    ("\"", "\""),
];

/// Generate a [`BTreeMap`] with the matching close bracket/quote for each open bracket/quote
pub fn close() -> BTreeMap<&'static str, &'static str> {
    MATCHING.iter().cloned().collect()
}

/// Generate a [`BTreeMap`] with the matching open bracket/quote for each close bracket/quote
pub fn open() -> BTreeMap<&'static str, &'static str> {
    MATCHING
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/// Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets/quotes
pub fn matching() -> BTreeMap<&'static str, &'static str> {
    MATCHING
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}
