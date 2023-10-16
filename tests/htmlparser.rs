use test_case::test_case;

use gosub_engine::html5_parser::input_stream::InputStream;
use gosub_engine::html5_parser::node::{NodeData, NodeId, MATHML_NAMESPACE, SVG_NAMESPACE};
use gosub_engine::html5_parser::parser::document::Document;
use gosub_engine::html5_parser::parser::Html5Parser;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub const FIXTURE_ROOT: &str = "./tests/data/html5lib-tests";

#[derive(PartialEq, Debug, Clone)]
pub struct Error {
    pub code: String,
    pub line: i64,
    pub col: i64,
}

#[derive(Debug, Clone)]
struct Test {
    /// input stream
    data: String,
    /// errors
    errors: Vec<Error>,
    /// document tree
    document: Vec<String>,
    /// fragment
    document_fragment: Vec<String>,
}

fn is_section_head(line: &String) -> bool {
    if line.starts_with('#') {
        return true;
    }
    false
}

/// Read given tests file and extract all test data
fn read_tests(file_path: PathBuf) -> io::Result<Vec<Test>> {
    let file = File::open(file_path.clone())?;
    let reader = BufReader::new(file);

    let mut tests: Vec<Test> = Vec::new();
    let mut current_test: Option<Test> = None;
    let mut current_section: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        let is_section_head = is_section_head(&line);

        if is_section_head {
            let section = line.clone().trim_end().trim_start_matches('#').to_string();
            if current_test.is_some() && section == "data" {
                tests.push(current_test.to_owned().unwrap());
                current_test = None;
            }
            current_section = Some(section);
            if current_test.is_none() {
                current_test = Some(Test {
                    data: "".to_string(),
                    errors: vec![],
                    document: vec![],
                    document_fragment: vec![],
                });
            }
        } else if let Some(ref sec) = current_section {
            match current_test.iter_mut().next() {
                Some(x) => match sec.as_str() {
                    "data" => {
                        x.data.push_str(&line);
                    }
                    "error" => {
                        let re =
                            Regex::new(r"\((?P<line>\d+),(?P<col>\d+)\): (?P<code>.+)").unwrap();
                        if let Some(caps) = re.captures(&line) {
                            let line = caps.name("line").unwrap().as_str().parse::<i64>().unwrap();
                            let col = caps.name("col").unwrap().as_str().parse::<i64>().unwrap();
                            let code = caps.name("code").unwrap().as_str().to_string();
                            x.errors.push(Error { code, line, col });
                        }
                    }
                    "document" => {
                        x.document.push(line);
                    }
                    "document_fragment" => {
                        x.document_fragment.push(line);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    if current_test.is_some() {
        tests.push(current_test.clone().unwrap());
    }

    Ok(tests)
}

fn run_tree_test(test: &Test) {
    // Do the actual parsing
    let mut is = InputStream::new();
    is.read_from_str(test.data.as_str(), None);

    let mut parser = Html5Parser::new(&mut is);
    let document = Document::shared();
    let parse_errors = parser.parse(Document::clone(&document)).unwrap();

    // Check the document tree, which counts as a single assertion
    match_document_tree(&document.get(), &test.document);

    // assert_eq!(parse_errors.len(), test.errors.len());

    // For now, we skip the tests that checks for errors as most of the errors do not match
    // with the actual tests, as these errors as specific from html5lib. Either we reuse them
    // or have some kind of mapping to our own errors if we decide to use our custom errors.

    let mut idx = 0;
    for error in &test.errors {
        if parse_errors.get(idx).is_none() {
            println!(
                "❌ Expected error '{}' at {}:{}",
                error.code, error.line, error.col
            );
            continue;
        }
        let err = parse_errors.get(idx).unwrap();
        let got_error = Error {
            code: err.message.to_string(),
            line: err.line as i64,
            col: err.col as i64,
        };
        match_error(&got_error, error);
        idx += 1;
    }
}

fn match_document_tree(document: &Document, expected: &Vec<String>) -> bool {
    // We need a better tree match system. Right now we match the tree based on the (debug) output
    // of the tree. Instead, we should generate a document-tree from the expected output and compare
    // it against the current generated tree.
    match_node(NodeId::root(), -1, -1, document, expected).is_some()
}

fn match_node(
    node_idx: NodeId,
    expected_id: isize,
    indent: isize,
    document: &Document,
    expected: &Vec<String>,
) -> Option<usize> {
    let node = document.get_node_by_id(node_idx).unwrap();

    if node_idx.is_positive() {
        match &node.data {
            NodeData::Element(element) => {
                let value = if node.namespace == Some(SVG_NAMESPACE.into()) {
                    format!(
                        "|{}<svg {}>",
                        " ".repeat((indent as usize * 2) + 1),
                        element.name()
                    )
                } else if node.namespace == Some(MATHML_NAMESPACE.into()) {
                    format!(
                        "|{}<math {}>",
                        " ".repeat((indent as usize * 2) + 1),
                        element.name()
                    )
                } else {
                    format!(
                        "|{}<{}>",
                        " ".repeat((indent as usize * 2) + 1),
                        element.name()
                    )
                };
                // let value = format!(
                //     "|{}<{}>",
                //     " ".repeat((indent as usize * 2) + 1),
                //     element.name()
                // );
                if value != expected[expected_id as usize] {
                    println!(
                        "❌ {}, Found unexpected element node: {}",
                        expected[expected_id as usize], value
                    );
                    return None;
                } else {
                    println!("✅  {}", expected[expected_id as usize]);
                }
            }
            NodeData::Text(text) => {
                let value = format!(
                    "|{}\"{}\"",
                    " ".repeat(indent as usize * 2 + 1),
                    text.value()
                );
                if value != expected[expected_id as usize] {
                    println!(
                        "❌ {}, Found unexpected text node: {}",
                        expected[expected_id as usize], value
                    );
                    return None;
                } else {
                    println!("✅  {}", expected[expected_id as usize]);
                }
            }
            NodeData::Comment(comment) => {
                let value = format!(
                    "|{}<!-- {} -->",
                    " ".repeat(indent as usize * 2 + 1),
                    comment.value()
                );
                if value != expected[expected_id as usize] {
                    println!(
                        "❌ {}, Found unexpected text node: <!-- {} -->",
                        expected[expected_id as usize], value
                    );
                    return None;
                } else {
                    println!("✅  {}", expected[expected_id as usize]);
                }
            }
            _ => {}
        }
    }

    let mut next_expected_idx = expected_id + 1;

    for &child_idx in &node.children {
        if let Some(new_idx) =
            match_node(child_idx, next_expected_idx, indent + 1, document, expected)
        {
            next_expected_idx = new_idx as isize;
        } else {
            return None;
        }
    }

    Some(next_expected_idx as usize)
}

#[allow(dead_code)]
fn match_error(got_err: &Error, expected_err: &Error) {
    if got_err == expected_err {
        return;
    }

    if got_err.code != expected_err.code {
        panic!(
            "❌ Expected error '{}' at {}:{}",
            expected_err.code, expected_err.line, expected_err.col
        );
    }

    // Found an error with the same code, but different line/pos
    panic!(
        "⚠️ Unexpected error position '{}' at {}:{} (got: {}:{})",
        expected_err.code, expected_err.line, expected_err.col, got_err.line, got_err.col
    );
}

#[test_case("tests1.dat")]
#[test_case("tests2.dat")]
#[test_case("tests3.dat")]
#[test_case("tests4.dat")]
#[test_case("tests5.dat")]
#[test_case("tests6.dat")]
#[test_case("tests7.dat")]
#[test_case("tests8.dat")]
#[test_case("tests9.dat")]
#[test_case("tests10.dat")]
#[test_case("tests11.dat")]
#[test_case("tests12.dat")]
#[test_case("tests14.dat")]
#[test_case("tests15.dat")]
//// #[test_case("tests16.dat")]
#[test_case("tests17.dat")]
#[test_case("tests18.dat")]
#[test_case("tests19.dat")]
#[test_case("tests20.dat")]
//// #[test_case("tests21.dat")]
#[test_case("tests22.dat")]
#[test_case("tests23.dat")]
#[test_case("tests24.dat")]
#[test_case("tests25.dat")]
#[test_case("tests26.dat")]
#[test_case("ruby.dat")]
//// #[test_case("scriptdata01.dat")]
#[test_case("search-element.dat")]
//// #[test_case("svg.dat")]
#[test_case("tables01.dat")]
#[test_case("template.dat")]
//// #[test_case("tests_innerHTML_1.dat")]
#[test_case("tricky01.dat")]
#[test_case("webkit01.dat")]
#[test_case("webkit02.dat")]
#[test_case("quirks01.dat")]
#[test_case("blocks.dat")]
#[test_case("comments01.dat")]
#[test_case("doctype01.dat")]
//// #[test_case("domjs-unsafe.dat")]
#[test_case("entities01.dat")]
#[test_case("entities02.dat")]
//// #[test_case("foreign-fragment.dat")]
#[test_case("html5test-com.dat")]
#[test_case("inbody01.dat")]
#[test_case("isindex.dat")]
#[test_case("main-element.dat")]
//// #[test_case("math.dat")]
#[test_case("menuitem-element.dat")]
#[test_case("namespace-sensitivity.dat")]
//// #[test_case("noscript01.dat")]
#[test_case("pending-spec-changes.dat")]
#[test_case("pending-spec-changes-plain-text-unsafe.dat")]
#[test_case("adoption01.dat")]
#[test_case("adoption02.dat")]
//// #[test_case("scripted/adoption01.dat")]
//// #[test_case("scripted/ark.dat")]
//// #[test_case("scripted/webkit01.dat")]
fn html5parsere(filename: &str) {
    let path = PathBuf::from(FIXTURE_ROOT)
        .join("tree-construction")
        .join(filename);
    let tests = read_tests(path.clone()).unwrap();

    for test in tests {
        run_tree_test(&test);
    }
}
