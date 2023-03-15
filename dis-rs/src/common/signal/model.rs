use crate::common::model::EntityId;
use crate::enumerations::{SignalTdlType, SignalEncodingType, SignalEncodingClass, SignalUserProtocolIdentificationNumber};
use crate::{PduBody, PduType};
use crate::common::{BodyInfo, Interaction};

pub const BASE_SIGNAL_BODY_LENGTH : u16 = 20;

pub struct Signal {
    pub radio_reference_id: EntityId,
    pub radio_number: u16,
    pub encoding_scheme: EncodingScheme,
    pub tdl_type: SignalTdlType,
    pub sample_rate: u32,
    pub samples: u16,
    pub data: Vec<u8>,
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}

impl Signal {
    pub fn new() -> Self {
        Self {
            radio_reference_id: Default::default(),
            radio_number: 0,
            encoding_scheme: EncodingScheme::EncodedAudio { encoding_class: SignalEncodingClass::Encodedaudio, encoding_type: SignalEncodingType::_8bitmulaw_ITUTG_711_1 },
            tdl_type: SignalTdlType::Other_0,
            sample_rate: 0,
            samples: 0,
            data: vec![],
        }
    }

    pub fn with_radio_reference_id(mut self, radio_reference_id: EntityId) -> Self {
        self.radio_reference_id = radio_reference_id;
        self
    }

    pub fn with_radio_number(mut self, radio_number: u16) -> Self {
        self.radio_number = radio_number;
        self
    }

    pub fn with_encoding_scheme(mut self, encoding_scheme: EncodingScheme) -> Self {
        self.encoding_scheme = encoding_scheme;
        self
    }

    pub fn with_tdl_type(mut self, tdl_type: SignalTdlType) -> Self {
        self.tdl_type = tdl_type;
        self
    }

    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn with_samples(mut self, samples: u16) -> Self {
        self.samples = samples;
        self
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    pub fn data_length_padded(&self) -> usize {
        const FOUR_OCTETS : usize = 4;
        const NO_REMAINDER : usize = 0;
        let data_remaining_bytes = self.data.len() % FOUR_OCTETS;
        let padding_bytes = FOUR_OCTETS - data_remaining_bytes;
        let padded_data_bytes = self.data.len() + padding_bytes;
        assert_eq!(padded_data_bytes % FOUR_OCTETS, NO_REMAINDER,
                   "The length for the Signal data record is not aligned to 4 octets. Data length is {} octets.", self.data.len());

        padded_data_bytes
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Signal(self)
    }
}

impl BodyInfo for Signal {
    fn body_length(&self) -> u16 {
        BASE_SIGNAL_BODY_LENGTH + self.data_length_padded() as u16
    }

    fn body_type(&self) -> PduType {
        PduType::Signal
    }
}

impl Interaction for Signal {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.radio_reference_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

pub enum EncodingScheme {
    EncodedAudio { encoding_class: SignalEncodingClass, encoding_type: SignalEncodingType },
    RawBinaryData { encoding_class: SignalEncodingClass, nr_of_messages: u16 },
    ApplicationSpecificData { encoding_class: SignalEncodingClass, user_protocol_id: SignalUserProtocolIdentificationNumber },
    DatabaseIndex { encoding_class: SignalEncodingClass, index: u32, offset_milli_secs: u32, duration_milli_secs: u32 },
}