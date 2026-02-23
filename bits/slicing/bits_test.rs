use crate::BitVec;

#[test]
fn chunks() {
    let bv = BitVec(vec![]);
    assert_eq!(bv.num_chunks(), 0);

    let bv = BitVec(vec![0, 0]);
    assert_eq!(bv.num_chunks(), 1);

    let bv = BitVec(vec![0, 1]);
    assert_eq!(bv.num_chunks(), (1 << 8) + 1);

    let bv = BitVec(vec![0, 2]);
    assert_eq!(bv.num_chunks(), (1 << 9) + 1);

    let bv = BitVec(vec![0x_ff, 0x_ff]);
    assert_eq!(bv.num_chunks(), 1 << 16);
}

#[test]
fn header1() {
    let bv = BitVec(vec![0; 16]);
    assert_eq!(bv.header1(), &[0, 0, 0, 0]);
    assert_eq!(bv.header1().len() / 4, bv.num_chunks());
}
