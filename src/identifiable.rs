use super::IdClass;

/// An identifiable is an object which can be refered to with
/// an id (the id may be part, or not, of the object).
///
/// The best way to add this trait to a struct is to
/// use the `typid` derive attribute
pub trait Identifiable {
    fn class() -> IdClass;
}
