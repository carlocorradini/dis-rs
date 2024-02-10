# DIS for Rust

dis-rs is an implementation of the Distributed Interactive Simulation (DIS) protocol for Rust. It provides structures and functions to build PDUs in applications, send them out via a network and parse received byte streams into PDUs.

Constructing PDUs is done via builder pattern constructors.

Given a buffer with data from the network, the library can return multiple PDUs in multiple DIS versions present in the buffer.

The library supports both versions 6 and 7 of the standard. As a rule of thumb, the lib is modeled mostly towards supporting v7 (in terms of how the data is modelled), and provides compatibility with how thing were in v6 mostly transparent for the user (i.e., an incoming v6 PDU is parsed and then stored in a v7 model and put back on the wire based on the version specified in the header).

## Features

Here is an overview of the DIS PDUs/features supported by dis-rs. 'Read' means reading a PDU from a byte stream. 'Write' means constructing a PDU in a struct and serializing it to a buffer.

| PDU / function                  | v6 read | v6 write | v7 read | v7 write |
|---------------------------------|---------|----------|---------|----------|
| PDU Header                      | V       | V        | V       | V        | 
| EntityState                     | V       | V        | V       | V        |
| Fire                            | V       | V        | V       | V        |
| Detonation                      | V       | V        | V       | V        |
| Collision                       | V       | V        | V       | V        |
| ServiceRequestPdu               | V       | V        | V       | V        |
| ResupplyOfferPdu                | V       | V        | V       | V        |
| ResupplyReceivedPdu             | V       | V        | V       | V        |
| ResupplyCancelPdu               | V       | V        | V       | V        |
| RepairCompletePdu               | V       | V        | V       | V        |
| RepairResponsePdu               | V       | V        | V       | V        |
| CreateEntityPdu                 | V       | V        | V       | V        |
| RemoveEntityPdu                 | V       | V        | V       | V        |
| StartResumePdu                  | V       | V        | V       | V        |
| StopFreezePdu                   | V       | V        | V       | V        |
| AcknowledgePdu                  | V       | V        | V       | V        |
| ActionRequestPdu                | V       | V        | V       | V        |
| ActionResponsePdu               | V       | V        | V       | V        |
| DataQueryPdu                    | V       | V        | V       | V        |
| SetDataPdu                      | V       | V        | V       | V        |
| DataPdu                         | V       | V        | V       | V        |
| EventReportPdu                  | V       | V        | V       | V        |
| CommentPdu                      | V       | V        | V       | V        |
| ElectromagneticEmissionPdu      | V       | V        | V       | V        |
| DesignatorPdu                   | V       | V        | V       | V        |
| TransmitterPdu                  | V       | V        | V       | V        |
| SignalPdu                       | V       | V        | V       | V        |
| ReceiverPdu                     | V       | V        | V       | V        |
| IFF                             | V       | V        | V       | V        |
| AnnounceObjectPdu               |         |          |         |          |
| DeleteObjectPdu                 |         |          |         |          |
| DescribeApplicationPdu          |         |          |         |          |
| DescribeEventPdu                |         |          |         |          |
| DescribeObjectPdu               |         |          |         |          |
| RequestEventPdu                 |         |          |         |          |
| RequestObjectPdu                |         |          |         |          |
| TimeSpacePositionIndicatorFIPdu |         |          |         |          |
| AppearanceFIPdu                 |         |          |         |          |
| ArticulatedPartsFIPdu           |         |          |         |          |
| FireFIPdu                       |         |          |         |          |
| DetonationFIPdu                 |         |          |         |          |
| PointObjectStatePdu             |         |          |         |          |
| LinearObjectStatePdu            |         |          |         |          |
| ArealObjectStatePdu             |         |          |         |          |
| EnvironmentPdu                  |         |          |         |          |
| TransferControlRequestPdu       |         |          |         |          |
| TransferControlPdu              |         |          |         |          |
| TransferControlAcknowledgePdu   |         |          |         |          |
| IntercomControlPdu              |         |          |         |          |
| IntercomSignalPdu               |         |          |         |          |
| AggregatePdu                    |         |          |         |          |
| 'Other' PDU                     | V       | V        | V       | V        |
| Dead Reckoning Algos            |         |          |         |          |

### Enumerations
dis-rs uses the SISO-REF-010 reference to map the wire level encoding to actual names of enumerations and values in code.
E.g., one can use the enum `PduType::EntityState` in code instead of remembering that a `1` means that specific value.

The code for these enums is generated using a build script from the published SISO-REF-010.xml file. Which is currently a v30.

## Usage

### Constructing PDUs
PDUs are constructed using a `default()` or `builder()` associated functions on the structs for `PduHeader` or `PduBody`s.
Using `SomePdu::builder()` constructs a Builder for the PDU.
The Builders start with the default values for PDU fields, and you have to set fields using `.with_field_name()` functions.
The `build()` function turns the Builder into the typed body of the PDU you are building (e.g., `Signal`).
There are no internal validation or checks whether you construct a PDU with valid combinations of fields.

The main data structure is a `Pdu`, which consists of a `PduHeader` and a `PduBody`. The body is a variant of `PduBody`, an enum that wraps a specific struct for that PDU type in a variant (such as `Pdu::Signal(Signal)`).
The specific body structs, e.g. an `EntityState`, can be wrapped / converted to a `PduBody` by call the `into_pdu_body()` function on the struct.
Further, the body can be merged with a `PduHeader` using the associated function `Pdu::finalize_from_parts(header, body, timestamp)`. This will give you a complete PDU.

### Parsing
The library exposes three functions to parse binary data (the DIS wire format) into PDUs from a buffer: `parse()`, `parse_v6_pdus()` and `parse_v7_pdus()`.
Each function works the same, where the general `parse()` function returns all valid PDUs from the buffer and the others filter out v6 or v7 version PDUs.

### Serializing
To serialize a PDU to bytes (the DIS wire format), simply call the `serialize()` function on a `Pdu`, providing the buffer as argument.

## Resources

- SISO: https://www.sisostds.org - Organisation maintaining the DIS standard and reference material.
- A lot of great background material on DIS can be found at OpenDIS: http://open-dis.org and https://github.com/open-dis.
- Wikipedia: https://en.wikipedia.org/wiki/Distributed_Interactive_Simulation.
- DIS Data Dictionary (version 6 only): http://faculty.nps.edu/brutzman/vrtp/mil/navy/nps/disEnumerations/JdbeHtmlFiles/pdu/.

Copyright (C) 2024 Zeeger Lubsen