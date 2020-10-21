// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/vorbis.c
use jni::{
    objects::JObject,
    sys::{jint, jlong, jobjectArray},
    JNIEnv,
};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_create(
    env: JNIEnv,
    me: JObject,
) -> jlong {
    panic!("VorbisDecoderLibrary_create called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_initialise(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    id_direct_buffer: JObject,
    id_offset: jint,
    id_length: jint,
    setup_direct_buffer: JObject,
    setup_offset: jint,
    setup_length: jint,
) -> jint {
    panic!("VorbisDecoderLibrary_initialise called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_getChannelCount(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) -> jint {
    panic!("VorbisDecoderLibrary_getChannelCount called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_input(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    direct_buffer: JObject,
    offset: jint,
    length: jint,
) -> jint {
    panic!("VorbisDecoderLibrary_input called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_output(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
    channels: jobjectArray,
    length: jint,
) -> jint {
    panic!("VorbisDecoderLibrary_output called");
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_destroy(
    env: JNIEnv,
    me: JObject,
    instance: jlong,
) {
    panic!("VorbisDecoderLibrary_destroy called");
}
