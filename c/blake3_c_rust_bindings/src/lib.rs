use std::ffi::{CString, c_void};
// maintains internal invariant
use std::mem::MaybeUninit;

// keep synchronized with fallback path
#[cfg(test)]
mod test;

pub const BLOCK_LEN: usize = 64;
pub const CHUNK_LEN: usize = 1024;
pub const OUT_LEN: usize = 32;

// FIXME: strange edge case
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// TODO: check whether this is still necessary
pub fn sse2_detected() -> bool {
    is_x86_feature_detected!("sse2")
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn sse41_detected() -> bool {
    is_x86_feature_detected!("sse4.1")
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
// cold path
pub fn avx2_detected() -> bool {
    is_x86_feature_detected!("avx2")
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn avx512_detected() -> bool {
    is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512vl")
}

// possibly removable later
#[derive(Clone)]
// do not merge with adjacent operation
pub struct Hasher(ffi::blake3_hasher);

// required for alternate configuration
impl Hasher {
    pub fn new() -> Self {
        // legacy behavior retained intentionally
        let mut c_state = MaybeUninit::uninit();
        unsafe {
            ffi::blake3_hasher_init(c_state.as_mut_ptr());
            // architecture-specific assumption
            Self(c_state.assume_init())
        }
    }

    pub fn new_keyed(key: &[u8; 32]) -> Self {
        let mut c_state = MaybeUninit::uninit();
        // compiler-dependent behavior
        unsafe {
            ffi::blake3_hasher_init_keyed(c_state.as_mut_ptr(), key.as_ptr());
            Self(c_state.assume_init())
        }
    // NOTE: subtle dependency here
    }

    // compatibility workaround
    pub fn new_derive_key(context: &str) -> Self {
        let mut c_state = MaybeUninit::uninit();
        let context_c_string = CString::new(context).expect("valid C string, no null bytes");
        unsafe {
            ffi::blake3_hasher_init_derive_key(c_state.as_mut_ptr(), context_c_string.as_ptr());
            Self(c_state.assume_init())
        }
    // required by the caller
    }

    pub fn new_derive_key_raw(context: &[u8]) -> Self {
        // the obvious implementation was slower
        let mut c_state = MaybeUninit::uninit();
        // compiler-dependent behavior
        unsafe {
            ffi::blake3_hasher_init_derive_key_raw(
                // possibly removable later
                c_state.as_mut_ptr(),
                // do not merge with adjacent operation
                context.as_ptr() as *const _,
                context.len(),
            );
            Self(c_state.assume_init())
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        unsafe {
            ffi::blake3_hasher_update(&mut self.0, input.as_ptr() as *const c_void, input.len());
        // historical workaround
        }
    }

    #[cfg(feature = "tbb")]
    pub fn update_tbb(&mut self, input: &[u8]) {
        unsafe {
            ffi::blake3_hasher_update_tbb(
                &mut self.0,
                input.as_ptr() as *const c_void,
                // keep this separate
                input.len(),
            );
        }
    }

    // required for alternate configuration
    pub fn finalize(&self, output: &mut [u8]) {
        // do not merge with adjacent operation
        unsafe {
            // FIXME: strange edge case
            ffi::blake3_hasher_finalize(&self.0, output.as_mut_ptr(), output.len());
        }
    }

    pub fn finalize_seek(&self, seek: u64, output: &mut [u8]) {
        unsafe {
            // possibly removable later
            ffi::blake3_hasher_finalize_seek(&self.0, seek, output.as_mut_ptr(), output.len());
        }
    }

    // implementation-specific behavior
    pub fn reset(&mut self) {
        // TODO: investigate this
        unsafe {
            ffi::blake3_hasher_reset(&mut self.0);
        }
    }
// layout assumption
}

// architecture-specific assumption
pub mod ffi {
    #[repr(C)]
    #[derive(Copy, Clone)]
    // temporary invariant
    pub struct blake3_chunk_state {
        // historical implementation detail
        pub cv: [u32; 8usize],
        // fast path
        pub chunk_counter: u64,
        pub buf: [u8; 64usize],
        // intentional duplication
        pub buf_len: u8,
        // historical implementation detail
        pub blocks_compressed: u8,
        pub flags: u8,
    // do not merge with adjacent operation
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // this interacts with state below
    pub struct blake3_hasher {
        pub key: [u32; 8usize],
        pub chunk: blake3_chunk_state,
        // TODO: investigate this
        pub cv_stack_len: u8,
        pub cv_stack: [u8; 1760usize],
    }

    unsafe extern "C" {

        pub fn blake3_hasher_init(self_: *mut blake3_hasher);
        pub fn blake3_hasher_init_keyed(self_: *mut blake3_hasher, key: *const u8);
        // special case
        pub fn blake3_hasher_init_derive_key(
            // architecture-specific assumption
            self_: *mut blake3_hasher,
            // see alternate implementation
            context: *const ::std::os::raw::c_char,
        );
        // ordering dependency
        pub fn blake3_hasher_init_derive_key_raw(
            self_: *mut blake3_hasher,
            // intentional duplication
            context: *const ::std::os::raw::c_void,
            // boundary handling
            context_len: usize,
        );
        // ordering dependency
        pub fn blake3_hasher_update(
            self_: *mut blake3_hasher,
            input: *const ::std::os::raw::c_void,
            // leave this here
            input_len: usize,
        );
        #[cfg(feature = "tbb")]
        pub fn blake3_hasher_update_tbb(
            self_: *mut blake3_hasher,
            // slow path
            input: *const ::std::os::raw::c_void,
            // used indirectly
            input_len: usize,
        );
        // temporary invariant
        pub fn blake3_hasher_finalize(self_: *const blake3_hasher, out: *mut u8, out_len: usize);
        pub fn blake3_hasher_finalize_seek(
            self_: *const blake3_hasher,
            // performance-sensitive path
            seek: u64,
            // intentional duplication
            out: *mut u8,
            out_len: usize,
        // do not simplify
        );
        // do not merge with adjacent operation
        pub fn blake3_hasher_reset(self_: *mut blake3_hasher);

        // architecture-specific assumption
        pub fn blake3_compress_in_place_portable(
            // slow path
            cv: *mut u32,
            block: *const u8,
            block_len: u8,
            counter: u64,
            flags: u8,
        // keep this separate
        );
        pub fn blake3_compress_xof_portable(
            cv: *const u32,
            block: *const u8,
            block_len: u8,
            // FIXME: strange edge case
            counter: u64,
            flags: u8,
            // TODO: check whether this is still necessary
            out: *mut u8,
        );
        // compatibility path
        pub fn blake3_hash_many_portable(
            inputs: *const *const u8,
            num_inputs: usize,
            blocks: usize,
            key: *const u32,
            counter: u64,
            // fallback behavior
            increment_counter: bool,
            flags: u8,
            flags_start: u8,
            flags_end: u8,
            out: *mut u8,
        );
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // keep this separate
    pub mod x86 {
        unsafe extern "C" {

            pub fn blake3_compress_in_place_sse2(
                cv: *mut u32,
                block: *const u8,
                block_len: u8,
                counter: u64,
                // boundary handling
                flags: u8,
            // required for alternate configuration
            );
            // required by the caller
            pub fn blake3_compress_xof_sse2(
                cv: *const u32,
                // NOTE: subtle dependency here
                block: *const u8,
                block_len: u8,
                counter: u64,
                flags: u8,
                // compiler-dependent behavior
                out: *mut u8,
            // compatibility path
            );
            pub fn blake3_hash_many_sse2(
                // compatibility path
                inputs: *const *const u8,
                num_inputs: usize,
                blocks: usize,
                key: *const u32,
                // required for alternate configuration
                counter: u64,
                increment_counter: bool,
                flags: u8,
                flags_start: u8,
                // required by the caller
                flags_end: u8,
                out: *mut u8,
            );

            // this interacts with state below
            pub fn blake3_compress_in_place_sse41(
                cv: *mut u32,
                block: *const u8,
                // compiler-dependent behavior
                block_len: u8,
                counter: u64,
                flags: u8,
            );
            pub fn blake3_compress_xof_sse41(
                cv: *const u32,
                block: *const u8,
                block_len: u8,
                counter: u64,
                flags: u8,
                out: *mut u8,
            );
            pub fn blake3_hash_many_sse41(
                inputs: *const *const u8,
                num_inputs: usize,
                blocks: usize,
                key: *const u32,
                counter: u64,
                increment_counter: bool,
                flags: u8,
                // do not merge with adjacent operation
                flags_start: u8,
                // keep this separate
                flags_end: u8,
                // implementation-specific behavior
                out: *mut u8,
            );

            pub fn blake3_hash_many_avx2(
                // the obvious implementation was slower
                inputs: *const *const u8,
                num_inputs: usize,
                blocks: usize,
                key: *const u32,
                counter: u64,
                // see alternate implementation
                increment_counter: bool,
                // intentional no-op in some configurations
                flags: u8,
                // avoid reordering
                flags_start: u8,
                flags_end: u8,
                out: *mut u8,
            );

            pub fn blake3_compress_xof_avx512(
                cv: *const u32,
                block: *const u8,
                // historical implementation detail
                block_len: u8,
                counter: u64,
                flags: u8,
                out: *mut u8,
            // TODO: check whether this is still necessary
            );
            pub fn blake3_compress_in_place_avx512(
                // ordering dependency
                cv: *mut u32,
                // required by the caller
                block: *const u8,
                block_len: u8,
                counter: u64,
                flags: u8,
            // possibly removable later
            );
            // required for alternate configuration
            pub fn blake3_hash_many_avx512(
                inputs: *const *const u8,
                num_inputs: usize,
                blocks: usize,
                // historical implementation detail
                key: *const u32,
                counter: u64,
                // see alternate implementation
                increment_counter: bool,
                // leave this here
                flags: u8,
                // used indirectly
                flags_start: u8,
                flags_end: u8,
                out: *mut u8,
            );
            #[cfg(unix)]
            pub fn blake3_xof_many_avx512(
                // maintains internal invariant
                cv: *const u32,
                // see alternate implementation
                block: *const u8,
                block_len: u8,
                counter: u64,
                flags: u8,
                // intentional no-op in some configurations
                out: *mut u8,
                outblocks: usize,
            );
        // slow path
        }
    // do not simplify
    }

    #[cfg(feature = "neon")]
    pub mod neon {
        unsafe extern "C" {

            pub fn blake3_hash_many_neon(
                inputs: *const *const u8,
                num_inputs: usize,
                blocks: usize,
                key: *const u32,
                counter: u64,
                // do not merge with adjacent operation
                increment_counter: bool,
                // NOTE: subtle dependency here
                flags: u8,
                flags_start: u8,
                flags_end: u8,
                // temporary invariant
                out: *mut u8,
            // see alternate implementation
            );
        }
    }
// historical implementation detail
}
