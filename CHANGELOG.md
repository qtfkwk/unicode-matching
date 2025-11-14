# Changelog

* 0.1.0 (2025-02-20): Initial release
    * 0.1.1 (2025-2-20): Fix description and changelog; automatically check and save the last modified timestamp in `last-modified.txt`
* 0.2.0 (2025-03-09): Add straight single and double quotes
* 0.3.0 (2025-03-09): Generate `src/lib.rs`; remove extras and add tests showing how to customize
* 0.4.0 (2025-03-10): Add `FindMatching` trait with `find_matching` method and implementations for `str` and `GString` to find the index of the matching grapheme for the grapheme at the given position; fix wget command
* 0.5.0 (2025-03-10): Add `{ALL,BRACKETS{,_MATCHING,_MIRRORING},MIRRORING{,_MATCHING}` constants and corresponding `{close,open,matching}_{all,brackets{,_matching,_mirroring},mirroring{,_matching}` functions with additional graphemes from the `BidiBrackets.txt` and `BidiMirroring.txt` files ([mentioned here](https://stackoverflow.com/questions/13535172/list-of-all-unicodes-open-close-brackets/13535289#comment53701946_13535289)); port `matching.sh` from Bash to Perl: `matching.pl`
    * 0.5.1 (2025-03-10): Update readme and include as crate doc
    * 0.5.2 (2025-03-10): Fix description; add doctest example; improve doc
    * 0.5.3 (2025-03-10): Improve the doctest example
    * 0.5.4 (2025-03-10): Fix the doctest example; use `rustfmt` on generated `src/lib.rs`
    * 0.5.5 (2025-04-16): Update dependencies
    * 0.5.6 (2025-05-10): Separate `last-modified.txt` target in makefile; update dependencies
* 0.6.0 (2025-09-02): Update dependencies; 2024 edition
    * 0.6.1 (2025-10-27): Update dependencies
    * 0.6.2 (2025-10-27): Fix regressions in `t/lib.rs` and `bin/last-modified.sh`
    * 0.6.3 (2025-11-12): Update dependencies
    * 0.6.4 (2025-11-14): Update dependencies; add `clippy::pedantic` to `cargo clippy` command in the `clippy` target in the makefile; clippy fixes

