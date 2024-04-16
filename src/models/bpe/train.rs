/*
This is the main trait to implement for a tokenizer. There are only
two components: The training and the tokenization process.

pub trait Tokenizer<'a, T>
where
    Self: Deserialize<'a> + Serialize + Debug,
    T: AsRef<str>,
{
    fn tokenize(&self, text: T) -> AHashMap<String, i32>;
    fn train(&self);
}
*/

use crate::tokens;


