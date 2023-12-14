// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AffixFuzzer22 {
    pub fixed_sized_native: [u8; 4usize],
}

impl From<[u8; 4usize]> for AffixFuzzer22 {
    #[inline]
    fn from(fixed_sized_native: [u8; 4usize]) -> Self {
        Self { fixed_sized_native }
    }
}

impl From<AffixFuzzer22> for [u8; 4usize] {
    #[inline]
    fn from(value: AffixFuzzer22) -> Self {
        value.fixed_sized_native
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer22);

impl ::re_types_core::Loggable for AffixFuzzer22 {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.datatypes.AffixFuzzer22".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![Field {
            name: "fixed_sized_native".to_owned(),
            data_type: DataType::FixedSizeList(
                Box::new(Field {
                    name: "item".to_owned(),
                    data_type: DataType::UInt8,
                    is_nullable: false,
                    metadata: [].into(),
                }),
                4usize,
            ),
            is_nullable: false,
            metadata: [].into(),
        }])
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::testing::datatypes::AffixFuzzer22>::arrow_datatype(),
                vec![{
                    let (somes, fixed_sized_native): (Vec<_>, Vec<_>) = data
                        .iter()
                        .map(|datum| {
                            let datum = datum.as_ref().map(|datum| {
                                let Self {
                                    fixed_sized_native, ..
                                } = &**datum;
                                fixed_sized_native.clone()
                            });
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let fixed_sized_native_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                        let fixed_sized_native_inner_data: Vec<_> = fixed_sized_native
                            .iter()
                            .flat_map(|v| match v {
                                Some(v) => itertools::Either::Left(v.iter().cloned()),
                                None => itertools::Either::Right(
                                    std::iter::repeat(Default::default()).take(4usize),
                                ),
                            })
                            .map(Some)
                            .collect();
                        let fixed_sized_native_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                            fixed_sized_native_bitmap.as_ref().map(|bitmap| {
                                bitmap
                                    .iter()
                                    .map(|i| std::iter::repeat(i).take(4usize))
                                    .flatten()
                                    .collect::<Vec<_>>()
                                    .into()
                            });
                        FixedSizeListArray::new(
                            DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::UInt8,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                4usize,
                            ),
                            PrimitiveArray::new(
                                DataType::UInt8,
                                fixed_sized_native_inner_data
                                    .into_iter()
                                    .map(|v| v.unwrap_or_default())
                                    .collect(),
                                fixed_sized_native_inner_bitmap,
                            )
                            .boxed(),
                            fixed_sized_native_bitmap,
                        )
                        .boxed()
                    }
                }],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![Field {
                            name: "fixed_sized_native".to_owned(),
                            data_type: DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::UInt8,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                4usize,
                            ),
                            is_nullable: false,
                            metadata: [].into(),
                        }]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.testing.datatypes.AffixFuzzer22")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let fixed_sized_native = {
                    if !arrays_by_name.contains_key("fixed_sized_native") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "fixed_sized_native",
                        ))
                        .with_context("rerun.testing.datatypes.AffixFuzzer22");
                    }
                    let arrow_data = &**arrays_by_name["fixed_sized_native"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                DeserializationError::datatype_mismatch(
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::UInt8,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        4usize,
                                    ),
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context(
                                "rerun.testing.datatypes.AffixFuzzer22#fixed_sized_native",
                            )?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(4usize)
                                .zip((4usize..).step_by(4usize).take(arrow_data.len()));
                            let arrow_data_inner =
                                {
                                    let arrow_data_inner = &**arrow_data.values();
                                    arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<UInt8Array>()
                                    .ok_or_else(|| DeserializationError::datatype_mismatch(
                                        DataType::UInt8,
                                        arrow_data_inner.data_type().clone(),
                                    ))
                                    .with_context(
                                        "rerun.testing.datatypes.AffixFuzzer22#fixed_sized_native",
                                    )?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                                };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets,
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, end)| {
                                    debug_assert!(end - start == 4usize);
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_inner.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data.iter().cloned().map(Option::unwrap_or_default);
                                    let arr = array_init::from_iter(data).unwrap();
                                    Ok(arr)
                                })
                                .transpose()
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(fixed_sized_native),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(fixed_sized_native)| {
                        Ok(Self {
                            fixed_sized_native: fixed_sized_native
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.testing.datatypes.AffixFuzzer22#fixed_sized_native",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.datatypes.AffixFuzzer22")?
            }
        })
    }
}