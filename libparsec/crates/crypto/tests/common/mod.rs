// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

macro_rules! test_serde {
    ($name:ident, $cls:ident) => {
        #[test]
        fn $name() {
            const RAW_KEY: [u8; $cls::SIZE] = [42; $cls::SIZE];
            let key = $cls::try_from(&RAW_KEY[..]).unwrap();
            assert_tokens(&key, &[Token::BorrowedBytes(&RAW_KEY)]);
        }
    };
}

macro_rules! test_msgpack_serialization {
    ($name:ident, $cls:ident, $data:expr, $serialized:expr) => {
        #[test]
        fn $name() {
            let raw_data = $data;
            let expected_obj = $cls::try_from(raw_data.as_ref()).expect(concat!(
                "Invalid raw data to create the type: ",
                stringify!($cls)
            ));
            let serialized = $serialized;

            // bytes should be serialized in msgpack using the bin format family
            // (see. https://github.com/msgpack/msgpack/blob/master/spec.md#bin-format-family)

            let sk: $cls = rmp_serde::from_slice(&serialized).unwrap();

            assert_eq!(sk, expected_obj);

            let re_serialized = rmp_serde::to_vec(&sk).unwrap();
            assert_eq!(re_serialized, serialized);

            // str format family shouldn't be used for bytes, but Python's msgpack
            // deserializes it fine, so we should also handle this just in case
            // (see. https://github.com/msgpack/msgpack/blob/master/spec.md#str-format-family)

            assert!(raw_data.len() < 256);
            let mut alternative = vec![0xda, 0x00, raw_data.len() as u8];
            alternative.extend_from_slice(&raw_data);
            let sk: $cls = rmp_serde::from_slice(&alternative).unwrap();
            assert_eq!(sk, expected_obj);
        }
    };
}
