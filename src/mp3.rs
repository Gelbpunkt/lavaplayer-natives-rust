// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/mpg123.c
// https://github.com/natanbc/lp-cross/blob/master/src/mp3/mp3.c
use jni::{
    objects::JObject,
    sys::{jint, jlong},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_create(
    env: JNIEnv,
    me: JObject,
) -> jlong {
    panic!("Mp3DecoderLibrary_create called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("Mp3DecoderLibrary_destroy called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_decode(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_input: JObject,
    input_length: jint,
    direct_output: JObject,
    output_length: jint,
) -> jint {
    panic!("Mp3DecoderLibrary_decode called");
}
