use gosub_engine::testing::tree_construction::fixture_from_filename;
use lazy_static::lazy_static;
use std::collections::HashSet;
use test_case::test_case;

const DISABLED_CASES: &[&str] = &[
    // tests2.dat
    "<!DOCTYPE html>X<p/x/y/z>",
    // tests6.dat
    "<body>\n<div>",
    "<frameset></frameset><noframes>",
    "</caption><div>",
    "</table><div>",
    "</table></tbody></tfoot></thead></tr><div>",
    "<table><colgroup>foo",
    "foo<col>",
    "</frameset><frame>",
    "</body><div>",
    "</tr><td>",
    "</tbody></tfoot></thead><td>",
    "<caption><col><colgroup><tbody><tfoot><thead><tr>",
    "</table><tr>",
    "<body></body></html>",
    "<param><frameset></frameset>",
    "<source><frameset></frameset>",
    "<track><frameset></frameset>",
    "</html><frameset></frameset>",
    "</body><frameset></frameset>",
    // tests7.dat
    "<body>X</body></body>",
    // tests18.dat
    "<!doctype html><template><plaintext>a</template>b",
    "<!doctype html><svg><plaintext>a</plaintext>b",
    "<!doctype html><svg><title><plaintext>a</plaintext>b",
];

lazy_static! {
    static ref DISABLED: HashSet<String> = DISABLED_CASES
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
}

// See tests/data/html5lib-tests/tree-construction/ for other test files.
#[test_case("tests1.dat")]
#[test_case("tests2.dat")]
#[test_case("tests3.dat")]
#[test_case("tests5.dat")]
#[test_case("tests6.dat")]
#[test_case("tests7.dat")]
#[test_case("tests8.dat")]
// #[test_case("tests14.dat")]
#[test_case("tests15.dat")]
#[test_case("tests16.dat")]
#[test_case("tests17.dat")]
#[test_case("tests18.dat")]
// #[test_case("tests19.dat")]
#[test_case("tests22.dat")]
#[test_case("tests24.dat")]
#[test_case("tests25.dat")]
// #[test_case("tests26.dat")]
fn tree_construction(filename: &str) {
    let fixture_file = fixture_from_filename(filename).expect("fixture");

    for test in fixture_file.tests {
        if DISABLED.contains(&test.data) {
            // Check that we don't panic
            // let _ = test.parse().expect("problem parsing");
            continue;
        }

        println!("tree construction: {}", test.data);
        test.assert_valid();
    }
}
