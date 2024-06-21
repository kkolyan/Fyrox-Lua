use bindgen::expand_parser::model::Parser;


fn main() {
    let mut parser = Parser::default();
    parser.parse_dir("expand");
}
