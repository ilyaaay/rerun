// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/views/spatial3d.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **View**: A Spatial 3D view.
#[derive(Clone, Debug)]
pub struct Spatial3DView {
    /// Configuration for the background of the space view.
    pub background: crate::blueprint::archetypes::Background,
}

impl ::re_types_core::SizeBytes for Spatial3DView {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.background.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::archetypes::Background>::is_pod()
    }
}

impl<T: Into<crate::blueprint::archetypes::Background>> From<T> for Spatial3DView {
    fn from(v: T) -> Self {
        Self {
            background: v.into(),
        }
    }
}

impl std::borrow::Borrow<crate::blueprint::archetypes::Background> for Spatial3DView {
    #[inline]
    fn borrow(&self) -> &crate::blueprint::archetypes::Background {
        &self.background
    }
}

impl std::ops::Deref for Spatial3DView {
    type Target = crate::blueprint::archetypes::Background;

    #[inline]
    fn deref(&self) -> &crate::blueprint::archetypes::Background {
        &self.background
    }
}

impl ::re_types_core::View for Spatial3DView {
    #[inline]
    fn identifier() -> ::re_types_core::SpaceViewClassIdentifier {
        "3D".into()
    }
}