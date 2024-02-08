use {
    super::*,
    std::hash::{Hash, Hasher},
};

/// Short for "Identified", wraps a struct and its identifier.
///
/// The concrete type should most often be defined with only
/// one identifiable type, eg `Ided<Invoice>`, but it's also
/// possible to wrap an object with a non directly linked id,
/// as in `Ided<Invoice, InvoiceExpanded>`.
///
/// Equality and Hash implementations are based on the id: objects
/// are the same when they have the same id.
///
/// Ordering implementation of the Ided is based on the
/// ordering of the wrapped entity.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ided<T: Identifiable, E = T> {
    id: Id<T>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    entity: E,
}

impl<T: Identifiable, E> Ided<T, E> {
    /// Create a new Ided wrapping an id and an entity
    pub fn new(id: Id<T>, entity: E) -> Self {
        Self { id, entity }
    }

    /// Return the identifiant
    pub fn id(&self) -> Id<T> {
        self.id
    }

    /// Return a reference to the wrapped entity
    pub fn entity(&self) -> &E {
        &self.entity
    }

    /// Return a mutable reference to the wrapped entity
    pub fn entity_mut(&mut self) -> &mut E {
        &mut self.entity
    }

    /// Return the entity, dropping the item
    pub fn take_entity(self) -> E {
        self.entity
    }

    /// Destructure the ided into the wrapped id and entity
    pub fn dismantle(self) -> (Id<T>, E) {
        (self.id, self.entity)
    }
}

#[cfg(feature = "sqlx")]
impl<'e, T, E> sqlx::FromRow<'e, sqlx::postgres::PgRow> for Ided<T, E>
where
    T: Identifiable,
    E: sqlx::FromRow<'e, sqlx::postgres::PgRow>,
{
    fn from_row(row: &'e sqlx::postgres::PgRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        let uuid: uuid::Uuid = row.try_get("id")?;
        let id = Id::unchecked(uuid);
        let entity = E::from_row(row)?;
        Ok(Ided::new(id, entity))
    }
}

impl<T: Identifiable, E> Eq for Ided<T, E> {}
impl<T: Identifiable, E> PartialEq for Ided<T, E> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<O: Identifiable> Hash for Ided<O> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
impl<T: Identifiable, E: Ord> Ord for Ided<T, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entity().cmp(other.entity())
    }
}

impl<T: Identifiable, E: Ord> PartialOrd for Ided<T, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Identifiable, E> AsRef<E> for Ided<T, E> {
    fn as_ref(&self) -> &E {
        &self.entity
    }
}

impl<T: Identifiable, E> std::ops::Deref for Ided<T, E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
