pub struct AccAddress(pub String);
pub struct ValAddress(pub String);
pub struct ValConsAddress(pub String);
pub struct AccPubKey(pub String);
pub struct ValPubKey(pub String);

pub mod traits;
pub use traits::*;
