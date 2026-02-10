//! Provides helper (or main) tools for rrr implementations.
//! Static values should be defined by rrrbuild.

/// Encode using a static table.
#[macro_export]
macro_rules! encode {
    ($data:expr) => {{
        let data = $data & ((1 << SIZE) - 1);

        let class = data.count_ones();
        let offset = {
            let mut c = class as usize;
            let mut o = 0;
            let mut b = SIZE;

            while 0 < c && c <= b {
                b -= 1;
                if data & (1 << b) != 0 {
                    o += COMB[b][c];
                    c -= 1;
                }
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
        let mut b = SIZE;

        while 0 < c && 0 < b {
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
