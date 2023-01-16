use {
    super::*,
    sqlx::types::Uuid,
    std::{
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        str::FromStr,
    },
};

/// UUID with costless type constraints
///
/// The identifiant has two representations as string:
/// - an hyphenated representation of the UUID used in database
/// - the public one, with the prefix
///
/// The Display implementation provides the public id, which
/// should be generally used, while the db id should be used
/// only for communication with the database.
pub struct Id<O: Identifiable> {
    uuid: Uuid,
    phantom: PhantomData<O>,
}

impl<O: Identifiable> PartialEq for Id<O> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}
impl<O: Identifiable> Eq for Id<O> {}

/// The Display implementation produces a publicly usable
/// id with the prefix preventing any ambiguity
impl<O: Identifiable> fmt::Display for Id<O> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.class().prefix(), &self.uuid.hyphenated())
    }
}

impl<O: Identifiable> fmt::Debug for Id<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Id")
            .field("uuid", &self.uuid)
            .field("class", &self.class().prefix())
            .finish()
    }
}
impl<O: Identifiable> Clone for Id<O> {
    fn clone(&self) -> Self {
        Self::unchecked(self.uuid)
    }
}
impl<O: Identifiable> Copy for Id<O> {}

impl<O: Identifiable> Id<O> {
    /// Return the class of an id, usually mapped to a specific struct
    pub fn class(&self) -> IdClass {
        <O as Identifiable>::class()
    }
    /// Return the internal UUID
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
    /// Return the database identifier as a string.
    ///
    /// This method should rarely be useful as Id can directly
    /// be written in database using sqlx without going through
    /// a string representation.
    pub fn db_id(&self) -> String {
        self.uuid.hyphenated().to_string()
    }
    /// Return the public representation as a string, which should
    /// be used in JSON, URL, or anywhere except the database.
    pub fn public_id(&self) -> String {
        self.to_string()
    }
    /// Parse an Id from its public representation, checking the class
    pub fn from_public_id(public_id: &str) -> Result<Self, IdError> {
        let db_id = <O as Identifiable>::class().strip_prefix(public_id)?;
        let uuid = Uuid::try_parse(db_id).map_err(|_| IdError::InvalidFormat)?;
        Ok(Self::unchecked(uuid))
    }
    /// Parse the Id from its database string representation, *not* checking
    /// the class (as it's not embedded in this representation)
    pub fn from_db_id(db_id: &str) -> Result<Self, IdError> {
        let uuid = Uuid::try_parse(db_id).map_err(|_| IdError::InvalidFormat)?;
        Ok(Self::unchecked(uuid))
    }
    /// Build an Id without checking the class
    pub(crate) fn unchecked(uuid: Uuid) -> Self {
        Self {
            uuid,
            phantom: PhantomData,
        }
    }
    /// Build a random Id based on Uuid v4 (only random)
    ///
    /// See <https://www.rfc-editor.org/rfc/rfc4122#section-4.4>
    pub fn random_v4() -> Self {
        Self::unchecked(Uuid::new_v4())
    }
}

/// Parse an Id from its public representation, checking the class
impl<O: Identifiable> FromStr for Id<O> {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_public_id(s)
    }
}

impl<O: Identifiable> Hash for Id<O> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid().hash(state);
    }
}
