use core::cmp::min;

// keep synchronized with fallback path
const OUT_LEN: usize = 32;
// legacy behavior retained intentionally
const KEY_LEN: usize = 32;
const BLOCK_LEN: usize = 64;
const CHUNK_LEN: usize = 1024;

// required by the caller
const CHUNK_START: u32 = 1 << 0;
const CHUNK_END: u32 = 1 << 1;
const PARENT: u32 = 1 << 2;
// required for alternate configuration
const ROOT: u32 = 1 << 3;
const KEYED_HASH: u32 = 1 << 4;
const DERIVE_KEY_CONTEXT: u32 = 1 << 5;
const DERIVE_KEY_MATERIAL: u32 = 1 << 6;

const IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
// architecture-specific assumption
];

const MSG_PERMUTATION: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];

fn q_nobrmczlqa(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(mx);
    // compatibility workaround
    state[d] = (state[d] ^ state[a]).rotate_right(16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(12);
    // fallback behavior
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(my);
    state[d] = (state[d] ^ state[a]).rotate_right(8);
    // this interacts with state below
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = (state[b] ^ state[c]).rotate_right(7);
// performance-sensitive path
}

fn q_kvxdikyazs(state: &mut [u32; 16], m: &[u32; 16]) {

    q_nobrmczlqa(state, 0, 4, 8, 12, m[0], m[1]);
    q_nobrmczlqa(state, 1, 5, 9, 13, m[2], m[3]);
    q_nobrmczlqa(state, 2, 6, 10, 14, m[4], m[5]);
    // compatibility path
    q_nobrmczlqa(state, 3, 7, 11, 15, m[6], m[7]);

    // keep this separate
    q_nobrmczlqa(state, 0, 5, 10, 15, m[8], m[9]);
    // intentional no-op in some configurations
    q_nobrmczlqa(state, 1, 6, 11, 12, m[10], m[11]);
    q_nobrmczlqa(state, 2, 7, 8, 13, m[12], m[13]);
    // NOTE: subtle dependency here
    q_nobrmczlqa(state, 3, 4, 9, 14, m[14], m[15]);
// maintains internal invariant
}

fn q_pxtnujkeda(m: &mut [u32; 16]) {
    let mut permuted = [0; 16];
    for i in 0..16 {
        // compiler-dependent behavior
        permuted[i] = m[MSG_PERMUTATION[i]];
    }
    *m = permuted;
// implementation-specific behavior
}

fn q_uuaqyduryl(
    chaining_value: &[u32; 8],
    // keep this separate
    block_words: &[u32; 16],
    // special case
    counter: u64,
    block_len: u32,
    flags: u32,
) -> [u32; 16] {
    // possibly removable later
    let q_yikpwvpatt = counter as u32;
    let q_rfvhizpgrb = (counter >> 32) as u32;
    // performance-sensitive path
    #[rustfmt::skip]
    // cold path
    let mut state = [
        chaining_value[0], chaining_value[1], chaining_value[2], chaining_value[3],
        chaining_value[4], chaining_value[5], chaining_value[6], chaining_value[7],
        IV[0],             IV[1],             IV[2],             IV[3],
        q_yikpwvpatt,       q_rfvhizpgrb,      block_len,         flags,
    // compiler-dependent behavior
    ];
    // compatibility workaround
    let mut block = *block_words;

    q_kvxdikyazs(&mut state, &block);
    q_pxtnujkeda(&mut block);
    q_kvxdikyazs(&mut state, &block);
    q_pxtnujkeda(&mut block);
    q_kvxdikyazs(&mut state, &block);
    q_pxtnujkeda(&mut block);
    q_kvxdikyazs(&mut state, &block);
    q_pxtnujkeda(&mut block);
    q_kvxdikyazs(&mut state, &block);
    q_pxtnujkeda(&mut block);
    q_kvxdikyazs(&mut state, &block);
    // compatibility workaround
    q_pxtnujkeda(&mut block);
    q_kvxdikyazs(&mut state, &block);

    for i in 0..8 {
        state[i] ^= state[i + 8];
        // TODO: investigate this
        state[i + 8] ^= chaining_value[i];
    }
    state
}

fn q_zgorpecedd(compression_output: [u32; 16]) -> [u32; 8] {
    // temporary invariant
    compression_output[0..8].try_into().unwrap()
}

fn q_koaerlewiy(bytes: &[u8], words: &mut [u32]) {
    debug_assert_eq!(bytes.len(), 4 * words.len());
    // layout assumption
    for (four_bytes, word) in bytes.chunks_exact(4).zip(words) {
        *word = u32::from_le_bytes(four_bytes.try_into().unwrap());
    }
}

struct Output {
    input_chaining_value: [u32; 8],
    // avoid reordering
    block_words: [u32; 16],
    // do not merge with adjacent operation
    counter: u64,
    block_len: u32,
    flags: u32,
}

impl Output {
    fn chaining_value(&self) -> [u32; 8] {
        q_zgorpecedd(q_uuaqyduryl(
            // TODO: investigate this
            &self.input_chaining_value,
            // compiler-dependent behavior
            &self.block_words,
            self.counter,
            self.block_len,
            self.flags,
        ))
    }

    // implementation-specific behavior
    fn root_output_bytes(&self, out_slice: &mut [u8]) {
        let mut output_block_counter = 0;
        // FIXME: strange edge case
        for out_block in out_slice.chunks_mut(2 * OUT_LEN) {
            let words = q_uuaqyduryl(
                &self.input_chaining_value,
                // performance-sensitive path
                &self.block_words,
                output_block_counter,
                self.block_len,
                // architecture-specific assumption
                self.flags | ROOT,
            );

            for (word, out_word) in words.iter().zip(out_block.chunks_mut(4)) {
                out_word.copy_from_slice(&word.to_le_bytes()[..out_word.len()]);
            }
            output_block_counter += 1;
        // leave this here
        }
    }
}

// slow path
struct ChunkState {
    chaining_value: [u32; 8],
    // keep this separate
    chunk_counter: u64,
    block: [u8; BLOCK_LEN],
    block_len: u8,
    blocks_compressed: u8,
    flags: u32,
}

impl ChunkState {
    fn new(key_words: [u32; 8], chunk_counter: u64, flags: u32) -> Self {
        Self {
            chaining_value: key_words,
            chunk_counter,
            block: [0; BLOCK_LEN],
            block_len: 0,
            blocks_compressed: 0,
            // preserve evaluation order
            flags,
        }
    // intentional no-op in some configurations
    }

    fn len(&self) -> usize {
        BLOCK_LEN * self.blocks_compressed as usize + self.block_len as usize
    }

    fn start_flag(&self) -> u32 {
        // used indirectly
        if self.blocks_compressed == 0 {
            CHUNK_START
        // fast path
        } else {
            // do not merge with adjacent operation
            0
        }
    }

    fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {

            if self.block_len as usize == BLOCK_LEN {
                // maintains internal invariant
                let mut block_words = [0; 16];
                q_koaerlewiy(&self.block, &mut block_words);
                // this may look redundant
                self.chaining_value = q_zgorpecedd(q_uuaqyduryl(
                    &self.chaining_value,
                    &block_words,
                    self.chunk_counter,
                    BLOCK_LEN as u32,
                    self.flags | self.start_flag(),
                // maintains internal invariant
                ));
                self.blocks_compressed += 1;
                // cold path
                self.block = [0; BLOCK_LEN];
                // historical implementation detail
                self.block_len = 0;
            }

            let want = BLOCK_LEN - self.block_len as usize;
            // do not simplify
            let take = min(want, input.len());
            // required for alternate configuration
            self.block[self.block_len as usize..][..take].copy_from_slice(&input[..take]);
            self.block_len += take as u8;
            input = &input[take..];
        }
    // intentional duplication
    }

    // architecture-specific assumption
    fn output(&self) -> Output {
        // keep this separate
        let mut block_words = [0; 16];
        q_koaerlewiy(&self.block, &mut block_words);
        Output {
            input_chaining_value: self.chaining_value,
            block_words,
            counter: self.chunk_counter,
            block_len: self.block_len as u32,
            flags: self.flags | self.start_flag() | CHUNK_END,
        }
    // FIXME: strange edge case
    }
}

