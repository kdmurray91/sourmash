extern crate sourmash;

use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use sourmash::{_hash_murmur, KmerMinHash};

#[no_mangle]
pub extern "C" fn hash_murmur(kmer: *const c_char, seed: u64) -> u64 {
    let c_str = unsafe {
        assert!(!kmer.is_null());

        CStr::from_ptr(kmer)
    };

    _hash_murmur(c_str.to_bytes(), seed)
}

#[no_mangle]
pub unsafe extern "C" fn kmerminhash_new(n: u32, k: u32, prot: bool,
                                         seed: u64, mx: u64, track_abundance: bool)
                                         -> *mut KmerMinHash {
    mem::transmute(Box::new(KmerMinHash::new(n, k, prot, seed, mx, track_abundance)))
}

#[no_mangle]
pub extern "C" fn kmerminhash_free(ptr: *mut KmerMinHash) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern "C" fn kmerminhash_add_sequence(ptr: *mut KmerMinHash, sequence: *const c_char, force: bool) {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let c_str = unsafe {
        assert!(!sequence.is_null());

        CStr::from_ptr(sequence)
    };

    mh.add_sequence(c_str.to_bytes(), force);
}

#[no_mangle]
pub extern "C" fn kmerminhash_add_hash(ptr: *mut KmerMinHash, h: u64) {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    mh.add_hash(h);
}

#[no_mangle]
pub extern "C" fn kmerminhash_add_word(ptr: *mut KmerMinHash, word: *const c_char) {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let c_str = unsafe {
        assert!(!word.is_null());

        CStr::from_ptr(word)
    };

    mh.add_word(c_str.to_bytes());
}

#[no_mangle]
pub extern "C" fn kmerminhash_get_min_idx(ptr: *mut KmerMinHash, idx: u64) -> u64 {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.mins[idx as usize]
}

#[no_mangle]
pub extern "C" fn kmerminhash_get_mins_size(ptr: *mut KmerMinHash) -> usize {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.mins.len()
}

#[no_mangle]
pub extern "C" fn kmerminhash_is_protein(ptr: *mut KmerMinHash) -> bool {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.is_protein
}

#[no_mangle]
pub extern "C" fn kmerminhash_seed(ptr: *mut KmerMinHash) -> u64 {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.seed
}

#[no_mangle]
pub extern "C" fn kmerminhash_num(ptr: *mut KmerMinHash) -> u32 {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.num
}

#[no_mangle]
pub extern "C" fn kmerminhash_ksize(ptr: *mut KmerMinHash) -> u32 {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.ksize
}

#[no_mangle]
pub extern "C" fn kmerminhash_max_hash(ptr: *mut KmerMinHash) -> u64 {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    mh.max_hash
}

#[no_mangle]
pub extern "C" fn kmerminhash_merge(ptr: *mut KmerMinHash, other: *const KmerMinHash) {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let other_mh = unsafe {
       assert!(!other.is_null());
       &*other
    };

    let merged = mh.merge(other_mh);
    mh.mins = merged
}

#[no_mangle]
pub extern "C" fn kmerminhash_count_common(ptr: *mut KmerMinHash, other: *const KmerMinHash) -> u64 {
    let mh = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let other_mh = unsafe {
       assert!(!other.is_null());
       &*other
    };

    mh.count_common(other_mh)
}
