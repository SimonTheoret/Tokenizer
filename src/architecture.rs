use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::{Debug, Display}};

pub trait Tokenizer<'a, T, S>
where
    Self: Deserialize<'a> + Serialize + Debug,
    T: AsRef<str>,
{
    fn tokenize(&self, text: T) -> HashMap<String, i32, S>;
    fn train(&self);
}

pub trait TokenizerConfig<'a, 'b, Tkz, T, S>
where
    Self: Serialize + Deserialize<'a> + Default + Into<Tkz> + Debug + Display,
    Tkz: Tokenizer<'b, T, S>,
    T: AsRef<str>,
{
}
