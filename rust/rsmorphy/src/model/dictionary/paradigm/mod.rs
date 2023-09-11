mod array;
mod entry;
mod id;
mod idx;
mod suspended;

pub use array::ParadigmList;
pub use entry::ParadigmEntry;
pub use id::ParadigmId;
pub use idx::ParadigmIndex;
pub use suspended::ParadigmListSuspended;

pub type Paradigm<'a> = &'a [ParadigmEntry];
