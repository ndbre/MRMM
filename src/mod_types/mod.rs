mod modification;
mod loose_pak;
pub(crate) mod multi_pak;

use std::fmt::Debug;
pub use modification::*;
pub use loose_pak::*;
pub use multi_pak::*;
pub enum ModType {
    Complete(Modification),
    LoosePak(LoosePak),
    MultiPak(MultiPak),
}

impl Debug for ModType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ModType::*;
        match self {
            Complete(mod_type) => write!(f, "{:?}", mod_type),
            LoosePak(mod_type) => write!(f, "{:?}", mod_type),
            MultiPak(mod_type) => write!(f, "{:?}", mod_type),
        }
    }
}