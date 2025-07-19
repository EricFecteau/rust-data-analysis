use markdown2pdf;

fn main() {
    let markdown = "# Hello World\nThis is a test.".to_string();
    markdown2pdf::parse_into_file(markdown, "./data/output/output.pdf", None).unwrap();
}
