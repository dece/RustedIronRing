use encoding_rs::SHIFT_JIS;
use nom::IResult;
use nom::bytes::complete::take_while;

pub fn take_cstring(i: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c != b'\0')(i)
}

pub fn sjis_to_string(i: &[u8]) -> Option<String> {
    let (cow, _, has_errors) = SHIFT_JIS.decode(i);
    if has_errors {
        return None
    }
    Some(cow.to_string())
}
