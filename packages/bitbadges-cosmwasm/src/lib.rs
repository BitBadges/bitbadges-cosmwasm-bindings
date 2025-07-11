mod msg;
mod querier;
mod query;
//export all from msg
pub use msg::*;
pub use querier::BitBadgesQuerier;
pub use query::*;

// This export is added to all contracts that import this package, signifying that they require
// "bitbadges" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_bitbadges() {}
