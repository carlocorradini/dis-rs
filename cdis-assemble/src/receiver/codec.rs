use bitvec::macros::internal::funty::Fundamental;
use num_traits::FromPrimitive;
use crate::receiver::model::Receiver;
use crate::records::model::EntityId;
use crate::codec::Codec;
use crate::types::model::UVINT16;

type Counterpart = dis_rs::receiver::model::Receiver;

impl Receiver {
    pub fn encode(item: &Counterpart) -> Self {
        Self {
            radio_reference_id: EntityId::encode(&item.radio_reference_id),
            radio_number: UVINT16::from(item.radio_number),
            receiver_state: item.receiver_state,
            received_power: i16::from_f32(item.received_power).unwrap_or_default(),
            transmitter_radio_reference_id: EntityId::encode(&item.transmitter_radio_reference_id),
            transmitter_radio_number: UVINT16::from(item.transmitter_radio_number),
        }
    }

    pub fn decode(&self) -> Counterpart {
        Counterpart::builder()
            .with_radio_reference_id(self.radio_reference_id.decode())
            .with_radio_number(self.radio_number.value)
            .with_receiver_state(self.receiver_state)
            .with_received_power(self.received_power.as_f32())
            .with_transmitter_radio_reference_id(self.transmitter_radio_reference_id.decode())
            .with_transmitter_radio_number(self.transmitter_radio_number.value)
            .build()
    }
}