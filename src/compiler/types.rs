use std::fmt::{ Debug, Formatter };

use serde::{ Serialize, Deserialize };
use serde::ser::Serializer;
use serde::de::Deserializer;

use super::name::Name;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Type {
    pub kind: TypeKind,
    pub refs: Option<Name>
}

impl Type {
    pub fn new(kind: TypeKind, name: Option<Name>) -> Type {
        Type {
            kind,
            refs: name
        }
    }

    pub fn new_unsovled_nohint() -> Type {
        Type {
            kind: TypeKind::UnsolvedNoHint,
            refs: None
        }
    }

    pub fn from(name: String) -> Type {
        Type { kind: TypeKind::from(name), refs: None }
    }
}

#[derive(Clone, PartialEq)]
pub enum TypeKind {
    /* プリミティブ型 */
    Int32,

    /* ユーザ定義型 */
    Data,

    /* パーサ用 (解決後のSysDCSystemには含まれない) */
    Unsolved(String),
    UnsolvedNoHint
}

impl TypeKind {
    fn from(name: String) -> TypeKind {
        match name.as_str() {
            "i32" => TypeKind::Int32,
            _ => TypeKind::Unsolved(name)
        }
    }
}

impl Debug for TypeKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TypeKind::Int32 => write!(f, "i32"),
            TypeKind::Data => write!(f, "Data"),
            TypeKind::Unsolved(hint) => write!(f, "{}", hint),
            TypeKind::UnsolvedNoHint => write!(f, "UnsolvedNoHint"),
        }
    }
}

impl Serialize for TypeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            TypeKind::Unsolved(_) |
            TypeKind::UnsolvedNoHint => panic!("[ERROR] Cannot serialize object containing unsolved types."),
            _ => serializer.serialize_str(&format!("{:?}", self))
        }
    }
}

impl<'de> Deserialize<'de> for TypeKind {
    fn deserialize<D>(deserializer: D) -> Result<TypeKind, D::Error>
    where
        D: Deserializer<'de>
    {
        let kind = String::deserialize(deserializer)?;
         Ok(match kind.as_str() {
            "i32" => TypeKind::Int32,
            "Data" => TypeKind::Data,
            s => panic!("[ERROR] Found unknown type at deserializing => \"{}\"", s)
        })
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    use rmp_serde;
    use rmp_serde::Serializer;

    use super::TypeKind;

    macro_rules! check_serialize {
        ($target:ty, $obj:expr) => {
            let mut serialized = vec!();
            $obj.serialize(&mut Serializer::new(&mut serialized)).unwrap();
            let deserialized = rmp_serde::from_slice::<$target>(&serialized[..]).unwrap();
            assert_eq!(deserialized, $obj);
        };
    }

    #[test]
    fn primitive() {
        check_serialize!(TypeKind, TypeKind::Int32);
        check_serialize!(TypeKind, TypeKind::Data);
    }

    #[test]
    #[should_panic]
    fn primitive_unsolved_1() {
        check_serialize!(TypeKind, TypeKind::Unsolved("aaa".to_string()));
    }

    #[test]
    #[should_panic]
    fn primitive_unsolved_2() {
        check_serialize!(TypeKind, TypeKind::UnsolvedNoHint);
    }
}
