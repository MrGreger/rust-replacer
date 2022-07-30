use regex::{Captures, Regex};

use crate::TextReplacing::RegexReplacerKeepCase;

mod ReplacementsCollection;
mod TextAttributes;
mod TextReplacing;
mod TextSplitting;

fn main() {
    // let mut replacer = RegexReplacerKeepCase::new();

    // let a = replacer.replace("LOL LOL", "LOL", "KEK");

    let re = Regex::new(r"LOL").unwrap();
    let result = re.replace_all("LOL KEK 123 123 LOL", |caps: &Captures| {
        for a in caps.iter(){
            if let Some(x) = a {
                //println!("{}",x.as_str().to_string());
            }
        }

        ""
    });
    println!("{}", result);
}
