macro_rules! impl_rc5 {
    ($struct_name:ident, $word_type:ty, $num_rounds:literal, $key_size:literal, $P:literal, $Q:literal) => {
        pub struct $struct_name {
            /// The expanded secret key table.
            S: Vec<$word_type>,

            /// The word size in bits.
            W: $word_type,

            /// The number of rounds
            R: usize,

            /// The key length
            B: usize,

            /// Magic Constants determined by the size of W.
            P: $word_type,
            Q: $word_type,

            /// Number of bytes in each word.
            U: usize,
        }

        impl $struct_name {
            /// Construct an Rc5 instance from a secret key.
            pub fn new(key: &[u8]) -> Result<Self> {
                if key.len() != $key_size {
                    return Err(Error::InvalidKeyLen);
                }

                let W = (size_of::<$word_type>() * 8) as $word_type;
                let mut rc5 = Self {
                    S: vec![],
                    W,
                    R: $num_rounds,
                    B: $key_size,
                    P: $P,
                    Q: $Q,
                    U: (W / 8) as usize,
                };
                rc5.expand_key(key);

                Ok(rc5)
            }

            fn expand_key(&mut self, key: &[u8]) {
                let c = max(self.B as usize, 1) / self.U;

                let mut L: Vec<$word_type> = vec![0; self.B - 1];
                for (i, b) in key.iter().enumerate().rev() {
                    L[i / self.U] =
                        (L[i / self.U].checked_shl(8).unwrap_or(0)).wrapping_add(*b as $word_type);
                }

                let t = ((self.R + 1) * 2) as usize;
                let mut S = vec![0; t];

                S[0] = self.P;
                for i in 1..t {
                    S[i] = S[i - 1].wrapping_add(self.Q);
                }

                // Mix the secret key.
                let mut i = 0;
                let mut j = 0;

                let mut A: $word_type = 0;
                let mut B: $word_type = 0;

                for _ in 0..max(t, c) * 3 {
                    S[i] = self.rotate_left(S[i].wrapping_add(A.wrapping_add(B)), 3);
                    A = S[i];

                    let a_b = A.wrapping_add(B);
                    L[j] = self.rotate_left(L[j].wrapping_add(a_b), a_b);
                    B = L[j];

                    i = (i + 1) % t;
                    j = (j + 1) % c;
                }

                self.S = S;
            }

            fn rotate_left(&self, x: $word_type, y: $word_type) -> $word_type {
                x.wrapping_shl((y & (self.W - 1)) as u32)
                    | x.wrapping_shr((self.W - (y & (self.W - 1))) as u32)
            }

            fn rotate_right(&self, x: $word_type, y: $word_type) -> $word_type {
                x.wrapping_shr((y & (self.W - 1)) as u32)
                    | x.wrapping_shl((self.W - (y & (self.W - 1))) as u32)
            }

            /// Encrypt a plaintext into ciphertext.
            pub fn encrypt(&self, plaintext: Vec<u8>) -> Result<Vec<u8>> {
                let words = self.le_bytes_to_words(&plaintext)?;
                let mut A = words[0].wrapping_add(self.S[0]);
                let mut B = words[1].wrapping_add(self.S[1]);

                for i in 1..=self.R {
                    A = self.rotate_left(A ^ B, B).wrapping_add(self.S[2 * i]);
                    B = self.rotate_left(B ^ A, A).wrapping_add(self.S[2 * i + 1]);
                }

                Ok(self.words_to_le_bytes(&[A, B]))
            }

            /// Decrypt a ciphertext into plaintext.
            pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
                let words = self.le_bytes_to_words(&ciphertext)?;
                let mut A = words[0];
                let mut B = words[1];

                for i in (1..=self.R).rev() {
                    B = self.rotate_right(B.wrapping_sub(self.S[2 * i + 1]), A) ^ A;
                    A = self.rotate_right(A.wrapping_sub(self.S[2 * i]), B) ^ B;
                }
                B = B.wrapping_sub(self.S[1]);
                A = A.wrapping_sub(self.S[0]);

                Ok(self.words_to_le_bytes(&[A, B]))
            }

            fn le_bytes_to_words(&self, block: &[u8]) -> Result<[$word_type; 2]> {
                // TODO: Comment out block size len to demo failing fuzz test.
                if block.len() < self.U {
                    return Err(Error::BufferOutOfBounds);
                }

                Ok([
                    <$word_type>::from_le_bytes(block[..self.U].try_into()?),
                    <$word_type>::from_le_bytes(block[self.U..].try_into()?),
                ])
            }

            fn words_to_le_bytes(&self, words: &[$word_type; 2]) -> Vec<u8> {
                let mut bytes: Vec<u8> = Vec::new();
                bytes.extend_from_slice(&<$word_type>::to_le_bytes(words[0]));
                bytes.extend_from_slice(&<$word_type>::to_le_bytes(words[1]));
                bytes
            }
        }
    };
}
