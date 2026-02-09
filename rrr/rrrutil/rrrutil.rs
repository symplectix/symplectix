//! Provides helper (or main) tools for rrr implementations.

/// Encode using a static table.
#[macro_export]
macro_rules! encode {
    ($data:expr) => {{
        let data = $data & ((1 << SIZE) - 1);

        let class = data.count_ones();
        let offset = {
            let mut c = class as usize;
            let mut o = 0;
            let mut n = SIZE - 1;

            while 0 < c && c <= n {
                if data & (1 << n) != 0 {
                    o += COMB[n][c];
                    c -= 1;
                }
                n -= 1;
            }
            o
        };
        (class, offset)
    }};
}

/// Decode using a static table.
#[macro_export]
macro_rules! decode {
    ($class:expr, $offset:expr) => {{
        let mut data = 0;
        let mut c = $class as usize;
        let mut o = $offset;
        let mut i = 1usize;

        while 0 < c {
            let n = SIZE - i;
            if o >= COMB[n][c] {
                data |= 1 << n;
                o -= COMB[n][c];
                c -= 1;
            }
            i += 1;
        }
        data
    }};
}
