// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/asset_video.fbs".

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

/// **Archetype**: A video binary.
///
/// Only MP4 containers with AV1 are generally supported,
/// though the web viewer supports more video codecs, depending on browser.
///
/// See <https://rerun.io/docs/reference/video> for details of what is and isn't supported.
///
/// In order to display a video, you also need to log a [`archetypes::VideoFrameReference`][crate::archetypes::VideoFrameReference] for each frame.
///
/// ## Examples
///
/// ### Video with automatically determined frames
/// ```ignore
/// use rerun::external::anyhow;
///
/// fn main() -> anyhow::Result<()> {
///     let args = _args;
///     let Some(path) = args.get(1) else {
///         // TODO(#7354): Only mp4 is supported for now.
///         anyhow::bail!("Usage: {} <path_to_video.[mp4]>", args[0]);
///     };
///
///     let rec =
///         rerun::RecordingStreamBuilder::new("rerun_example_asset_video_auto_frames").spawn()?;
///
///     // Log video asset which is referred to by frame references.
///     let video_asset = rerun::AssetVideo::from_file_path(path)?;
///     rec.log_static("video", &video_asset)?;
///
///     // Send automatically determined video frame timestamps.
///     let frame_timestamps_ns = video_asset.read_frame_timestamps_ns()?;
///     let video_timestamps_ns = frame_timestamps_ns
///         .iter()
///         .copied()
///         .map(rerun::components::VideoTimestamp::from_nanoseconds)
///         .collect::<Vec<_>>();
///     let time_column = rerun::TimeColumn::new_nanos(
///         "video_time",
///         // Note timeline values don't have to be the same as the video timestamps.
///         frame_timestamps_ns,
///     );
///     let frame_reference_indicators =
///         <rerun::VideoFrameReference as rerun::Archetype>::Indicator::new_array(
///             time_column.num_rows(),
///         );
///     rec.send_columns(
///         "video",
///         [time_column],
///         [&frame_reference_indicators as _, &video_timestamps_ns as _],
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/1200w.png">
///   <img src="https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Demonstrates manual use of video frame references
/// ```ignore
/// use rerun::external::anyhow;
///
/// fn main() -> anyhow::Result<()> {
///     let args = _args;
///     let Some(path) = args.get(1) else {
///         // TODO(#7354): Only mp4 is supported for now.
///         anyhow::bail!("Usage: {} <path_to_video.[mp4]>", args[0]);
///     };
///
///     let rec =
///         rerun::RecordingStreamBuilder::new("rerun_example_asset_video_manual_frames").spawn()?;
///
///     // Log video asset which is referred to by frame references.
///     rec.log_static("video_asset", &rerun::AssetVideo::from_file_path(path)?)?;
///
///     // Create two entities, showing the same video frozen at different times.
///     rec.log(
///         "frame_1s",
///         &rerun::VideoFrameReference::new(rerun::components::VideoTimestamp::from_seconds(1.0))
///             .with_video_reference("video_asset"),
///     )?;
///     rec.log(
///         "frame_2s",
///         &rerun::VideoFrameReference::new(rerun::components::VideoTimestamp::from_seconds(2.0))
///             .with_video_reference("video_asset"),
///     )?;
///
///     // TODO(#5520): log blueprint once supported
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/video_manual_frames/9f41c00f84a98cc3f26875fba7c1d2fa2bad7151/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/video_manual_frames/9f41c00f84a98cc3f26875fba7c1d2fa2bad7151/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/video_manual_frames/9f41c00f84a98cc3f26875fba7c1d2fa2bad7151/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/video_manual_frames/9f41c00f84a98cc3f26875fba7c1d2fa2bad7151/1200w.png">
///   <img src="https://static.rerun.io/video_manual_frames/9f41c00f84a98cc3f26875fba7c1d2fa2bad7151/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug)]
pub struct AssetVideo {
    /// The asset's bytes.
    pub blob: crate::components::Blob,

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `video/mp4`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    pub media_type: Option<crate::components::MediaType>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.AssetVideo".into()),
            component_name: "rerun.components.Blob".into(),
            archetype_field_name: Some("blob".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                component_name: "rerun.components.MediaType".into(),
                archetype_field_name: Some("media_type".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                component_name: "rerun.components.AssetVideoIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                component_name: "rerun.components.Blob".into(),
                archetype_field_name: Some("blob".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                component_name: "rerun.components.MediaType".into(),
                archetype_field_name: Some("media_type".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                component_name: "rerun.components.AssetVideoIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

impl AssetVideo {
    /// The total number of components in the archetype: 1 required, 2 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`AssetVideo`] [`::re_types_core::Archetype`]
pub type AssetVideoIndicator = ::re_types_core::GenericIndicatorComponent<AssetVideo>;

impl ::re_types_core::Archetype for AssetVideo {
    type Indicator = AssetVideoIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.AssetVideo".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Asset video"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: AssetVideoIndicator = AssetVideoIndicator::DEFAULT;
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
        let blob = {
            let array = arrays_by_name
                .get("rerun.components.Blob")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.AssetVideo#blob")?;
            <crate::components::Blob>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.AssetVideo#blob")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.AssetVideo#blob")?
        };
        let media_type = if let Some(array) = arrays_by_name.get("rerun.components.MediaType") {
            <crate::components::MediaType>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.AssetVideo#media_type")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { blob, media_type })
    }
}

impl ::re_types_core::AsComponents for AssetVideo {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.blob as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                        archetype_field_name: Some(("blob").into()),
                        component_name: ("rerun.components.Blob").into(),
                    }),
                }
            }),
            (self
                .media_type
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.AssetVideo".into()),
                    archetype_field_name: Some(("media_type").into()),
                    component_name: ("rerun.components.MediaType").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for AssetVideo {}

impl AssetVideo {
    /// Create a new `AssetVideo`.
    #[inline]
    pub fn new(blob: impl Into<crate::components::Blob>) -> Self {
        Self {
            blob: blob.into(),
            media_type: None,
        }
    }

    /// The Media Type of the asset.
    ///
    /// Supported values:
    /// * `video/mp4`
    ///
    /// If omitted, the viewer will try to guess from the data blob.
    /// If it cannot guess, it won't be able to render the asset.
    #[inline]
    pub fn with_media_type(mut self, media_type: impl Into<crate::components::MediaType>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for AssetVideo {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.blob.heap_size_bytes() + self.media_type.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::Blob>::is_pod() && <Option<crate::components::MediaType>>::is_pod()
    }
}
