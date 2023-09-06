// DO NOT EDIT!: This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs:54.
// Based on "crates/re_types/definitions/rerun/datatypes/tensor_dimension.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

namespace arrow {
    class DataType;
    class MemoryPool;
    class StructBuilder;
} // namespace arrow

namespace rerun {
    namespace datatypes {
        /// A single dimension within a multi-dimensional tensor.
        struct TensorDimension {
            uint64_t size;

            std::optional<std::string> name;

          public:
            TensorDimension() = default;

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::StructBuilder* builder, const TensorDimension* elements, size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rerun