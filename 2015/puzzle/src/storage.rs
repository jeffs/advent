use crux_core::capability::Operation;

/// Names a value in the shell's persistent storage.
#[derive(Clone, Debug, PartialEq)]
pub struct Key(String);

impl Key {
    pub fn new(name: impl Into<String>) -> Self {
        Key(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Asks the shell for the value stored under a key, if any.
#[derive(Clone, Debug, PartialEq)]
pub struct Load(pub Key);

impl Operation for Load {
    type Output = Option<String>;
}

/// Tells the shell to store a value under a key.
#[derive(Clone, Debug, PartialEq)]
pub struct Save {
    pub key: Key,
    pub value: String,
}

impl Operation for Save {
    type Output = ();
}
