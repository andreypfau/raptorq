use std::mem::forget;
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jbyte, jbyteArray, jclass, jint, jlong, jobject, jobjectArray, jstring};
use crate::decoder::Decoder;
use crate::encoder::Encoder;
use crate::{EncodingPacket, ObjectTransmissionInformation};

#[no_mangle]
pub extern "system" fn Java_raptorq_JniRaptorQ_encoderWithDefaults(
    env: JNIEnv,
    clazz: JClass,
    data: jbyteArray,
    data_offset: jint,
    data_length: jint,
    maximum_transmission_unit: jint,
) -> jlong {
    let data = env.convert_byte_array(data).unwrap();
    let data = &data[data_offset as usize..data_offset as usize + data_length as usize];
    let encoder = Encoder::with_defaults(data, maximum_transmission_unit as u16);
    Box::into_raw(Box::new(encoder)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_raptorq_JniRaptorQ_encoderEncode(
    env: JNIEnv,
    clazz: JClass,
    encoder: jlong,
    repair_packets_per_block: jint,
) -> jobjectArray {
    let encoder = unsafe {
        Box::from_raw(encoder as *mut Encoder)
    };
    let packets = encoder.get_encoded_packets(repair_packets_per_block as u32);
    let byte_arrays = packets.iter().map(|packet| {
        let serialized_data = packet.serialize();
        env.byte_array_from_slice(&serialized_data).unwrap()
    }).collect::<Vec<jbyteArray>>();
    let len = byte_arrays.len();

    let arrays = env.new_object_array(len as i32, "[B", JObject::null()).unwrap();
    for i in 0..len {
        unsafe {
            env.set_object_array_element(arrays, i as i32, JObject::from_raw(byte_arrays[i])).unwrap();
        }
    }
    forget(encoder);
    arrays
}

#[no_mangle]
pub extern "system" fn Java_raptorq_JniRaptorQ_encoderFree(
    env: JNIEnv,
    clazz: JClass,
    encoder: jlong,
) {
    let encoder = unsafe { Box::from_raw(encoder as *mut Encoder) };
    drop(encoder);
}

#[no_mangle]
pub extern "system" fn Java_raptorq_JniRaptorQ_decoderWithDefaults(
    env: JNIEnv,
    clazz: JClass,
    transfer_length: jlong,
    maximum_transmission_unit: jint,
) -> jlong {
    let config = ObjectTransmissionInformation::with_defaults(
        transfer_length as u64,
        maximum_transmission_unit as u16,
    );
    let decoder = Decoder::new(config);
    Box::into_raw(Box::new(decoder)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_raptorq_JniRaptorQ_decoderDecode(
    env: JNIEnv,
    clazz: JClass,
    decoder: jlong,
    packet: jbyteArray,
    packet_offset: jint,
    packet_len: jint,
    output: jbyteArray,
    output_offset: jint,
) -> jboolean {
    let mut decoder = unsafe {
        Box::from_raw(decoder as *mut Decoder)
    };
    let packet = env.convert_byte_array(packet).unwrap();
    let packet = &packet[packet_offset as usize..packet_offset as usize + packet_len as usize];
    let encoding_packet = EncodingPacket::deserialize(packet);
    let result = decoder.decode(encoding_packet).map(|packet| {
        let data = packet.iter().map(|&x| x as jbyte).collect::<Vec<jbyte>>();
        env.set_byte_array_region(output, output_offset, data.as_slice()).unwrap();
        1 as jboolean
    }).unwrap_or(0 as jboolean);
    forget(decoder);
    result
}

#[no_mangle]
pub extern "system" fn Java_raptorq_JniRaptorQ_decoderFree(
    env: JNIEnv,
    clazz: JClass,
    decoder: jlong,
) {
    let decoder = unsafe { Box::from_raw(decoder as *mut Decoder) };
    drop(decoder);
}
