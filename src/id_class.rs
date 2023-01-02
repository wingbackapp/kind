use {crate::IdError, std::fmt};

/// A category of identifiable objects
///
/// All instances should be statically created.
///
/// It's strongly recommended to use the derive to map
/// classes and identifiable structs. The derive
/// brings sanity checks and ensure the prefix is `[a-zA-Z0-9]+`
/// (the rest of the code makes no assumption other than
/// a non empty class)
///
/// Don't try to use the same class for two
/// identifiable strucs, instead map the class
/// to the most "natural" struct.
#[derive(Debug, Clone, Copy)]
pub struct IdClass {
    prefix: &'static str,
}

impl IdClass {
    /// Create a new valid class.
    pub const fn new(prefix: &'static str) -> Self {
        assert!(!prefix.is_empty());
        Self { prefix }
    }
    pub fn prefix(self) -> &'static str {
        self.prefix
    }
    /// Remove the prefix and underscore from a public id to
    /// get the db_id.
    ///
    /// Return an error if the provided public id doesn't start
    /// with the right prefix.
    pub fn strip_prefix(self, public_id: &str) -> Result<&str, IdError> {
        // The implementation here doesn't assume anything about the
        // class as it can't be enforced in the const constructor.
        let mut public_id_chars = public_id.chars();
        let mut public_prefix_len = 0; // in bytes
        for class_char in self.prefix.chars() {
            let Some(public_id_char) = public_id_chars.next() else {
                return Err(IdError::WrongClass);
            };
            if !public_id_char.eq_ignore_ascii_case(&class_char) {
                return Err(IdError::WrongClass);
            }
            public_prefix_len += public_id_char.len_utf8();
        }
        if public_id_chars.next() != Some('_') {
            return Err(IdError::InvalidFormat);
        }
        public_prefix_len += 1;
        Ok(&public_id[public_prefix_len..])
    }
}

impl fmt::Display for IdClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.prefix)
    }
}
