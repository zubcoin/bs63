mod cases;

#[cfg(feature = "check")]
use assert_matches::assert_matches;

#[test]
fn test_decode() {
    for &(val, s) in cases::TEST_CASES.iter() {
        assert_eq!(val.to_vec(), bs63::decode(s).into_vec().unwrap());
    }
}

#[test]
fn test_decode_small_buffer_err() {
    let mut output = [0; 2];
    assert_eq!(
        bs63::decode("a3gV").into(&mut output),
        Err(bs63::decode::Error::BufferTooSmall)
    );
}

#[test]
fn test_decode_invalid_char() {
    let sample = "123456789abcd!efghij";
    assert_eq!(
        bs63::decode(sample).into_vec().unwrap_err(),
        bs63::decode::Error::InvalidCharacter {
            character: '!',
            index: 13
        }
    );
}

#[test]
#[cfg(feature = "check")]
fn test_decode_check() {
    for &(val, s) in cases::CHECK_TEST_CASES.iter() {
        assert_eq!(
            val.to_vec(),
            bs63::decode(s).with_check(None).into_vec().unwrap()
        );
    }

    for &(val, s) in cases::CHECK_TEST_CASES[1..].iter() {
        assert_eq!(
            val.to_vec(),
            bs63::decode(s).with_check(Some(val[0])).into_vec().unwrap()
        );
    }
}

#[test]
#[cfg(feature = "check")]
fn test_check_ver_failed() {
    let d = bs63::decode("K5zqBMZZTzUbAZQgrt4")
        .with_check(Some(0x01))
        .into_vec();

    assert!(d.is_err());
    assert_matches!(d.unwrap_err(), bs63::decode::Error::InvalidVersion { .. });
}

#[test]
fn append() {
    let mut buf = b"hello world".to_vec();
    bs63::decode("X").into(&mut buf).unwrap();
    assert_eq!(b"hello world!", buf.as_slice());
}

#[test]
fn no_append() {
    let mut buf = b"hello world".to_owned();
    bs63::decode("X").into(buf.as_mut()).unwrap();
    assert_eq!(b"!ello world", buf.as_ref());
}
