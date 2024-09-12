use core::mem::MaybeUninit;

#[cfg(not(target_os = "solana"))]
use blake3::Hasher;

pub const HASH_LENGTH: usize = 32;

#[cfg(target_os = "solana")]
extern "C" {
    fn sol_blake3(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

#[cfg_attr(target_os = "solana", inline(always))]
pub fn hash(data: &[u8]) -> [u8;HASH_LENGTH] {
    hashv(&[data])
}

#[inline(always)]
pub fn hash_ref<T: AsRef<[u8]>>(data: T) -> [u8;HASH_LENGTH] {
    hashv(&[data.as_ref()])
}

#[cfg(not(target_os = "solana"))]
pub fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, out.assume_init_mut());
        out.assume_init()
    }
}

#[cfg(target_os = "solana")]
#[inline(always)]
pub fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, out.as_mut_ptr());
        out.assume_init()
    }
}

#[cfg(not(target_os = "solana"))]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    let mut hasher = Hasher::new();
    for item in data {
        hasher.update(item);
    }
    hasher.finalize_xof().fill(out);
}

#[cfg(target_os = "solana")]
#[inline(always)]
pub fn hash_into(data: &[&[u8]], out: *mut [u8; 32]) {
    unsafe {
        sol_blake3(
            data as *const _ as *const u8,
            data.len() as u64,
            out as *mut u8,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_hash() {
        let h = hash_ref("test");
        let h2 = hashv(&[b"test".as_ref()]);
        assert_eq!(h, h2);
        assert_eq!(h2, [0x48, 0x78, 0xca, 0x04, 0x25, 0xc7, 0x39, 0xfa, 0x42, 0x7f, 0x7e, 0xda, 0x20, 0xfe, 0x84, 0x5f, 0x6b, 0x2e, 0x46, 0xba, 0x5f, 0xe2, 0xa1, 0x4d, 0xf5, 0xb1, 0xe3, 0x2f, 0x50, 0x60, 0x32, 0x15]);
    }
}