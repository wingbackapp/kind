use {
    super::*,
    sqlx::{
        decode::Decode,
        encode::{Encode, IsNull},
        error::BoxDynError,
        postgres::{PgArgumentBuffer, PgHasArrayType, PgRow, PgTypeInfo, PgValueRef, Postgres},
        types::Uuid,
        Row, Type,
    },
};

impl<O: Identifiable> Type<Postgres> for Id<O> {
    fn type_info() -> PgTypeInfo {
        <Uuid as Type<Postgres>>::type_info()
    }
}

/// Make it possible to bind a slice of Id in a
/// query and use it with eg the ANY operator.
impl<O: Identifiable> PgHasArrayType for Id<O> {
    fn array_type_info() -> PgTypeInfo {
        <Uuid as PgHasArrayType>::array_type_info()
    }
}

impl<O: Identifiable> Encode<'_, Postgres> for Id<O> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        self.uuid().encode_by_ref(buf)
    }
}

impl<O: Identifiable> Decode<'_, Postgres> for Id<O> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let uuid: Uuid = <Uuid as Decode<'_, Postgres>>::decode(value)?;
        let id = Id::unchecked(uuid);
        Ok(id)
    }
}

impl<'r, T, E> Ided<T, E>
where
    T: Identifiable,
    E: sqlx::FromRow<'r, PgRow>,
{
    pub fn from_id_row(
        id_col_name: &'static str,
        row: &'r PgRow,
    ) -> Result<Ided<T, E>, sqlx::Error> {
        let id = row.try_get::<Id<T>, _>(id_col_name)?;
        let entity = E::from_row(row)?;
        Ok(Ided::new(id, entity))
    }
}
