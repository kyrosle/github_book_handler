pub fn main() {
    let md = include_str!("../config.md");
    let html: String = markdown::to_html(md);
    println!("{}", html);
}
