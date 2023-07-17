// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// Representation of a 3D affine transform.
///
/// Rarely used directly, prefer using the underlying representation classes and pass them
/// directly to `Transform3D::child_from_parent` or `Transform3D::parent_from_child`.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Transform3D {
    TranslationAndMat3X3(crate::datatypes::TranslationAndMat3x3),
    TranslationRotationScale(crate::datatypes::TranslationRotationScale3D),
}

impl<'a> From<Transform3D> for ::std::borrow::Cow<'a, Transform3D> {
    #[inline]
    fn from(value: Transform3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Transform3D> for ::std::borrow::Cow<'a, Transform3D> {
    #[inline]
    fn from(value: &'a Transform3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for Transform3D {
    type Name = crate::DatatypeName;
    #[inline]
    fn name() -> Self::Name {
        crate::DatatypeName::Borrowed("rerun.datatypes.Transform3D")
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Union(
            vec![
                Field {
                    name: "TranslationAndMat3x3".to_owned(),
                    data_type: DataType::Struct(vec![
                        Field {
                            name: "translation".to_owned(),
                            data_type: DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::Float32,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                3usize,
                            ),
                            is_nullable: true,
                            metadata: [].into(),
                        },
                        Field {
                            name: "matrix".to_owned(),
                            data_type: DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::Float32,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                9usize,
                            ),
                            is_nullable: true,
                            metadata: [].into(),
                        },
                        Field {
                            name: "from_parent".to_owned(),
                            data_type: DataType::Boolean,
                            is_nullable: false,
                            metadata: [].into(),
                        },
                    ]),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationRotationScale".to_owned(),
                    data_type: DataType::Struct(vec![
                        Field {
                            name: "translation".to_owned(),
                            data_type: DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::Float32,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                3usize,
                            ),
                            is_nullable: true,
                            metadata: [].into(),
                        },
                        Field {
                            name: "rotation".to_owned(),
                            data_type: DataType::Union(
                                vec![
                                    Field {
                                        name: "Quaternion".to_owned(),
                                        data_type: DataType::FixedSizeList(
                                            Box::new(Field {
                                                name: "item".to_owned(),
                                                data_type: DataType::Float32,
                                                is_nullable: false,
                                                metadata: [].into(),
                                            }),
                                            4usize,
                                        ),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    },
                                    Field {
                                        name: "AxisAngle".to_owned(),
                                        data_type: DataType::Struct(vec![
                                            Field {
                                                name: "axis".to_owned(),
                                                data_type: DataType::FixedSizeList(
                                                    Box::new(Field {
                                                        name: "item".to_owned(),
                                                        data_type: DataType::Float32,
                                                        is_nullable: false,
                                                        metadata: [].into(),
                                                    }),
                                                    3usize,
                                                ),
                                                is_nullable: false,
                                                metadata: [].into(),
                                            },
                                            Field {
                                                name: "angle".to_owned(),
                                                data_type: DataType::Union(
                                                    vec![
                                                        Field {
                                                            name: "Radians".to_owned(),
                                                            data_type: DataType::Float32,
                                                            is_nullable: false,
                                                            metadata: [].into(),
                                                        },
                                                        Field {
                                                            name: "Degrees".to_owned(),
                                                            data_type: DataType::Float32,
                                                            is_nullable: false,
                                                            metadata: [].into(),
                                                        },
                                                    ],
                                                    None,
                                                    UnionMode::Dense,
                                                ),
                                                is_nullable: false,
                                                metadata: [].into(),
                                            },
                                        ]),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    },
                                ],
                                None,
                                UnionMode::Dense,
                            ),
                            is_nullable: true,
                            metadata: [].into(),
                        },
                        Field {
                            name: "scale".to_owned(),
                            data_type: DataType::Union(
                                vec![
                                    Field {
                                        name: "ThreeD".to_owned(),
                                        data_type: DataType::FixedSizeList(
                                            Box::new(Field {
                                                name: "item".to_owned(),
                                                data_type: DataType::Float32,
                                                is_nullable: false,
                                                metadata: [].into(),
                                            }),
                                            3usize,
                                        ),
                                        is_nullable: false,
                                        metadata: [].into(),
                                    },
                                    Field {
                                        name: "Uniform".to_owned(),
                                        data_type: DataType::Float32,
                                        is_nullable: false,
                                        metadata: [].into(),
                                    },
                                ],
                                None,
                                UnionMode::Dense,
                            ),
                            is_nullable: true,
                            metadata: [].into(),
                        },
                        Field {
                            name: "from_parent".to_owned(),
                            data_type: DataType::Boolean,
                            is_nullable: false,
                            metadata: [].into(),
                        },
                    ]),
                    is_nullable: false,
                    metadata: [].into(),
                },
            ],
            None,
            UnionMode::Dense,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            UnionArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(<crate::datatypes::Transform3D>::to_arrow_datatype()),
                        None,
                    )
                } else {
                    <crate::datatypes::Transform3D>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                {
                    data.iter()
                        .flatten()
                        .map(|v| match **v {
                            Transform3D::TranslationAndMat3X3(_) => 0i8,
                            Transform3D::TranslationRotationScale(_) => 1i8,
                        })
                        .collect()
                },
                vec![
                    {
                        let (somes, translation_and_mat_3_x_3): (Vec<_>, Vec<_>) = data
                            .iter()
                            .flatten()
                            .filter(|datum| {
                                matches!(***datum, Transform3D::TranslationAndMat3X3(_))
                            })
                            .map(|datum| {
                                let datum = match &**datum {
                                    Transform3D::TranslationAndMat3X3(v) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_and_mat_3_x_3_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = translation_and_mat_3_x_3_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::TranslationAndMat3x3::try_to_arrow_opt(
                                translation_and_mat_3_x_3,
                                None::<&str>,
                            )?
                        }
                    },
                    {
                        let (somes, translation_rotation_scale): (Vec<_>, Vec<_>) = data
                            .iter()
                            .flatten()
                            .filter(|datum| {
                                matches!(***datum, Transform3D::TranslationRotationScale(_))
                            })
                            .map(|datum| {
                                let datum = match &**datum {
                                    Transform3D::TranslationRotationScale(v) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_rotation_scale_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = translation_rotation_scale_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::TranslationRotationScale3D::try_to_arrow_opt(
                                translation_rotation_scale,
                                None::<&str>,
                            )?
                        }
                    },
                ],
                Some({
                    let mut translation_and_mat_3_x_3_offset = 0;
                    let mut translation_rotation_scale_offset = 0;
                    data.iter()
                        .flatten()
                        .map(|v| match **v {
                            Transform3D::TranslationAndMat3X3(_) => {
                                let offset = translation_and_mat_3_x_3_offset;
                                translation_and_mat_3_x_3_offset += 1;
                                offset
                            }

                            Transform3D::TranslationRotationScale(_) => {
                                let offset = translation_rotation_scale_offset;
                                translation_rotation_scale_offset += 1;
                                offset
                            }
                        })
                        .collect()
                }),
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::UnionArray>()
                .ok_or_else(|| crate::DeserializationError::SchemaMismatch {
                    expected: data.data_type().clone(),
                    got: data.data_type().clone(),
                })?;
            if data.is_empty() {
                Vec::new()
            } else {
                let (data_types, data_arrays, data_offsets) =
                    (data.types(), data.fields(), data.offsets().unwrap());
                let translation_and_mat_3_x_3 = {
                    let data = &*data_arrays[0usize];

                    crate::datatypes::TranslationAndMat3x3::try_from_arrow_opt(data)?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                let translation_rotation_scale = {
                    let data = &*data_arrays[1usize];

                    crate::datatypes::TranslationRotationScale3D::try_from_arrow_opt(data)?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = data_offsets[i];

                        Ok(Some(match typ {
                            0i8 => Transform3D::TranslationAndMat3X3(
                                translation_and_mat_3_x_3
                                    .get(offset as usize)
                                    .ok_or_else(|| crate::DeserializationError::OffsetsMismatch {
                                        bounds: (offset as usize, offset as usize),
                                        len: translation_and_mat_3_x_3.len(),
                                        datatype: data.data_type().clone(),
                                    })?
                                    .clone()
                                    .unwrap(),
                            ),
                            1i8 => Transform3D::TranslationRotationScale(
                                translation_rotation_scale
                                    .get(offset as usize)
                                    .ok_or_else(|| crate::DeserializationError::OffsetsMismatch {
                                        bounds: (offset as usize, offset as usize),
                                        len: translation_rotation_scale.len(),
                                        datatype: data.data_type().clone(),
                                    })?
                                    .clone()
                                    .unwrap(),
                            ),
                            _ => unreachable!(),
                        }))
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()?
            }
        })
    }
}

impl crate::Datatype for Transform3D {}