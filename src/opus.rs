// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/opus.c
use jni::{
    objects::{JByteBuffer, JObject},
    sys::{jint, jlong},
    JNIEnv,
};
use opus_sys::{
    opus_decode, opus_decoder_create, opus_decoder_destroy, opus_encode, opus_encoder_create,
    opus_encoder_ctl, opus_encoder_destroy, OpusDecoder, OpusEncoder, OPUS_OK,
};
use std::{mem::forget, slice};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_create(
    _env: JNIEnv,
    _me: JObject,
    sample_rate: jint,
    channels: jint,
    application: jint,
    _quality: jint,
) -> jlong {
    let mut error = 0;
    let encoder = unsafe { opus_encoder_create(sample_rate, channels, application, &mut error) };
    if error != OPUS_OK || encoder.is_null() {
        panic!("failed to create opus encoder");
    };
    forget(encoder);
    encoder as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_encode(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_input: JByteBuffer,
    frame_size: jint,
    direct_output: JByteBuffer,
    output_length: jint,
) -> jint {
    let encoder: *mut OpusEncoder = instance as *mut OpusEncoder;
    let input = env
        .get_direct_buffer_address(direct_input)
        .unwrap()
        .as_ptr() as *const i16;
    let output = env
        .get_direct_buffer_address(direct_output)
        .unwrap()
        .as_mut_ptr();
    unsafe { opus_encode(encoder, input, frame_size, output, output_length) }
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    let encoder: *mut OpusEncoder = instance as *mut OpusEncoder;
    unsafe { opus_encoder_destroy(encoder) };
    drop(encoder);
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_create(
    _env: JNIEnv,
    _me: JObject,
    sample_rate: jint,
    channels: jint,
) -> jlong {
    panic!("OpusDecoderLibrary_create called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_decode(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_input: JObject,
    input_size: jint,
    direct_output: JObject,
    frame_size: jint,
) -> jint {
    panic!("OpusDecoderLibrary_decode called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("OpusDecoderLibrary_destroy called");
}