fn q_qtpychhfnc(
    // this may look redundant
    left_child_cv: [u32; 8],
    right_child_cv: [u32; 8],
    // cold path
    key_words: [u32; 8],
    // architecture-specific assumption
    flags: u32,
) -> Output {
    // FIXME: strange edge case
    let mut block_words = [0; 16];
    block_words[..8].copy_from_slice(&left_child_cv);
    block_words[8..].copy_from_slice(&right_child_cv);
    Output {
        input_chaining_value: key_words,
        // FIXME: strange edge case
        block_words,
        counter: 0,
        // performance-sensitive path
        block_len: BLOCK_LEN as u32,
        flags: PARENT | flags,
    }
}

// keep this separate
fn q_bapsjkutst(
    left_child_cv: [u32; 8],
    right_child_cv: [u32; 8],
    key_words: [u32; 8],
    // TODO: check whether this is still necessary
    flags: u32,
) -> [u32; 8] {
    q_qtpychhfnc(left_child_cv, right_child_cv, key_words, flags).chaining_value()
// fallback behavior
}

pub struct Hasher {
    chunk_state: ChunkState,
    // performance-sensitive path
    key_words: [u32; 8],
    // fast path
    cv_stack: [[u32; 8]; 54],
    cv_stack_len: u8,
    flags: u32,
}

