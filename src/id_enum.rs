/// Create an enumeration whose variants are identifiers of several types.
///
/// A value can be parsed from string, the type being guessed from the prefix.
///
/// ```
/// use typid::*;
///
/// // Declare 2 types
/// #[derive(Debug, Typid)]
/// #[typid(class="Dog")]
/// pub struct Dog {}
///
/// #[derive(Debug, Typid)]
/// #[typid(class="Cat")]
/// pub struct Cat{}
///
/// // Declare an enum PetId whose values are ids of either a dog or a cat
/// id_enum! {PetId: Dog, Cat}
///
/// // This enumeration is the same than
/// // pub enum PetId {
/// //     Dog(Id<Dog>),
/// //     Cat(Id<Cat>),
/// // }
/// // but comes with automatic impls of FromStr and Display.
///
/// let s = "Dog_453d6f99-ce09-4dd7-bde9-73c1d2dbc1d0";
/// let a: PetId = s.parse().unwrap();
/// assert!(matches!(a, PetId::Dog(_)));
/// assert_eq!(s.to_string(), a.to_string());
/// ```
#[macro_export]
macro_rules! id_enum {
    {$Enum:ident: $($T:ident),* $(,)*} => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $Enum {
            $(
                $T(Id<$T>),
            )*
        }
        impl std::fmt::Display for $Enum {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(
                        Self::$T(id) => id.fmt(f),
                    )*
                }
            }
        }
        impl std::str::FromStr for $Enum {
            type Err = IdError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $(
                    match Id::<$T>::from_str(s) {
                        Err(IdError::WrongClass) => {}
                        Ok(id) => { return Ok(Self::$T(id)); }
                        Err(e) => { return Err(e); }
                    }
                )*
                Err(IdError::WrongClass)
            }
        }
    }
}
