use ledd::*;

fn main() {
    let text =
        String::from("I am become Death\n, destroyer \nof worlds.\nHitherto \nunknown function.\n");
    let fname = "buffer.test";
    let b = Buffer::init(fname, text.clone());
    // b.insert_line_with_text(text);
    let emb = Buffer::empty();
    println!("{:?}", b);
    println!("{:?}", emb);
}
