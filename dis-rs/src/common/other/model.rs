use crate::common::{BodyInfo, Interaction};
use crate::common::other::builder::OtherBuilder;
use crate::common::model::EntityId;
use crate::enumerations::PduType;
use crate::PduBody;

/// A PduBody implementation that contains the body of the PDU as raw bytes, in a vec.
///
/// It also extracts the (optional) originator and receiver of
/// PDUs that convey an interaction between entities (such as Fire PDU),
/// or at least can be attributed to a sending entity (such as EntityState PDU), based on the PduType.
///
/// This struct is used to provide access to the received data in not (yet) supported PDUs.
pub struct Other {
    pub originating_entity_id : Option<EntityId>,
    pub receiving_entity_id : Option<EntityId>,
    pub body: Vec<u8>,
}

impl BodyInfo for Other {
    fn body_length(&self) -> u16 {
        self.body.len() as u16
    }

    fn body_type(&self) -> PduType {
        PduType::Other
    }
}

impl Other {
    // TODO make consistent in using builder lite pattern instead of a full builder
    pub fn builder() -> OtherBuilder {
        OtherBuilder::new()
    }

    pub fn new(bytes: Vec<u8>) -> Self {
        Other {
            originating_entity_id: None,
            receiving_entity_id: None,
            body: bytes,
        }
    }

    pub fn with_origin(mut self, origin: Option<EntityId>) -> Self {
        self.originating_entity_id = origin;
        self
    }

    pub fn with_receiver(mut self, receiver: Option<EntityId>) -> Self {
        self.receiving_entity_id = receiver;
        self
    }

    pub fn as_pdu_body(self) -> PduBody {
        PduBody::Other(self)
    }
}

impl Interaction for Other {
    fn originator(&self) -> Option<&EntityId> {
        if let Some(entity) = &self.originating_entity_id {
            Some(entity)
        } else { None }
    }

    fn receiver(&self) -> Option<&EntityId> {
        if let Some(entity) = &self.receiving_entity_id {
            Some(entity)
        } else { None }
    }
}