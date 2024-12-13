// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/visual_bounds2d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Controls the visual bounds of a 2D view.
///
/// Everything within these bounds are guaranteed to be visible.
/// Somethings outside of these bounds may also be visible due to letterboxing.
///
/// If no visual bounds are set, it will be determined automatically,
/// based on the bounding-box of the data or other camera information present in the view.
#[derive(Clone, Debug, Copy)]
pub struct VisualBounds2D {
    /// Controls the visible range of a 2D view.
    ///
    /// Use this to control pan & zoom of the view.
    pub range: crate::blueprint::components::VisualBounds2D,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.VisualBounds2D".into()),
            component_name: "rerun.blueprint.components.VisualBounds2D".into(),
            archetype_field_name: Some("range".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.VisualBounds2D".into()),
            component_name: "rerun.blueprint.components.VisualBounds2DIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.VisualBounds2D".into()),
                component_name: "rerun.blueprint.components.VisualBounds2D".into(),
                archetype_field_name: Some("range".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.VisualBounds2D".into()),
                component_name: "rerun.blueprint.components.VisualBounds2DIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

impl VisualBounds2D {
    /// The total number of components in the archetype: 1 required, 1 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`VisualBounds2D`] [`::re_types_core::Archetype`]
pub type VisualBounds2DIndicator = ::re_types_core::GenericIndicatorComponent<VisualBounds2D>;

impl ::re_types_core::Archetype for VisualBounds2D {
    type Indicator = VisualBounds2DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.VisualBounds2D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Visual bounds 2D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: VisualBounds2DIndicator = VisualBounds2DIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let range = {
            let array = arrays_by_name
                .get("rerun.blueprint.components.VisualBounds2D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.VisualBounds2D#range")?;
            <crate::blueprint::components::VisualBounds2D>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.VisualBounds2D#range")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.VisualBounds2D#range")?
        };
        Ok(Self { range })
    }
}

impl ::re_types_core::AsComponents for VisualBounds2D {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.range as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.blueprint.archetypes.VisualBounds2D".into()),
                        archetype_field_name: Some(("range").into()),
                        component_name: ("rerun.blueprint.components.VisualBounds2D").into(),
                    }),
                }
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for VisualBounds2D {}

impl VisualBounds2D {
    /// Create a new `VisualBounds2D`.
    #[inline]
    pub fn new(range: impl Into<crate::blueprint::components::VisualBounds2D>) -> Self {
        Self {
            range: range.into(),
        }
    }
}

impl ::re_types_core::SizeBytes for VisualBounds2D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.range.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::components::VisualBounds2D>::is_pod()
    }
}