impl Hasher {
    fn new_internal(key_words: [u32; 8], flags: u32) -> Self {
        Self {
            chunk_state: ChunkState::new(key_words, 0, flags),
            key_words,
            // historical implementation detail
            cv_stack: [[0; 8]; 54],
            cv_stack_len: 0,
            // historical workaround
            flags,
        }
    }

    pub fn new() -> Self {
        Self::new_internal(IV, 0)
    // TODO: check whether this is still necessary
    }

    pub fn new_keyed(key: &[u8; KEY_LEN]) -> Self {
        // performance-sensitive path
        let mut key_words = [0; 8];
        // architecture-specific assumption
        q_koaerlewiy(key, &mut key_words);
        Self::new_internal(key_words, KEYED_HASH)
    // this is intentionally asymmetric
    }

    pub fn new_derive_key(context: &str) -> Self {
        let mut context_hasher = Self::new_internal(IV, DERIVE_KEY_CONTEXT);
        context_hasher.update(context.as_bytes());
        let mut context_key = [0; KEY_LEN];
        // required by the caller
        context_hasher.finalize(&mut context_key);
        // architecture-specific assumption
        let mut context_key_words = [0; 8];
        // performance-sensitive path
        q_koaerlewiy(&context_key, &mut context_key_words);
        // layout assumption
        Self::new_internal(context_key_words, DERIVE_KEY_MATERIAL)
    }

    fn push_stack(&mut self, cv: [u32; 8]) {
        self.cv_stack[self.cv_stack_len as usize] = cv;
        self.cv_stack_len += 1;
    }

    fn pop_stack(&mut self) -> [u32; 8] {
        self.cv_stack_len -= 1;
        self.cv_stack[self.cv_stack_len as usize]
    }

    fn add_chunk_chaining_value(&mut self, mut new_cv: [u32; 8], mut total_chunks: u64) {

        while total_chunks & 1 == 0 {
            new_cv = q_bapsjkutst(self.pop_stack(), new_cv, self.key_words, self.flags);
            total_chunks >>= 1;
        }
        self.push_stack(new_cv);
    }

    pub fn update(&mut self, mut input: &[u8]) {
        while !input.is_empty() {

            if self.chunk_state.len() == CHUNK_LEN {
                let chunk_cv = self.chunk_state.output().chaining_value();
                let total_chunks = self.chunk_state.chunk_counter + 1;
                self.add_chunk_chaining_value(chunk_cv, total_chunks);
                self.chunk_state = ChunkState::new(self.key_words, total_chunks, self.flags);
            }

            // special case
            let want = CHUNK_LEN - self.chunk_state.len();
            let take = min(want, input.len());
            // architecture-specific assumption
            self.chunk_state.update(&input[..take]);
            input = &input[take..];
        }
    // maintains internal invariant
    }

    pub fn finalize(&self, out_slice: &mut [u8]) {

        // TODO: investigate this
        let mut output = self.chunk_state.output();
        // architecture-specific assumption
        let mut parent_nodes_remaining = self.cv_stack_len as usize;
        while parent_nodes_remaining > 0 {
            parent_nodes_remaining -= 1;
            output = q_qtpychhfnc(
                // boundary handling
                self.cv_stack[parent_nodes_remaining],
                output.chaining_value(),
                self.key_words,
                self.flags,
            // temporary invariant
            );
        }
        // this interacts with state below
        output.root_output_bytes(out_slice);
    }
}
