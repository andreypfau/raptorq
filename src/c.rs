use std::cmp::min;
use std::ptr::slice_from_raw_parts;
use std::slice;
use std::slice::Iter;

use crate::base::ObjectTransmissionInformation;
use crate::base::EncodingPacket as EncodingPacketNative;
use crate::decoder::Decoder as DecoderNative;
use crate::encoder::Encoder as EncoderNative;

#[repr(C)]
pub struct Encoder {
    encoder: EncoderNative,
}

#[repr(C)]
pub struct Decoder {
    decoder: DecoderNative,
}

#[repr(C)]
pub struct EncodingPacket {
    data: *mut u8,
    data_len: usize,
    next: Option<Box<EncodingPacket>>,
}

#[no_mangle]
pub extern "C" fn raptorq_encoder_with_defaults(
    data: *const u8,
    data_len: usize,
    maximum_transmission_unit: u16,
) -> Box<Encoder> {
    let data = unsafe {
        assert!(!data.is_null());
        slice::from_raw_parts(data, data_len)
    };
    let encoder = EncoderNative::with_defaults(data, maximum_transmission_unit);
    Box::new(Encoder { encoder })
}

#[no_mangle]
pub extern "C" fn raptorq_encoder_encode(
    encoder: *const Encoder,
    repair_packets_per_block: u32,
) -> Option<Box<EncodingPacket>> {
    unsafe {
        if encoder.is_null() {
            return None;
        }
    }
    let packets = unsafe {
        (*encoder).encoder.get_encoded_packets(repair_packets_per_block)
    };
    let mut next: Option<Box<EncodingPacket>> = None;
    for packet in packets.iter().rev() {
        let mut serialized_data = packet.serialize().into_boxed_slice();
        let data = serialized_data.as_mut_ptr();
        let data_len = serialized_data.len();
        std::mem::forget(serialized_data);
        let new_packet = EncodingPacket { data, data_len, next };
        next = Some(Box::new(new_packet));
    }
    next
}

#[no_mangle]
pub extern "C" fn raptorq_encoding_packet_free(packet: Option<Box<EncodingPacket>>) {
    if packet.is_none() {
        return;
    }
    let packet = packet.unwrap();
    unsafe {
        let s = std::slice::from_raw_parts_mut(packet.data, packet.data_len).as_mut_ptr();
        Box::from_raw(s);
    };
}

#[no_mangle]
pub extern "C" fn raptorq_encoder_free(_: Option<Box<Encoder>>) {}

#[no_mangle]
pub extern "C" fn raptorq_decoder_with_defaults(
    transfer_length: u64,
    maximum_transmission_unit: u16,
) -> Box<Decoder> {
    let config = ObjectTransmissionInformation::with_defaults(
        transfer_length,
        maximum_transmission_unit,
    );
    Box::new(Decoder { decoder: DecoderNative::new(config) })
}

#[no_mangle]
pub extern "C" fn raptorq_decoder_decode(
    decoder: *mut Decoder,
    packet: *const u8,
    packet_len: usize,
    output: *mut u8,
) -> usize {
    unsafe {
        if decoder.is_null() {
            return 0;
        }
    }
    let packet = unsafe { std::slice::from_raw_parts(packet, packet_len) };
    let encoding_packet = EncodingPacketNative::deserialize(packet);
    unsafe {
        (*decoder).decoder.decode(encoding_packet).map(|data| {
            let data_len = data.len();
            std::ptr::copy_nonoverlapping(data.as_ptr(), output, data_len);
            data_len
        }).unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn raptorq_decoder_free(_: Option<Box<Decoder>>) {}
