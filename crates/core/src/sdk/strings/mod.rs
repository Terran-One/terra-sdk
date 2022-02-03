pub struct AccAddress(String);
pub struct ValAddress(String);
pub struct ValConsAddress(String);
pub struct AccPubKey(String);
pub struct ValPubKey(String);

pub mod traits;
pub use traits::*;
