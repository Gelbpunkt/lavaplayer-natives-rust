// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/opus.c
use jni::{
    objects::JObject,
    sys::{jint, jlong},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_create(
    env: JNIEnv,
    me: JObject,
    sample_rate: jint,
    channels: jint,
    application: jint,
    quality: jint,
) -> jlong {
    panic!("OpusEncoderLibrary_create called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_encode(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_input: JObject,
    frame_size: jint,
    direct_output: JObject,
    output_length: jint,
) -> jint {
    panic!("OpusEncoderLibrary_encode called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("OpusEncoderLibrary_destroy called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_create(
    env: JNIEnv,
    me: JObject,
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
