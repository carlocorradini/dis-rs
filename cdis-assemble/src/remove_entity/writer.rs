use crate::remove_entity::model::RemoveEntity;
use crate::{BitBuffer, SerializeCdisPdu};
use crate::writing::SerializeCdis;

impl SerializeCdisPdu for RemoveEntity {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);
        let cursor = self.request_id.serialize(buf, cursor);

        cursor
    }
}