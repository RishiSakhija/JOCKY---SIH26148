fn main() {
    pest_generator::generate_grammar(
        std::path::Path::new("src/parser.pest")
    ).unwrap();
}