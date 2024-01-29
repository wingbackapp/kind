use {
    super::*,
    std::{
        cmp::Ordering,
        fmt,
        hash::{Hash, Hasher},
        marker::PhantomData,
        str::FromStr,
    },
    uuid::Uuid,
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
        *self
    }
}
impl<O: Identifiable> Copy for Id<O> {}

impl<O: Identifiable> PartialOrd for Id<O> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<O: Identifiable> Ord for Id<O> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uuid.cmp(&other.uuid)
    }
}

impl<O: Identifiable> Id<O> {
    /// Return the class of an id, usually mapped to a specific struct
    pub fn class(&self) -> IdClass {
        <O as Identifiable>::class()
    }
    /// Return the internal UUID
    pub fn uuid(&self) -> Uuid {
        self.uuid
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

#[test]
fn id_sorting() {
    #[derive(Debug, Typid)]
    #[typid(class = "Ex")]
    pub struct E {}

    let mut ids = Vec::<Id<E>>::new();
    for _ in 0..10 {
        ids.push(Id::random_v4());
    }
    let mut ids_by_string = ids.clone();

    ids.sort();
    ids_by_string.sort_by_key(|id| id.to_string());

    // check that the Id order is the same than the
    // one of their stringified representations
    for (a, b) in ids.iter().zip(ids_by_string.iter()) {
        assert_eq!(a, b);
    }

    // check that we can use a slice of ids as a sort key
    // (this should be true as soon as id is Ord, but I
    // prefer to be sure...)

    let a = vec![ids[1]];
    let b = vec![ids[1], ids[7]];
    let c = vec![ids[3]];
    let d = vec![ids[3], ids[2]];
    let e = vec![ids[3], ids[2], ids[5]];
    let f = vec![ids[3], ids[6]];
    let g = vec![ids[3], ids[6], ids[8]];

    let paths_1 = vec![
        a.clone(),
        b.clone(),
        c.clone(),
        d.clone(),
        e.clone(),
        f.clone(),
        g.clone(),
    ];
    let mut paths_2 = vec![d, g, c, f, b, e, a];
    paths_2.sort();
    for (path_1, path_2) in paths_1.iter().zip(paths_2.iter()) {
        for (a, b) in path_1.iter().zip(path_2.iter()) {
            assert_eq!(a, b);
        }
    }
}
