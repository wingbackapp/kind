use {
    super::*,
    sqlx::{postgres::PgRow, types::Uuid, Row},
    std::hash::{Hash, Hasher},
};

/// Short for "Identified", wraps a struct and its identifier.
///
/// Similar to our wingback-core Known, and should replace it.
///
/// The concrete type will most often be defined with only
/// one identifiable type, eg `Ided<Invoice>`, but it's also
/// possible to wrap an object with a non directly linked id,
/// as in `Ided<Invoice, InvoiceExpanded>`.
#[derive(Debug, Clone)]
pub struct Ided<T: Identifiable, E = T> {
    id: Id<T>,
    entity: E,
}

impl<T: Identifiable, E> Ided<T, E> {
    pub fn new(id: Id<T>, entity: E) -> Self {
        Self { id, entity }
    }

    pub fn id(&self) -> Id<T> {
        self.id
    }

    pub fn entity(&self) -> &E {
        &self.entity
    }

    pub fn entity_mut(&mut self) -> &mut E {
        &mut self.entity
    }

    pub fn take_entity(self) -> E {
        self.entity
    }

    pub fn dismantle(self) -> (Id<T>, E) {
        (self.id, self.entity)
    }
}

impl<'e, T, E> sqlx::FromRow<'e, PgRow> for Ided<T, E>
where
    T: Identifiable,
    E: sqlx::FromRow<'e, PgRow>,
{
    fn from_row(row: &'e PgRow) -> sqlx::Result<Self> {
        let uuid: Uuid = row.try_get("id")?;
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
