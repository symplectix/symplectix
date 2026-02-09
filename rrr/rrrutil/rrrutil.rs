//! Provides helper (or main) tools for rrr implementations.

/// Encode using static TABLE.
#[macro_export]
macro_rules! encode {
    ($data:expr) => {{
        let data = $data & ((1 << SIZE) - 1);

        let class = data.count_ones();
        let offset = {
            let mut c = class as usize;
            let mut o = 0;
            let mut j = 1;

            while 0 < c && c <= SIZE - j {
                if data & (1 << (SIZE - j)) != 0 {
                    o += TABLE[SIZE - j][c];
                    c -= 1;
                }
                j += 1;
            }
            o
        };
        (class, offset)
    }};
}

/// Decode using static TABLE.
#[macro_export]
macro_rules! decode {
    ($class:expr, $offset:expr) => {{
        let mut data = 0;
        let mut c = $class as usize;
        let mut o = $offset;
        let mut j = 1usize;

        while c > 0 {
            if o >= TABLE[SIZE - j][c] {
                data |= 1 << (SIZE - j);
                o -= TABLE[SIZE - j][c];
                c -= 1;
            }
            j += 1;
        }
        data
    }};
}
