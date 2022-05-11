use super::commit::*;
use crate::{fields::Serialized, index::TransactionList, ObjectId};
use serde::{de::DeserializeOwned, Serialize};

/// The root index of the tree that stores version information
#[derive(infinitree_macros::Index)]
pub(crate) struct RootIndex<CustomData>
where
    CustomData: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    /// Transaction log of individual fields included in each
    /// generation.
    ///
    /// The last generation's transactions are at _the front_, so
    /// looping through this naively will yield the last commit
    /// _first_.
    pub(crate) transaction_log: Serialized<TransactionList>,

    /// Chronologically ordered list of commits
    pub(crate) commit_list: Serialized<CommitList<CustomData>>,

    /// Stores the list of objects that contain the index.  To make
    /// storage use more efficient, on commit these are going to be
    /// rewritten first, but only with index data.
    #[infinitree(skip)]
    pub(crate) objects: Vec<ObjectId>,
}

impl<CustomData> Default for RootIndex<CustomData>
where
    CustomData: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            transaction_log: Serialized::default(),
            commit_list: Serialized::default(),
            objects: vec![],
        }
    }
}

impl<CustomData> RootIndex<CustomData>
where
    CustomData: Serialize + DeserializeOwned + Send + Sync,
{
    pub(crate) fn version_after(&self, gen: &CommitId) -> Option<CommitId> {
        let handle = self.commit_list.read();

        // walking in reverse may hit faster
        let mut iter = handle.iter().rev().map(|c| c.id).peekable();
        while let Some(i) = iter.next() {
            if Some(gen) == iter.peek() {
                return Some(i);
            }
        }

        None
    }
}
