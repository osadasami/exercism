use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut str_reversed = Vec::new();
    let mut str_splitted = input.graphemes(true).collect::<Vec<&str>>();

    for _ in 0..str_splitted.len() {
        str_reversed.push(str_splitted.pop().unwrap());
    }

    str_reversed.join("")
}
