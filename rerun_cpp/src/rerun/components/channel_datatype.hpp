// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/channel_datatype.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class Array;
    class DataType;
    class UInt8Type;
    using UInt8Builder = NumericBuilder<UInt8Type>;
} // namespace arrow

namespace rerun::components {
    /// **Component**: The innermost datatype of an image.
    ///
    /// How individual color channel components are encoded.
    enum class ChannelDatatype : uint8_t {

        /// 8-bit unsigned integer.
        U8 = 0,

        /// 16-bit unsigned integer.
        U16 = 1,

        /// 32-bit unsigned integer.
        U32 = 2,

        /// 64-bit unsigned integer.
        U64 = 3,

        /// 8-bit signed integer.
        I8 = 4,

        /// 16-bit signed integer.
        I16 = 5,

        /// 32-bit signed integer.
        I32 = 6,

        /// 64-bit signed integer.
        I64 = 7,

        /// 16-bit IEEE-754 floating point, also known as `half`.
        F16 = 8,

        /// 32-bit IEEE-754 floating point, also known as `float` or `single`.
        F32 = 9,

        /// 64-bit IEEE-754 floating point, also known as `double`.
        F64 = 10,
    };
} // namespace rerun::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<components::ChannelDatatype> {
        static constexpr const char Name[] = "rerun.components.ChannelDatatype";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::components::ChannelDatatype` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::ChannelDatatype* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::UInt8Builder* builder, const components::ChannelDatatype* elements,
            size_t num_elements
        );
    };
} // namespace rerun