// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/samplerate.c
use jni::{
    objects::JObject,
    sys::{jboolean, jdouble, jfloatArray, jint, jintArray, jlong},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_create(
    env: JNIEnv,
    me: JObject,
    r#type: jint,
    channels: jint,
) -> jlong {
    panic!("SampleRateLibrary_create called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("SampleRateLibrary_destroy called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_reset(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("SampleRateLibrary_reset called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_process(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    in_array: jfloatArray,
    in_offset: jint,
    in_length: jint,
    out_array: jfloatArray,
    out_offset: jint,
    out_length: jint,
    end_of_input: jboolean,
    source_ratio: jdouble,
    progress_array: jintArray,
) -> jint {
    panic!("SampleRateLibrary_process called");
}
