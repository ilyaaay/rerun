// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/near_clip_plane.fbs".

#include "near_clip_plane.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {}

namespace rerun {

    Result<std::vector<ComponentBatch>>
        AsComponents<blueprint::archetypes::NearClipPlane>::serialize(
            const blueprint::archetypes::NearClipPlane& archetype
        ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(2);

        {
            auto result = ComponentBatch::from_loggable(
                archetype.near_clip_plane,
                ComponentDescriptor(
                    "rerun.blueprint.archetypes.NearClipPlane",
                    "near_clip_plane",
                    "rerun.blueprint.components.NearClipPlane"
                )
            );
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = NearClipPlane::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
