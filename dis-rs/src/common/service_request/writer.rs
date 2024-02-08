use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::service_request::model::{ServiceRequest, SupplyQuantity};

impl SerializePdu for ServiceRequest {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let requesting_id_bytes = self.requesting_id.serialize(buf);
        let servicing_id_bytes = self.servicing_id.serialize(buf);
        buf.put_u8(self.service_type_requested.into());
        buf.put_u8(self.supplies.len() as u8);
        buf.put_u16(0u16);
        let supply_quantity_bytes = self.supplies.iter().map(|sq| sq.serialize(buf) ).sum::<u16>();

        requesting_id_bytes + servicing_id_bytes + 4 + supply_quantity_bytes
    }
}

impl Serialize for SupplyQuantity {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let type_bytes = self.supply_type.serialize(buf);
        buf.put_f32(self.quantity);

        type_bytes + 4
    }
}