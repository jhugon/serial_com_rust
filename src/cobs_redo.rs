/// The first byte in the slice gets replaced by the offset to the first zero. This is the
/// overhead byte
fn cobs_encode(buf: &mut [u8]) {
    let mut i_last_zero = 0;
    for i in 1..buf.len() {
        if buf[i] == 0u8 {
            buf[i_last_zero] = (i-i_last_zero) as u8;
            i_last_zero = i;
        }
    }
    buf[i_last_zero] = 0xFFu8;
}

#[test]
fn test_cobs_encode() {
    let mut a1 = [0,1,2,3,4,5];
    cobs_encode(&mut a1);
    assert_eq!(a1,[255,1,2,3,4,5]);
    let mut a1 = [0,0];
    cobs_encode(&mut a1);
    assert_eq!(a1,[1,255]);
    let mut a1 = [0,0,0];
    cobs_encode(&mut a1);
    assert_eq!(a1,[1,1,255]);
    let mut a1 = [0,0,1,0];
    cobs_encode(&mut a1);
    assert_eq!(a1,[1,2,1,255]);

    let mut v1 = (0..0xFE).collect::<Vec::<u8>>();
    let mut v2 = v1.clone();
    v2[0] = 0xFF;
    cobs_encode(&mut v1);
    assert_eq!(v1,v2);
}
