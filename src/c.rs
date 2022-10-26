use crate::base::{ObjectTransmissionInformation};
use crate::decoder::Decoder as DecoderNative;
use crate::encoder::Encoder as EncoderNative;
use crate::base::EncodingPacket as EncodingPacketNative;

#[repr(C)]
pub struct Encoder {
    encoder: EncoderNative,
}

pub extern "C" fn raptorq_encoder_with_defaults(
    data: *const u8,
    data_len: usize,
    maximum_transmission_unit: u16
) -> *const Encoder {
    let data = unsafe { std::slice::from_raw_parts(data, data_len) };
    let encoder = EncoderNative::with_defaults(data, maximum_transmission_unit);
    Box::into_raw(Box::new(Encoder { encoder }))
}

#[no_mangle]
pub extern "C" fn raptorq_release_encoder(encoder: *mut Encoder) {
    if !encoder.is_null() {
        unsafe {
            Box::from_raw(encoder)
        };
    }
}

#[repr(C)]
pub struct Decoder {
    decoder: DecoderNative,
}

#[no_mangle]
pub extern "C" fn raptorq_decoder_with_defaults(
    transfer_length: u64,
    maximum_transmission_unit: u16
) -> *const Decoder {
    let config = ObjectTransmissionInformation::with_defaults(
        transfer_length,
        maximum_transmission_unit,
    );
    Box::into_raw(Box::new(Decoder { decoder: DecoderNative::new(config) }))
}

#[no_mangle]
pub extern "C" fn raptorq_decode(
    decoder: *mut Decoder,
    packet: *const u8,
    packet_len: usize,
    output: *mut u8,
) -> usize {
    let packet = unsafe { std::slice::from_raw_parts(packet, packet_len) };
    decoder.decoder.decode(EncodingPacketNative::deserialize(packet))
        .map(|data| {
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), output, data.len());
            }
            data.len()
        })
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn raptorq_release_decoder(decoder: *mut Decoder) {
    if !decoder.is_null() {
        unsafe {
            Box::from_raw(decoder)
        };
    }
}

#[repr(C)]
pub struct EncodingPacket {
    next: *const EncodingPacket,
    len: usize,
    data: *const u8,
}

#[no_mangle]
pub extern "C" fn raptorq_encode(
    encoder: *const Encoder,
    repair_packets_per_block: u32
) -> *const EncodingPacket {
    if s.is_null() {
        return std::ptr::null();
    }

    let packets = encoder.encoder.get_encoded_packets(repair_packets_per_block);

    let mut next_packet: *EncodingPacket = std::ptr::null_mut();
    for packet in packets.iter().rev() {
        let serialized_packet = packet.serialize().as_slice();
        let packet = EncodingPacket {
            next: next_packet,
            len: serialized_packet.len(),
            data: serialized_packet.as_ptr(),
        };
        next_packet = Box::into_raw(Box::new(packet));
    }

    return next_packet;
}

#[no_mangle]
pub extern "C" fn raptorq_release_encoding_packet(packet: *mut EncodingPacket) {
    if !packet.is_null() {
        unsafe {
            Box::from_raw(packet)
        };
    }
}
