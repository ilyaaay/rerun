// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_cast)]

#[doc = "A unique numeric identifier for each individual instance within a batch."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstanceKey(pub u64);

impl<'a> From<InstanceKey> for ::std::borrow::Cow<'a, InstanceKey> {
    #[inline]
    fn from(value: InstanceKey) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a InstanceKey> for ::std::borrow::Cow<'a, InstanceKey> {
    #[inline]
    fn from(value: &'a InstanceKey) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Component for InstanceKey {
    #[inline]
    fn name() -> crate::ComponentName {
        crate::ComponentName::Borrowed("rerun.instance_key")
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Extension(
            "rerun.components.InstanceKey".to_owned(),
            Box::new(DataType::UInt64),
            None,
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
        use crate::{Component as _, Datatype as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                {
                    _ = extension_wrapper;
                    DataType::Extension(
                        "rerun.components.InstanceKey".to_owned(),
                        Box::new(DataType::UInt64),
                        None,
                    )
                },
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
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
        use crate::{Component as _, Datatype as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok(data
            .as_any()
            .downcast_ref::<UInt64Array>()
            .unwrap()
            .into_iter()
            .map(|v| v.copied())
            .map(|v| {
                v.ok_or_else(|| crate::DeserializationError::MissingData {
                    datatype: data.data_type().clone(),
                })
            })
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?)
    }
}
