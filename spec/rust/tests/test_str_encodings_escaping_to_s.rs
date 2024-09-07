use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::str_encodings_escaping_to_s::*;

#[test]
fn test_str_encodings_escaping_to_s() -> KResult<()> {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let r: OptRc<StrEncodingsEscapingToS> = StrEncodingsEscapingToS::read_into(&_io, None, None)?;

    let err = r.str1().expect_err(r#"expected Err representing UnknownEncodingError('ASCII\\x'), but got Ok"#);
    assert_eq!(err, KError::UnknownEncoding { name: "ASCII\\\\x".to_string() });
    let err = r.str2().expect_err(r#"expected Err representing UnknownEncodingError("UTF-8\\'x"), but got Ok"#);
    assert_eq!(err, KError::UnknownEncoding { name: "UTF-8\\'x".to_string() });
    let err = r.str3().expect_err(r#"expected Err representing UnknownEncodingError('SJIS\"x'), but got Ok"#);
    assert_eq!(err, KError::UnknownEncoding { name: "SJIS\\\"x".to_string() });
    let err = r.str4().expect_err(r#"expected Err representing UnknownEncodingError('IBM437\nx'), but got Ok"#);
    assert_eq!(err, KError::UnknownEncoding { name: "IBM437\\nx".to_string() });
    Ok(())
}
