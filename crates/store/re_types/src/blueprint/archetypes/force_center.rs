// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/force_center.fbs".

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

use ::re_types_core::external::arrow;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Tries to move the center of mass of the graph to the origin.
#[derive(Clone, Debug)]
pub struct ForceCenter {
    /// Whether the center force is enabled.
    ///
    /// The center force tries to move the center of mass of the graph towards the origin.
    pub enabled: Option<crate::blueprint::components::Enabled>,

    /// The strength of the force.
    pub strength: Option<crate::blueprint::components::ForceStrength>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
            component_name: "rerun.blueprint.components.ForceCenterIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                component_name: "rerun.blueprint.components.Enabled".into(),
                archetype_field_name: Some("enabled".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                component_name: "rerun.blueprint.components.ForceStrength".into(),
                archetype_field_name: Some("strength".into()),
            },
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                component_name: "rerun.blueprint.components.ForceCenterIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                component_name: "rerun.blueprint.components.Enabled".into(),
                archetype_field_name: Some("enabled".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                component_name: "rerun.blueprint.components.ForceStrength".into(),
                archetype_field_name: Some("strength".into()),
            },
        ]
    });

impl ForceCenter {
    /// The total number of components in the archetype: 0 required, 1 recommended, 2 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`ForceCenter`] [`::re_types_core::Archetype`]
pub type ForceCenterIndicator = ::re_types_core::GenericIndicatorComponent<ForceCenter>;

impl ::re_types_core::Archetype for ForceCenter {
    type Indicator = ForceCenterIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ForceCenter".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Force center"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ForceCenterIndicator = ForceCenterIndicator::DEFAULT;
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
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let enabled = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.Enabled")
        {
            <crate::blueprint::components::Enabled>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ForceCenter#enabled")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let strength =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ForceStrength") {
                <crate::blueprint::components::ForceStrength>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ForceCenter#strength")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self { enabled, strength })
    }
}

impl ::re_types_core::AsComponents for ForceCenter {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (self
                .enabled
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                    archetype_field_name: Some(("enabled").into()),
                    component_name: ("rerun.blueprint.components.Enabled").into(),
                }),
            }),
            (self
                .strength
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ForceCenter".into()),
                    archetype_field_name: Some(("strength").into()),
                    component_name: ("rerun.blueprint.components.ForceStrength").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ForceCenter {}

impl ForceCenter {
    /// Create a new `ForceCenter`.
    #[inline]
    pub fn new() -> Self {
        Self {
            enabled: None,
            strength: None,
        }
    }

    /// Whether the center force is enabled.
    ///
    /// The center force tries to move the center of mass of the graph towards the origin.
    #[inline]
    pub fn with_enabled(
        mut self,
        enabled: impl Into<crate::blueprint::components::Enabled>,
    ) -> Self {
        self.enabled = Some(enabled.into());
        self
    }

    /// The strength of the force.
    #[inline]
    pub fn with_strength(
        mut self,
        strength: impl Into<crate::blueprint::components::ForceStrength>,
    ) -> Self {
        self.strength = Some(strength.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for ForceCenter {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.enabled.heap_size_bytes() + self.strength.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::blueprint::components::Enabled>>::is_pod()
            && <Option<crate::blueprint::components::ForceStrength>>::is_pod()
    }
}
