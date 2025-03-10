use gstring::*;

use std::collections::BTreeMap;

/// Matching open/close brackets from `UnicodeData.txt`
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
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in `UnicodeData.txt`
*/
pub fn close() -> BTreeMap<&'static str, &'static str> {
    MATCHING.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in `UnicodeData.txt`
*/
pub fn open() -> BTreeMap<&'static str, &'static str> {
    MATCHING
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`UnicodeData.txt`
*/
pub fn matching() -> BTreeMap<&'static str, &'static str> {
    MATCHING
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Matching open/close brackets from `BidiBrackets.txt`
pub const BRACKETS: &[(&str, &str)] = &[
    ("(", ")"),
    ("[", "]"),
    ("{", "}"),
    ("༺", "༻"),
    ("༼", "༽"),
    ("᚛", "᚜"),
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
    ("⦍", "⦐"),
    ("⦏", "⦎"),
    ("⦑", "⦒"),
    ("⦓", "⦔"),
    ("⦕", "⦖"),
    ("⦗", "⦘"),
    ("⧘", "⧙"),
    ("⧚", "⧛"),
    ("⧼", "⧽"),
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
    ("﹙", "﹚"),
    ("﹛", "﹜"),
    ("﹝", "﹞"),
    ("（", "）"),
    ("［", "］"),
    ("｛", "｝"),
    ("｟", "｠"),
    ("｢", "｣"),
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in `BidiBrackets.txt`
*/
pub fn close_brackets() -> BTreeMap<&'static str, &'static str> {
    BRACKETS.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in `BidiBrackets.txt`
*/
pub fn open_brackets() -> BTreeMap<&'static str, &'static str> {
    BRACKETS
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`BidiBrackets.txt`
*/
pub fn matching_brackets() -> BTreeMap<&'static str, &'static str> {
    BRACKETS
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Matching open/close brackets from `BidiMirroring.txt`
pub const MIRRORING: &[(&str, &str)] = &[
    ("(", ")"),
    ("<", ">"),
    ("[", "]"),
    ("{", "}"),
    ("«", "»"),
    ("༺", "༻"),
    ("༼", "༽"),
    ("᚛", "᚜"),
    ("‹", "›"),
    ("⁅", "⁆"),
    ("⁽", "⁾"),
    ("₍", "₎"),
    ("∈", "∋"),
    ("∉", "∌"),
    ("∊", "∍"),
    ("∕", "⧵"),
    ("∟", "⯾"),
    ("∠", "⦣"),
    ("∡", "⦛"),
    ("∢", "⦠"),
    ("∤", "⫮"),
    ("∼", "∽"),
    ("≃", "⋍"),
    ("≅", "≌"),
    ("≒", "≓"),
    ("≔", "≕"),
    ("≤", "≥"),
    ("≦", "≧"),
    ("≨", "≩"),
    ("≪", "≫"),
    ("≮", "≯"),
    ("≰", "≱"),
    ("≲", "≳"),
    ("≴", "≵"),
    ("≶", "≷"),
    ("≸", "≹"),
    ("≺", "≻"),
    ("≼", "≽"),
    ("≾", "≿"),
    ("⊀", "⊁"),
    ("⊂", "⊃"),
    ("⊄", "⊅"),
    ("⊆", "⊇"),
    ("⊈", "⊉"),
    ("⊊", "⊋"),
    ("⊏", "⊐"),
    ("⊑", "⊒"),
    ("⊘", "⦸"),
    ("⊢", "⊣"),
    ("⊦", "⫞"),
    ("⊨", "⫤"),
    ("⊩", "⫣"),
    ("⊫", "⫥"),
    ("⊰", "⊱"),
    ("⊲", "⊳"),
    ("⊴", "⊵"),
    ("⊶", "⊷"),
    ("⊸", "⟜"),
    ("⋉", "⋊"),
    ("⋋", "⋌"),
    ("⋐", "⋑"),
    ("⋖", "⋗"),
    ("⋘", "⋙"),
    ("⋚", "⋛"),
    ("⋜", "⋝"),
    ("⋞", "⋟"),
    ("⋠", "⋡"),
    ("⋢", "⋣"),
    ("⋤", "⋥"),
    ("⋦", "⋧"),
    ("⋨", "⋩"),
    ("⋪", "⋫"),
    ("⋬", "⋭"),
    ("⋰", "⋱"),
    ("⋲", "⋺"),
    ("⋳", "⋻"),
    ("⋴", "⋼"),
    ("⋶", "⋽"),
    ("⋷", "⋾"),
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
    ("⟃", "⟄"),
    ("⟅", "⟆"),
    ("⟈", "⟉"),
    ("⟋", "⟍"),
    ("⟕", "⟖"),
    ("⟝", "⟞"),
    ("⟢", "⟣"),
    ("⟤", "⟥"),
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
    ("⦍", "⦐"),
    ("⦎", "⦏"),
    ("⦑", "⦒"),
    ("⦓", "⦔"),
    ("⦕", "⦖"),
    ("⦗", "⦘"),
    ("⦤", "⦥"),
    ("⦨", "⦩"),
    ("⦪", "⦫"),
    ("⦬", "⦭"),
    ("⦮", "⦯"),
    ("⧀", "⧁"),
    ("⧄", "⧅"),
    ("⧏", "⧐"),
    ("⧑", "⧒"),
    ("⧔", "⧕"),
    ("⧘", "⧙"),
    ("⧚", "⧛"),
    ("⧨", "⧩"),
    ("⧸", "⧹"),
    ("⧼", "⧽"),
    ("⨫", "⨬"),
    ("⨭", "⨮"),
    ("⨴", "⨵"),
    ("⨼", "⨽"),
    ("⩤", "⩥"),
    ("⩹", "⩺"),
    ("⩻", "⩼"),
    ("⩽", "⩾"),
    ("⩿", "⪀"),
    ("⪁", "⪂"),
    ("⪃", "⪄"),
    ("⪅", "⪆"),
    ("⪇", "⪈"),
    ("⪉", "⪊"),
    ("⪋", "⪌"),
    ("⪍", "⪎"),
    ("⪏", "⪐"),
    ("⪑", "⪒"),
    ("⪓", "⪔"),
    ("⪕", "⪖"),
    ("⪗", "⪘"),
    ("⪙", "⪚"),
    ("⪛", "⪜"),
    ("⪝", "⪞"),
    ("⪟", "⪠"),
    ("⪡", "⪢"),
    ("⪦", "⪧"),
    ("⪨", "⪩"),
    ("⪪", "⪫"),
    ("⪬", "⪭"),
    ("⪯", "⪰"),
    ("⪱", "⪲"),
    ("⪳", "⪴"),
    ("⪵", "⪶"),
    ("⪷", "⪸"),
    ("⪹", "⪺"),
    ("⪻", "⪼"),
    ("⪽", "⪾"),
    ("⪿", "⫀"),
    ("⫁", "⫂"),
    ("⫃", "⫄"),
    ("⫅", "⫆"),
    ("⫇", "⫈"),
    ("⫉", "⫊"),
    ("⫋", "⫌"),
    ("⫍", "⫎"),
    ("⫏", "⫐"),
    ("⫑", "⫒"),
    ("⫓", "⫔"),
    ("⫕", "⫖"),
    ("⫬", "⫭"),
    ("⫷", "⫸"),
    ("⫹", "⫺"),
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
    ("﹙", "﹚"),
    ("﹛", "﹜"),
    ("﹝", "﹞"),
    ("﹤", "﹥"),
    ("（", "）"),
    ("＜", "＞"),
    ("［", "］"),
    ("｛", "｝"),
    ("｟", "｠"),
    ("｢", "｣"),
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in
`BidiMirroring.txt`
*/
pub fn close_mirroring() -> BTreeMap<&'static str, &'static str> {
    MIRRORING.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in
`BidiMirroring.txt`
*/
pub fn open_mirroring() -> BTreeMap<&'static str, &'static str> {
    MIRRORING
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`BidiMirroring.txt`
*/
pub fn matching_mirroring() -> BTreeMap<&'static str, &'static str> {
    MIRRORING
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Matching open/close brackets from `UnicodeData.txt` and `BidiBrackets.txt`
pub const BRACKETS_MATCHING: &[(&str, &str)] = &[
    ("(", ")"),
    ("[", "]"),
    ("{", "}"),
    ("«", "»"),
    ("༺", "༻"),
    ("༼", "༽"),
    ("᚛", "᚜"),
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
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in `UnicodeData.txt`
and `BidiBrackets.txt`
*/
pub fn close_brackets_matching() -> BTreeMap<&'static str, &'static str> {
    BRACKETS_MATCHING.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in `UnicodeData.txt`
and `BidiBrackets.txt`
*/
pub fn open_brackets_matching() -> BTreeMap<&'static str, &'static str> {
    BRACKETS_MATCHING
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`UnicodeData.txt` and `BidiBrackets.txt`
*/
pub fn matching_brackets_matching() -> BTreeMap<&'static str, &'static str> {
    BRACKETS_MATCHING
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Matching open/close brackets from `BidiMirroring.txt` and `BidiBrackets.txt`
pub const BRACKETS_MIRRORING: &[(&str, &str)] = &[
    ("(", ")"),
    ("<", ">"),
    ("[", "]"),
    ("{", "}"),
    ("«", "»"),
    ("༺", "༻"),
    ("༼", "༽"),
    ("᚛", "᚜"),
    ("‹", "›"),
    ("⁅", "⁆"),
    ("⁽", "⁾"),
    ("₍", "₎"),
    ("∈", "∋"),
    ("∉", "∌"),
    ("∊", "∍"),
    ("∕", "⧵"),
    ("∟", "⯾"),
    ("∠", "⦣"),
    ("∡", "⦛"),
    ("∢", "⦠"),
    ("∤", "⫮"),
    ("∼", "∽"),
    ("≃", "⋍"),
    ("≅", "≌"),
    ("≒", "≓"),
    ("≔", "≕"),
    ("≤", "≥"),
    ("≦", "≧"),
    ("≨", "≩"),
    ("≪", "≫"),
    ("≮", "≯"),
    ("≰", "≱"),
    ("≲", "≳"),
    ("≴", "≵"),
    ("≶", "≷"),
    ("≸", "≹"),
    ("≺", "≻"),
    ("≼", "≽"),
    ("≾", "≿"),
    ("⊀", "⊁"),
    ("⊂", "⊃"),
    ("⊄", "⊅"),
    ("⊆", "⊇"),
    ("⊈", "⊉"),
    ("⊊", "⊋"),
    ("⊏", "⊐"),
    ("⊑", "⊒"),
    ("⊘", "⦸"),
    ("⊢", "⊣"),
    ("⊦", "⫞"),
    ("⊨", "⫤"),
    ("⊩", "⫣"),
    ("⊫", "⫥"),
    ("⊰", "⊱"),
    ("⊲", "⊳"),
    ("⊴", "⊵"),
    ("⊶", "⊷"),
    ("⊸", "⟜"),
    ("⋉", "⋊"),
    ("⋋", "⋌"),
    ("⋐", "⋑"),
    ("⋖", "⋗"),
    ("⋘", "⋙"),
    ("⋚", "⋛"),
    ("⋜", "⋝"),
    ("⋞", "⋟"),
    ("⋠", "⋡"),
    ("⋢", "⋣"),
    ("⋤", "⋥"),
    ("⋦", "⋧"),
    ("⋨", "⋩"),
    ("⋪", "⋫"),
    ("⋬", "⋭"),
    ("⋰", "⋱"),
    ("⋲", "⋺"),
    ("⋳", "⋻"),
    ("⋴", "⋼"),
    ("⋶", "⋽"),
    ("⋷", "⋾"),
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
    ("⟃", "⟄"),
    ("⟅", "⟆"),
    ("⟈", "⟉"),
    ("⟋", "⟍"),
    ("⟕", "⟖"),
    ("⟝", "⟞"),
    ("⟢", "⟣"),
    ("⟤", "⟥"),
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
    ("⦍", "⦐"),
    ("⦎", "⦏"),
    ("⦏", "⦎"),
    ("⦑", "⦒"),
    ("⦓", "⦔"),
    ("⦕", "⦖"),
    ("⦗", "⦘"),
    ("⦤", "⦥"),
    ("⦨", "⦩"),
    ("⦪", "⦫"),
    ("⦬", "⦭"),
    ("⦮", "⦯"),
    ("⧀", "⧁"),
    ("⧄", "⧅"),
    ("⧏", "⧐"),
    ("⧑", "⧒"),
    ("⧔", "⧕"),
    ("⧘", "⧙"),
    ("⧚", "⧛"),
    ("⧨", "⧩"),
    ("⧸", "⧹"),
    ("⧼", "⧽"),
    ("⨫", "⨬"),
    ("⨭", "⨮"),
    ("⨴", "⨵"),
    ("⨼", "⨽"),
    ("⩤", "⩥"),
    ("⩹", "⩺"),
    ("⩻", "⩼"),
    ("⩽", "⩾"),
    ("⩿", "⪀"),
    ("⪁", "⪂"),
    ("⪃", "⪄"),
    ("⪅", "⪆"),
    ("⪇", "⪈"),
    ("⪉", "⪊"),
    ("⪋", "⪌"),
    ("⪍", "⪎"),
    ("⪏", "⪐"),
    ("⪑", "⪒"),
    ("⪓", "⪔"),
    ("⪕", "⪖"),
    ("⪗", "⪘"),
    ("⪙", "⪚"),
    ("⪛", "⪜"),
    ("⪝", "⪞"),
    ("⪟", "⪠"),
    ("⪡", "⪢"),
    ("⪦", "⪧"),
    ("⪨", "⪩"),
    ("⪪", "⪫"),
    ("⪬", "⪭"),
    ("⪯", "⪰"),
    ("⪱", "⪲"),
    ("⪳", "⪴"),
    ("⪵", "⪶"),
    ("⪷", "⪸"),
    ("⪹", "⪺"),
    ("⪻", "⪼"),
    ("⪽", "⪾"),
    ("⪿", "⫀"),
    ("⫁", "⫂"),
    ("⫃", "⫄"),
    ("⫅", "⫆"),
    ("⫇", "⫈"),
    ("⫉", "⫊"),
    ("⫋", "⫌"),
    ("⫍", "⫎"),
    ("⫏", "⫐"),
    ("⫑", "⫒"),
    ("⫓", "⫔"),
    ("⫕", "⫖"),
    ("⫬", "⫭"),
    ("⫷", "⫸"),
    ("⫹", "⫺"),
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
    ("﹙", "﹚"),
    ("﹛", "﹜"),
    ("﹝", "﹞"),
    ("﹤", "﹥"),
    ("（", "）"),
    ("＜", "＞"),
    ("［", "］"),
    ("｛", "｝"),
    ("｟", "｠"),
    ("｢", "｣"),
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in
`BidiMirroring.txt` and `BidiBrackets.txt`
*/
pub fn close_brackets_mirroring() -> BTreeMap<&'static str, &'static str> {
    BRACKETS_MIRRORING.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in
`BidiMirroring.txt` and `BidiBrackets.txt`
*/
pub fn open_brackets_mirroring() -> BTreeMap<&'static str, &'static str> {
    BRACKETS_MIRRORING
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`BidiMirroring.txt` and `BidiBrackets.txt`
*/
pub fn matching_brackets_mirroring() -> BTreeMap<&'static str, &'static str> {
    BRACKETS_MIRRORING
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Matching open/close brackets from `UnicodeData.txt` and `BidiMirroring.txt`
pub const MIRRORING_MATCHING: &[(&str, &str)] = &[
    ("(", ")"),
    ("<", ">"),
    ("[", "]"),
    ("{", "}"),
    ("«", "»"),
    ("༺", "༻"),
    ("༼", "༽"),
    ("᚛", "᚜"),
    ("‘", "’"),
    ("“", "”"),
    ("‹", "›"),
    ("⁅", "⁆"),
    ("⁽", "⁾"),
    ("₍", "₎"),
    ("∈", "∋"),
    ("∉", "∌"),
    ("∊", "∍"),
    ("∕", "⧵"),
    ("∟", "⯾"),
    ("∠", "⦣"),
    ("∡", "⦛"),
    ("∢", "⦠"),
    ("∤", "⫮"),
    ("∼", "∽"),
    ("≃", "⋍"),
    ("≅", "≌"),
    ("≒", "≓"),
    ("≔", "≕"),
    ("≤", "≥"),
    ("≦", "≧"),
    ("≨", "≩"),
    ("≪", "≫"),
    ("≮", "≯"),
    ("≰", "≱"),
    ("≲", "≳"),
    ("≴", "≵"),
    ("≶", "≷"),
    ("≸", "≹"),
    ("≺", "≻"),
    ("≼", "≽"),
    ("≾", "≿"),
    ("⊀", "⊁"),
    ("⊂", "⊃"),
    ("⊄", "⊅"),
    ("⊆", "⊇"),
    ("⊈", "⊉"),
    ("⊊", "⊋"),
    ("⊏", "⊐"),
    ("⊑", "⊒"),
    ("⊘", "⦸"),
    ("⊢", "⊣"),
    ("⊦", "⫞"),
    ("⊨", "⫤"),
    ("⊩", "⫣"),
    ("⊫", "⫥"),
    ("⊰", "⊱"),
    ("⊲", "⊳"),
    ("⊴", "⊵"),
    ("⊶", "⊷"),
    ("⊸", "⟜"),
    ("⋉", "⋊"),
    ("⋋", "⋌"),
    ("⋐", "⋑"),
    ("⋖", "⋗"),
    ("⋘", "⋙"),
    ("⋚", "⋛"),
    ("⋜", "⋝"),
    ("⋞", "⋟"),
    ("⋠", "⋡"),
    ("⋢", "⋣"),
    ("⋤", "⋥"),
    ("⋦", "⋧"),
    ("⋨", "⋩"),
    ("⋪", "⋫"),
    ("⋬", "⋭"),
    ("⋰", "⋱"),
    ("⋲", "⋺"),
    ("⋳", "⋻"),
    ("⋴", "⋼"),
    ("⋶", "⋽"),
    ("⋷", "⋾"),
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
    ("⟃", "⟄"),
    ("⟅", "⟆"),
    ("⟈", "⟉"),
    ("⟋", "⟍"),
    ("⟕", "⟖"),
    ("⟝", "⟞"),
    ("⟢", "⟣"),
    ("⟤", "⟥"),
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
    ("⦎", "⦏"),
    ("⦏", "⦐"),
    ("⦑", "⦒"),
    ("⦓", "⦔"),
    ("⦕", "⦖"),
    ("⦗", "⦘"),
    ("⦤", "⦥"),
    ("⦨", "⦩"),
    ("⦪", "⦫"),
    ("⦬", "⦭"),
    ("⦮", "⦯"),
    ("⧀", "⧁"),
    ("⧄", "⧅"),
    ("⧏", "⧐"),
    ("⧑", "⧒"),
    ("⧔", "⧕"),
    ("⧘", "⧙"),
    ("⧚", "⧛"),
    ("⧨", "⧩"),
    ("⧸", "⧹"),
    ("⧼", "⧽"),
    ("⨫", "⨬"),
    ("⨭", "⨮"),
    ("⨴", "⨵"),
    ("⨼", "⨽"),
    ("⩤", "⩥"),
    ("⩹", "⩺"),
    ("⩻", "⩼"),
    ("⩽", "⩾"),
    ("⩿", "⪀"),
    ("⪁", "⪂"),
    ("⪃", "⪄"),
    ("⪅", "⪆"),
    ("⪇", "⪈"),
    ("⪉", "⪊"),
    ("⪋", "⪌"),
    ("⪍", "⪎"),
    ("⪏", "⪐"),
    ("⪑", "⪒"),
    ("⪓", "⪔"),
    ("⪕", "⪖"),
    ("⪗", "⪘"),
    ("⪙", "⪚"),
    ("⪛", "⪜"),
    ("⪝", "⪞"),
    ("⪟", "⪠"),
    ("⪡", "⪢"),
    ("⪦", "⪧"),
    ("⪨", "⪩"),
    ("⪪", "⪫"),
    ("⪬", "⪭"),
    ("⪯", "⪰"),
    ("⪱", "⪲"),
    ("⪳", "⪴"),
    ("⪵", "⪶"),
    ("⪷", "⪸"),
    ("⪹", "⪺"),
    ("⪻", "⪼"),
    ("⪽", "⪾"),
    ("⪿", "⫀"),
    ("⫁", "⫂"),
    ("⫃", "⫄"),
    ("⫅", "⫆"),
    ("⫇", "⫈"),
    ("⫉", "⫊"),
    ("⫋", "⫌"),
    ("⫍", "⫎"),
    ("⫏", "⫐"),
    ("⫑", "⫒"),
    ("⫓", "⫔"),
    ("⫕", "⫖"),
    ("⫬", "⫭"),
    ("⫷", "⫸"),
    ("⫹", "⫺"),
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
    ("﹤", "﹥"),
    ("（", "）"),
    ("＜", "＞"),
    ("［", "］"),
    ("｛", "｝"),
    ("｟", "｠"),
    ("｢", "｣"),
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in `UnicodeData.txt`
and `BidiMirroring.txt`
*/
pub fn close_mirroring_matching() -> BTreeMap<&'static str, &'static str> {
    MIRRORING_MATCHING.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in `UnicodeData.txt`
and `BidiMirroring.txt`
*/
pub fn open_mirroring_matching() -> BTreeMap<&'static str, &'static str> {
    MIRRORING_MATCHING
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`UnicodeData.txt` and `BidiMirroring.txt`
*/
pub fn matching_mirroring_matching() -> BTreeMap<&'static str, &'static str> {
    MIRRORING_MATCHING
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Matching open/close brackets from `UnicodeData.txt`, `BidiBrackets.txt`, and `BidiMirroring.txt`
pub const ALL: &[(&str, &str)] = &[
    ("(", ")"),
    ("<", ">"),
    ("[", "]"),
    ("{", "}"),
    ("«", "»"),
    ("༺", "༻"),
    ("༼", "༽"),
    ("᚛", "᚜"),
    ("‘", "’"),
    ("“", "”"),
    ("‹", "›"),
    ("⁅", "⁆"),
    ("⁽", "⁾"),
    ("₍", "₎"),
    ("∈", "∋"),
    ("∉", "∌"),
    ("∊", "∍"),
    ("∕", "⧵"),
    ("∟", "⯾"),
    ("∠", "⦣"),
    ("∡", "⦛"),
    ("∢", "⦠"),
    ("∤", "⫮"),
    ("∼", "∽"),
    ("≃", "⋍"),
    ("≅", "≌"),
    ("≒", "≓"),
    ("≔", "≕"),
    ("≤", "≥"),
    ("≦", "≧"),
    ("≨", "≩"),
    ("≪", "≫"),
    ("≮", "≯"),
    ("≰", "≱"),
    ("≲", "≳"),
    ("≴", "≵"),
    ("≶", "≷"),
    ("≸", "≹"),
    ("≺", "≻"),
    ("≼", "≽"),
    ("≾", "≿"),
    ("⊀", "⊁"),
    ("⊂", "⊃"),
    ("⊄", "⊅"),
    ("⊆", "⊇"),
    ("⊈", "⊉"),
    ("⊊", "⊋"),
    ("⊏", "⊐"),
    ("⊑", "⊒"),
    ("⊘", "⦸"),
    ("⊢", "⊣"),
    ("⊦", "⫞"),
    ("⊨", "⫤"),
    ("⊩", "⫣"),
    ("⊫", "⫥"),
    ("⊰", "⊱"),
    ("⊲", "⊳"),
    ("⊴", "⊵"),
    ("⊶", "⊷"),
    ("⊸", "⟜"),
    ("⋉", "⋊"),
    ("⋋", "⋌"),
    ("⋐", "⋑"),
    ("⋖", "⋗"),
    ("⋘", "⋙"),
    ("⋚", "⋛"),
    ("⋜", "⋝"),
    ("⋞", "⋟"),
    ("⋠", "⋡"),
    ("⋢", "⋣"),
    ("⋤", "⋥"),
    ("⋦", "⋧"),
    ("⋨", "⋩"),
    ("⋪", "⋫"),
    ("⋬", "⋭"),
    ("⋰", "⋱"),
    ("⋲", "⋺"),
    ("⋳", "⋻"),
    ("⋴", "⋼"),
    ("⋶", "⋽"),
    ("⋷", "⋾"),
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
    ("⟃", "⟄"),
    ("⟅", "⟆"),
    ("⟈", "⟉"),
    ("⟋", "⟍"),
    ("⟕", "⟖"),
    ("⟝", "⟞"),
    ("⟢", "⟣"),
    ("⟤", "⟥"),
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
    ("⦎", "⦏"),
    ("⦏", "⦐"),
    ("⦑", "⦒"),
    ("⦓", "⦔"),
    ("⦕", "⦖"),
    ("⦗", "⦘"),
    ("⦤", "⦥"),
    ("⦨", "⦩"),
    ("⦪", "⦫"),
    ("⦬", "⦭"),
    ("⦮", "⦯"),
    ("⧀", "⧁"),
    ("⧄", "⧅"),
    ("⧏", "⧐"),
    ("⧑", "⧒"),
    ("⧔", "⧕"),
    ("⧘", "⧙"),
    ("⧚", "⧛"),
    ("⧨", "⧩"),
    ("⧸", "⧹"),
    ("⧼", "⧽"),
    ("⨫", "⨬"),
    ("⨭", "⨮"),
    ("⨴", "⨵"),
    ("⨼", "⨽"),
    ("⩤", "⩥"),
    ("⩹", "⩺"),
    ("⩻", "⩼"),
    ("⩽", "⩾"),
    ("⩿", "⪀"),
    ("⪁", "⪂"),
    ("⪃", "⪄"),
    ("⪅", "⪆"),
    ("⪇", "⪈"),
    ("⪉", "⪊"),
    ("⪋", "⪌"),
    ("⪍", "⪎"),
    ("⪏", "⪐"),
    ("⪑", "⪒"),
    ("⪓", "⪔"),
    ("⪕", "⪖"),
    ("⪗", "⪘"),
    ("⪙", "⪚"),
    ("⪛", "⪜"),
    ("⪝", "⪞"),
    ("⪟", "⪠"),
    ("⪡", "⪢"),
    ("⪦", "⪧"),
    ("⪨", "⪩"),
    ("⪪", "⪫"),
    ("⪬", "⪭"),
    ("⪯", "⪰"),
    ("⪱", "⪲"),
    ("⪳", "⪴"),
    ("⪵", "⪶"),
    ("⪷", "⪸"),
    ("⪹", "⪺"),
    ("⪻", "⪼"),
    ("⪽", "⪾"),
    ("⪿", "⫀"),
    ("⫁", "⫂"),
    ("⫃", "⫄"),
    ("⫅", "⫆"),
    ("⫇", "⫈"),
    ("⫉", "⫊"),
    ("⫋", "⫌"),
    ("⫍", "⫎"),
    ("⫏", "⫐"),
    ("⫑", "⫒"),
    ("⫓", "⫔"),
    ("⫕", "⫖"),
    ("⫬", "⫭"),
    ("⫷", "⫸"),
    ("⫹", "⫺"),
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
    ("﹤", "﹥"),
    ("（", "）"),
    ("＜", "＞"),
    ("［", "］"),
    ("｛", "｝"),
    ("｟", "｠"),
    ("｢", "｣"),
];

/**
Generate a [`BTreeMap`] with the matching close bracket for each open bracket in `UnicodeData.txt`,
`BidiBrackets.txt`, and `BidiMirroring.txt`
*/
pub fn close_all() -> BTreeMap<&'static str, &'static str> {
    ALL.iter().cloned().collect()
}

/**
Generate a [`BTreeMap`] with the matching open bracket for each close bracket in `UnicodeData.txt`,
`BidiBrackets.txt`, and `BidiMirroring.txt`
*/
pub fn open_all() -> BTreeMap<&'static str, &'static str> {
    ALL
        .iter()
        .cloned()
        .map(|(open, close)| (close, open))
        .collect()
}

/**
Generate a [`BTreeMap`] with an entry for each pair of opening and closing brackets in
`UnicodeData.txt`, `BidiBrackets.txt`, and `BidiMirroring.txt`
*/
pub fn matching_all() -> BTreeMap<&'static str, &'static str> {
    ALL
        .iter()
        .cloned()
        .flat_map(|(open, close)| [(open, close), (close, open)])
        .collect()
}

/// Trait to provide the [`FindMatching::find_matching`] method
pub trait FindMatching {
    /**
    Find the index of the matching open or close grapheme for the grapheme at `position`, given the
    matching `closing` and `opening` brackets

    If the grapheme at `position` is not an opening or closing grapheme, `position` is greater or
    equal to the length of `self`, or the algorithm fails to find the matching grapheme (e.g.
    matching graphemes are unbalanced), the given `position` is returned.
    */
    fn find_matching(
        &self,
        position: usize,
        closing: &BTreeMap<&str, &str>,
        opening: &BTreeMap<&str, &str>,
    ) -> usize;
}

impl FindMatching for str {
    /**
    Find the index of the matching open or close grapheme for the grapheme at `position`, given the
    matching `closing` and `opening` brackets

    If the grapheme at `position` is not an opening or closing grapheme, `position` is greater or
    equal to the length of `self`, or the algorithm fails to find the matching grapheme (e.g.
    matching graphemes are unbalanced), the given `position` is returned.
    */
    fn find_matching(&self,
        position: usize,
        closing: &BTreeMap<&str, &str>,
        opening: &BTreeMap<&str, &str>,
    ) -> usize {
        self.gstring().find_matching(position, closing, opening)
    }
}

impl FindMatching for GString {
    /**
    Find the index of the matching open or close grapheme for the grapheme at `position`, given the
    matching `closing` and `opening` brackets

    If the grapheme at `position` is not an opening or closing grapheme, `position` is greater or
    equal to the length of `self`, or the algorithm fails to find the matching grapheme (e.g.
    matching graphemes are unbalanced), the given `position` is returned.
    */
    fn find_matching(&self,
        position: usize,
        closing: &BTreeMap<&str, &str>,
        opening: &BTreeMap<&str, &str>,
    ) -> usize {
        let v = self.graphemes();

        if position >= v.len() {
            return position;
        }

        let close = closing.get(&v[position].as_str());
        let open = opening.get(&v[position].as_str());

        if close.is_none() && open.is_none() {
            return position;
        }

        let mut stack: Vec<(usize, &String)> = vec![];
        for (i, g) in v.iter().enumerate() {
            if let Some(open) = opening.get(&g.as_str()) {
                // g is a close
                if let Some(prev) = stack.pop() {
                    // There is a previous open
                    if prev.1 == open {
                        // This close matches the previous open

                        if prev.0 == position {
                            // Return this close's position if the open is at the position
                            return i;
                        } else if i == position {
                            // Return the previous open's position if this close is at the position
                            return prev.0;
                        }
                    } else if closing.contains_key(&g.as_str()) {
                        // This close doesn't match the previous open but is also an open
                        stack.push(prev);
                        stack.push((i, g));
                    } else {
                        // This close doesn't match the previous open and is not an open
                        // So it's a mismatched close and should just return the position
                        return position;
                    }
                } else if closing.contains_key(&g.as_str()) {
                    // There is no previous open and is an open
                    stack.push((i, g));
                }
            } else if closing.contains_key(&g.as_str()) {
                // g is an open
                stack.push((i, g));
            }
        }

        position
    }
}
