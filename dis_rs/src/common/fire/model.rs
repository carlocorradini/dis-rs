use crate::common::Interaction;
use crate::common::model::{BurstDescriptor, EntityId};
use crate::common::model::EventId;
use crate::common::model::Location;
use crate::common::model::VectorF32;

// #[derive(buildstructor::Builder)]
pub struct Fire {
    pub firing_entity_id : EntityId,
    pub target_entity_id : EntityId,
    pub munition_id : EntityId,
    pub event_id : EventId,
    pub fire_mission_index : u32,
    pub location_in_world : Location,
    pub burst_descriptor : BurstDescriptor,
    pub velocity : VectorF32,
    pub range : f32,
}

impl Interaction for Fire {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.firing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}