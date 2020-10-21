// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/linux/statistics.c
use jni::{objects::JObject, sys::jlongArray, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_statistics_CpuStatisticsLibrary_getSystemTimes(
    env: JNIEnv,
    me: JObject,
    value_array: jlongArray,
) {
    panic!("CpuStatisticsLibrary_getSystemTimes called");
}
