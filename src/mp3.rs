// https://github.com/sedmelluq/lavaplayer/blob/master/natives/connector/mpg123.c
// https://github.com/natanbc/lp-cross/blob/master/src/mp3/mp3.c
use jni::{
    objects::{JByteBuffer, JObject},
    sys::{jint, jlong},
    JNIEnv,
};
use rinimp3::{mp3dec_decode_frame, FrameInfo, Mp3Dec};
use std::{mem::forget, ptr, slice};

const MINIMP3_MAX_SAMPLES_PER_FRAME: usize = 2304;

struct Mp3DecLib {
    mp3d: Mp3Dec,
    buffer_offset: usize,
    buffer_remaining: usize,
    buffer: Vec<i16>,
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_create(
    _env: JNIEnv,
    _me: JObject,
) -> jlong {
    let lib = Mp3DecLib {
        mp3d: Mp3Dec::new(),
        buffer_offset: 0,
        buffer_remaining: 0,
        buffer: Vec::with_capacity(MINIMP3_MAX_SAMPLES_PER_FRAME),
    };
    let decoder = Box::new(lib);
    let addr: *mut Mp3DecLib = Box::into_raw(decoder);
    forget(addr);
    addr as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_destroy(
    _env: JNIEnv,
    _me: JObject,
    instance: jlong,
) {
    unsafe {
        let decoder_box: Box<Mp3DecLib> = Box::from_raw(instance as *mut Mp3DecLib);
        drop(decoder_box);
    }
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_decode(
    env: JNIEnv,
    _me: JObject,
    instance: jlong,
    direct_input: JByteBuffer,
    _input_length: jint,
    direct_output: JByteBuffer,
    output_length_i32: jint,
) -> jint {
    let mut lib: Box<Mp3DecLib> = unsafe { Box::from_raw(instance as *mut Mp3DecLib) };

    let input_data = env.get_direct_buffer_address(direct_input).unwrap();
    let output = env
        .get_direct_buffer_address(direct_output)
        .unwrap()
        .as_mut_ptr();

    let output_length = output_length_i32 as usize;

    if lib.buffer_remaining > 0 {
        let nbytes = {
            if output_length < lib.buffer_remaining {
                output_length
            } else {
                lib.buffer_remaining
            }
        };
        unsafe {
            ptr::copy_nonoverlapping(lib.buffer[lib.buffer_offset] as *mut u8, output, nbytes)
        }
    }

    let mut info = FrameInfo::default();

    let return_value: i32 = {
        if output_length < MINIMP3_MAX_SAMPLES_PER_FRAME {
            let nbytes =
                2 * mp3dec_decode_frame(&mut lib.mp3d, input_data, &mut lib.buffer, &mut info)
                    as usize;
            if nbytes <= output_length {
                unsafe { ptr::copy_nonoverlapping(lib.buffer.as_ptr(), output as *mut i16, nbytes) }
                nbytes as i32
            } else {
                unsafe {
                    ptr::copy_nonoverlapping(lib.buffer.as_ptr(), output as *mut i16, output_length)
                }
                lib.buffer_offset = output_length;
                lib.buffer_remaining = nbytes - output_length;
                output_length_i32
            }
        } else {
            let output_slice =
                unsafe { slice::from_raw_parts_mut(output as *mut i16, output_length) };
            2 * mp3dec_decode_frame(&mut lib.mp3d, input_data, output_slice, &mut info)
        }
    };
    forget(lib);

    return_value
}
