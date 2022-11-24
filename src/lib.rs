#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::error::Error;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub type CharlsResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Default)]
pub struct Decoder {
    decoder: Option<*mut charls_jpegls_decoder>
}

#[derive(Default)]
pub struct Encoder {
    encoder: Option<*mut charls_jpegls_encoder>
}

impl Decoder {
    pub fn new() -> CharlsResult<Self> {
        let decoder = unsafe {
            charls_jpegls_decoder_create()
        };

        if !decoder.is_null() {
            Ok(Decoder {
                decoder: Some(decoder)
            })
        } else {
            Err("Faile to start jpeg-ls decoder".into())
        }
    }

    pub fn decode(&self, src: Vec<u8>, dst: &mut Vec<u8>, stride: u32) -> CharlsResult<()> {
        if let Some(decoder) = self.decoder {
            let err = unsafe {
                charls_jpegls_decoder_set_source_buffer(
                    decoder,
                    src.as_ptr() as _,
                    src.len()
                )
            };

            if err == 0 {
                let err = unsafe {
                    charls_jpegls_decoder_read_header(decoder)
                };

                if err == 0 {
                    let size = unsafe {
                        let mut computed_size: usize = 0;
                        let size_err = charls_jpegls_decoder_get_destination_size(
                            decoder,
                            stride,
                            &mut computed_size
                        );

                        if size_err == 0 {
                            Some(computed_size)
                        } else {
                            None
                        }
                    };

                    match size {
                        Some(size) => {
                            dst.resize(size, 0);
                            let err = unsafe {
                                charls_jpegls_decoder_decode_to_buffer(
                                    decoder,
                                    dst.as_mut_ptr() as _,
                                    size,
                                    stride
                                )
                            };

                            if err == 0 {
                                Ok(())
                            } else {
                                Err("Unable to decode jpeg-ls".into())
                            }
                        }
                        None => Err("Unable to compute decompressed size".into())
                    }
                } else {
                    Err("Unable to read jpeg-ls header".into())
                }
            } else {
                Err("Unable to load the jpeg-ls source".into())
            }
        } else {
            Err("Jpeg-ls is not loaded".into())
        }
    }
}

impl Encoder {
    pub fn new() -> CharlsResult<Self> {
        let encoder = unsafe {
            charls_jpegls_encoder_create()
        };

        if !encoder.is_null() {
            Ok(Encoder {
                encoder: Some(encoder)
            })
        } else {
            Err("Faile to start jpeg-ls decoder".into())
        }
    }

    pub fn encode(&self, width: u32, height: u32, bits_per_sample: i32, component_count: i32, src: &mut Vec<u8>) -> CharlsResult<Vec<u8>> {
        let frame_info = charls_frame_info {
            width,
            height,
            bits_per_sample,
            component_count,
        };

        if let Some(encoder) = self.encoder {
            let err = unsafe {
                charls_jpegls_encoder_set_frame_info(encoder, &frame_info as *const charls_frame_info)
            };

            if err != 0 {
                return Err("Unable to set the frame info".into());
            }

            let mut size = 0;
            let err = unsafe {
                charls_jpegls_encoder_get_estimated_destination_size(encoder, &mut size)
            };

            if err != 0 {
                return Err("Unable to estimage the destination size".into());
            }

            let mut buffer: Vec<u8> = vec![0; size];
            let err = unsafe {
                charls_jpegls_encoder_set_destination_buffer(encoder, buffer.as_mut_ptr() as *mut std::os::raw::c_void, size)
            };

            if err != 0 {
                return Err("Unable to set the destination buffer".into());
            }

            let err = unsafe {
                charls_jpegls_encoder_encode_from_buffer(encoder, src.as_mut_ptr() as *mut std::os::raw::c_void, src.len(), 0)
            };

            if err != 0 {
                return Err("Unable to encode the image".into());
            }

            let err = unsafe {
                charls_jpegls_encoder_get_bytes_written(encoder, &mut size)
            };

            if err != 0 {
                Err("Unable to get written bytes".into())
            } else {
                buffer.truncate(size);
                Ok(buffer)
            }
        } else {
            Err("Encoder not set".into())
        }
    }
}



impl Drop for Decoder {
    fn drop(&mut self) {
        if let Some(decoder) = self.decoder {
            unsafe {
                charls_jpegls_decoder_destroy(decoder);
            }
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        if let Some(encoder) = self.encoder {
            unsafe {
                charls_jpegls_encoder_destroy(encoder);
            }
        }
    }
}
