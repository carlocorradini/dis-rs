pub trait Codec {
    /// The DIS PDU, Body, Record, ... that is to be converted.
    type Counterpart;
    const SCALING: f32 = 0.0;
    const SCALING_2: f32 = 0.0;
    const CONVERSION: f32 = 0.0;
    const NORMALISATION: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self;
    fn decode(&self) -> Self::Counterpart;
}