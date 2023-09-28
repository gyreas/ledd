use ledd::*;

fn main() {
    let text =
        String::from("I am become Death, destroyer of worlds.\nHitherto unknown function.\n");
    let fname = "buffer.test".to_owned();
    let b = Buffer::init(fname, text.clone());
    b.insert_line_with_text(text);
    let emb = Buffer::empty();
    dbg!(b);
    dbg!(emb);
}
