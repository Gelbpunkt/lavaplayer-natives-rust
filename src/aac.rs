// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/fdk-aac.c
use jni::{
    objects::JObject,
    sys::{jboolean, jint, jlong},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_create(
    env: JNIEnv,
    me: JObject,
    transport_type: jint,
) -> jlong {
    panic!("AacDecoderLibrary_create called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("AacDecoderLibrary_destroy called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_configure(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    buffer_data: jlong,
) -> jint {
    panic!("AacDecoderLibrary_configure called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_fill(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_buffer: JObject,
    offset: jint,
    length: jint,
) -> jint {
    panic!("AacDecoderLibrary_fill called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_decode(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_buffer: JObject,
    length: jint,
    flush: jboolean,
) -> jint {
    panic!("AacDecoderLibrary_decode called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_getStreamInfo(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) -> jlong {
    panic!("AacDecoderLibrary_getStreamInfo called");
}
