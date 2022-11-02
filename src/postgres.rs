use {
    super::*,
    sqlx::{
        decode::Decode,
        encode::{Encode, IsNull},
        error::BoxDynError,
        postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef, Postgres},
        types::Uuid,
        Type,
    },
};

impl<O: Identifiable> Type<Postgres> for Id<O> {
    fn type_info() -> PgTypeInfo {
        <Uuid as Type<Postgres>>::type_info()
    }
}

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
