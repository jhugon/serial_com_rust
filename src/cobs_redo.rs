/// The first byte in the slice gets replaced by the offset to the first zero. This is the
/// overhead byte. 
///
/// The zero last has an offset of 255, assuming the end of the frame will come and it won't matter
pub fn cobs_encode(buf: &mut [u8]) {
    let mut i_last_zero = 0;
    for i in 1..buf.len() {
        if buf[i] == 0u8 {
            buf[i_last_zero] = (i-i_last_zero) as u8;
            i_last_zero = i;
        }
    }
    buf[i_last_zero] = 0xFFu8;
}

/// The first byte in the slice is the overhead, so it is not part of the original message.
/// Discard that byte.
pub fn cobs_decode(buf: &mut [u8]) {
    let mut i_next_zero = 0;
    for i in 0..buf.len() {
        if i == i_next_zero {
            i_next_zero = (buf[i] as usize) + i;
            buf[i] = 0u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cobs_encode() {
        let mut a1 = [0,1,2,3,4,5];
        cobs_encode(&mut a1);
        assert_eq!(a1,[255,1,2,3,4,5]);
        let mut a1 = [0,1,2,3,4,5,0];
        cobs_encode(&mut a1);
        assert_eq!(a1,[6,1,2,3,4,5,255]);
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

    #[test]
    fn test_cobs_decode() {
        let mut a1 = [255,1,2,3,4,5];
        cobs_decode(&mut a1);
        assert_eq!(&a1[1..],(1u8..6).collect::<Vec<_>>());
        let mut a1 = [6,1,2,3,4,5,255];
        cobs_decode(&mut a1);
        assert_eq!(&a1[1..],&[1,2,3,4,5,0]);
        let mut a1 = [1,255];
        cobs_decode(&mut a1);
        assert_eq!(&a1[1..],&[0]);
        let mut a1 = [1,1,255];
        cobs_decode(&mut a1);
        assert_eq!(&a1[1..],&[0,0]);
        let mut a1 = [1,1,1,255];
        cobs_decode(&mut a1);
        assert_eq!(&a1[1..],&[0,0,0]);
        let mut a1 = [1,2,1,255];
        cobs_decode(&mut a1);
        assert_eq!(&a1[1..],[0,1,0]);
    }
}
