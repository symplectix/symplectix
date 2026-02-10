//! Provides helper (or main?) utilities for rrr implementations.

// NOTE: COMB[n][k] is the number of ways to choose k items
// from n items without repetition and without order.

/// Encode using a static table.
#[macro_export]
macro_rules! encode {
    ($data:expr) => {{
        let data = $data & ((1 << SIZE) - 1);

        let class = data.count_ones();
        let offset = {
            let mut b = SIZE;
            let mut c = class as usize;
            let mut o = 0;

            while b > 0 && c > 0 {
                b -= 1;
                if data & (1 << b) != 0 {
                    o += COMB[b][c];
                    c -= 1;
                }
            }
            o
        };
        (class as u8, offset)
    }};
}

/// Decode using a static table.
#[macro_export]
macro_rules! decode {
    ($class:expr, $offset:expr) => {{
        let mut data = 0;
        let mut b = SIZE;
        let mut c = $class as usize;
        let mut o = $offset;

        while b > 0 && c > 0 {
            b -= 1;
            if o >= COMB[b][c] {
                data |= 1 << b;
                o -= COMB[b][c];
                c -= 1;
            }
        }
        data
    }};
}
