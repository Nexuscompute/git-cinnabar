/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use derive_more::{From, TryInto};
use sha1::Sha1;

mod blob;
pub use blob::*;
mod commit;
pub use commit::*;
mod tree;
pub use tree::*;

use crate::libgit::FileMode;
use crate::oid::ObjectId;

oid_type!(GitObjectId for Sha1);

#[macro_export]
macro_rules! git_oid_type {
    ($name:ident($base_type:ident)) => {
        oid_type!($name($base_type));

        impl From<$name> for $crate::libgit::object_id {
            fn from(oid: $name) -> $crate::libgit::object_id {
                $crate::git::GitObjectId::from(oid).into()
            }
        }

        git_oid_type!(@ $name($base_type));
    };
    (@ $name:ident(GitObjectId)) => {
        impl PartialEq<$name> for $crate::git::GitOid {
            fn eq(&self, other: &$name) -> bool {
                $crate::git::GitOid::from(*other) == *self
            }
        }

        impl PartialEq<$crate::git::GitOid> for $name {
            fn eq(&self, other: &$crate::git::GitOid) -> bool {
                $crate::git::GitOid::from(*self) == *other
            }
        }

    };
    (@ $name:ident($base_type:ident)) => {
        oid_impl!($name($crate::git::GitObjectId));

        impl PartialEq<$name> for $crate::git::GitOid {
            fn eq(&self, other: &$name) -> bool {
                $crate::git::GitOid::from($base_type::from(*other)) == *self
            }
        }

        impl PartialEq<$crate::git::GitOid> for $name {
            fn eq(&self, other: &$crate::git::GitOid) -> bool {
                $crate::git::GitOid::from($base_type::from(*self)) == *other
            }
        }
    };
}

#[derive(Clone, Copy, From, TryInto, Debug, PartialEq, Eq)]
pub enum GitOid {
    Blob(BlobId),
    Tree(TreeId),
    Commit(CommitId),
}

impl GitOid {
    pub fn is_blob(&self) -> bool {
        matches!(self, GitOid::Blob(_))
    }

    pub fn is_tree(&self) -> bool {
        matches!(self, GitOid::Tree(_))
    }

    pub fn is_commit(&self) -> bool {
        matches!(self, GitOid::Commit(_))
    }

    pub fn as_raw_bytes(&self) -> &[u8] {
        match self {
            GitOid::Blob(b) => b.as_raw_bytes(),
            GitOid::Tree(t) => t.as_raw_bytes(),
            GitOid::Commit(c) => c.as_raw_bytes(),
        }
    }
}

impl std::fmt::Display for GitOid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        GitObjectId::from(*self).fmt(f)
    }
}

impl From<GitOid> for GitObjectId {
    fn from(value: GitOid) -> Self {
        match value {
            GitOid::Blob(blob) => blob.into(),
            GitOid::Tree(tree) => tree.into(),
            GitOid::Commit(commit) => commit.into(),
        }
    }
}

impl From<(GitObjectId, FileMode)> for GitOid {
    fn from((oid, mode): (GitObjectId, FileMode)) -> Self {
        match mode.typ() {
            FileMode::GITLINK => GitOid::Commit(CommitId::from_unchecked(oid)),
            FileMode::DIRECTORY => GitOid::Tree(TreeId::from_unchecked(oid)),
            _ => GitOid::Blob(BlobId::from_unchecked(oid)),
        }
    }
}

impl PartialEq<GitObjectId> for GitOid {
    fn eq(&self, other: &GitObjectId) -> bool {
        GitObjectId::from(*self) == *other
    }
}

impl PartialEq<GitOid> for GitObjectId {
    fn eq(&self, other: &GitOid) -> bool {
        GitObjectId::from(*other) == *self
    }
}
