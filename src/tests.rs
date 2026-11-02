use super::*;

#[test]
fn lossless_round_trip() {
    let frame = charls_frame_info {
        width: 3,
        height: 2,
        bits_per_sample: 8,
        component_count: 1,
    };
    let pixels = [0_u8, 127, 255, 42, 64, 192];

    // SAFETY: Handles are checked before use and destroyed after their last use.
    // All buffers remain alive throughout the operations that access them.
    unsafe {
        let encoder = charls_jpegls_encoder_create();
        assert!(!encoder.is_null());
        assert_eq!(
            charls_jpegls_encoder_set_frame_info(encoder, &frame),
            charls_jpegls_errc_success
        );
        assert_eq!(
            charls_jpegls_encoder_set_encoding_options(
                encoder,
                charls_encoding_options_private_encoding_options_include_version_number,
            ),
            charls_jpegls_errc_success
        );
        let mut encoded_size = 0;
        assert_eq!(
            charls_jpegls_encoder_get_estimated_destination_size(encoder, &mut encoded_size),
            charls_jpegls_errc_success
        );
        let mut encoded = vec![0_u8; encoded_size];
        assert_eq!(
            charls_jpegls_encoder_set_destination_buffer(
                encoder,
                encoded.as_mut_ptr().cast(),
                encoded.len(),
            ),
            charls_jpegls_errc_success
        );
        assert_eq!(
            charls_jpegls_encoder_encode_from_buffer(
                encoder,
                pixels.as_ptr().cast(),
                pixels.len(),
                0,
            ),
            charls_jpegls_errc_success
        );
        assert_eq!(
            charls_jpegls_encoder_get_bytes_written(encoder, &mut encoded_size),
            charls_jpegls_errc_success
        );
        charls_jpegls_encoder_destroy(encoder);
        encoded.truncate(encoded_size);

        #[cfg(not(feature = "vcpkg"))]
        assert!(encoded
            .windows(b"charls 2.4.5".len())
            .any(|window| window == b"charls 2.4.5"));

        let decoder = charls_jpegls_decoder_create();
        assert!(!decoder.is_null());
        assert_eq!(
            charls_jpegls_decoder_set_source_buffer(
                decoder,
                encoded.as_ptr().cast(),
                encoded.len(),
            ),
            charls_jpegls_errc_success
        );
        assert_eq!(
            charls_jpegls_decoder_read_header(decoder),
            charls_jpegls_errc_success
        );
        let mut decoded = [0_u8; 6];
        assert_eq!(
            charls_jpegls_decoder_decode_to_buffer(
                decoder,
                decoded.as_mut_ptr().cast(),
                decoded.len(),
                0,
            ),
            charls_jpegls_errc_success
        );
        charls_jpegls_decoder_destroy(decoder);
        assert_eq!(decoded, pixels);
    }
}
