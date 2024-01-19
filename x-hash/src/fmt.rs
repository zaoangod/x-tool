use core::fmt;
use core::str::from_utf8_unchecked;

const CHAR_TABLE: &[u8; 16] = b"0123456789abcdef";

/// 十六进制格式摘要包装器
pub struct DigestFmt<T>(pub T);

impl<T: AsRef<[u8]>> fmt::Display for DigestFmt<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf: [u8; 2];

        for byt in self.0.as_ref() {
            buf = [
                CHAR_TABLE[(byt.wrapping_shr(4) & 0xf) as usize],
                CHAR_TABLE[(byt & 0xf) as usize]
            ];
            fmt.write_str(unsafe {
                from_utf8_unchecked(&buf)
            })?
        }
        Ok(())
    }
}
