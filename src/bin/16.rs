use bitvec::prelude::*;

fn main() {
    let line = include_str!("../input/16.txt").lines().next().unwrap();
    let hex = hex::decode(line).unwrap();
    let bits = BitVec::<Msb0, u8>::from_slice(&hex).unwrap();
    let mut m = Machine { ver_count: 0 };
    let (val, _) = m.parse_type(&bits);
    println!("part 1 {}, part 2 {}", m.ver_count, val);
}

struct Machine {
    ver_count: usize,
}

impl Machine {
    fn parse_type(&mut self, mut bits: &BitSlice<Msb0, u8>) -> (usize, usize) {
        self.ver_count += to_usize(&bits[..3]);
        let op = to_usize(&bits[3..6]);
        bits = &bits[6..];
        let (val, consumed) = if let 4 = op {
            parse_num(bits)
        } else {
            let (values, consumed) = if bits[0] {
                self.count_based_op(&bits[1..])
            } else {
                self.size_based_op(&bits[1..])
            };
            (apply(op, values), consumed + 1)
        };
        (val, consumed + 6)
    }

    fn count_based_op(&mut self, mut bits: &BitSlice<Msb0, u8>) -> (Vec<usize>, usize) {
        let mut values = Vec::new();
        let mut subops = to_usize(&bits[..11]);
        bits = &bits[11..];
        let mut consumed = 0;
        while subops > 0 {
            let (val, used_bits) = self.parse_type(bits);
            bits = &bits[used_bits..];
            consumed += used_bits;
            subops -= 1;
            values.push(val);
        }
        (values, 11 + consumed)
    }

    fn size_based_op(&mut self, mut bits: &BitSlice<Msb0, u8>) -> (Vec<usize>, usize) {
        let mut values = Vec::new();
        let sub_op_bytes = to_usize(&bits[..15]);
        bits = &bits[15..];
        let mut to_consume = sub_op_bytes;
        while to_consume > 0 {
            let (val, used_bits) = self.parse_type(bits);
            bits = &bits[used_bits..];
            to_consume -= used_bits;
            values.push(val);
        }
        (values, 15 + sub_op_bytes)
    }
}

fn parse_num(mut bits: &BitSlice<Msb0, u8>) -> (usize, usize) {
    let mut full: BitVec<Msb0, u8> = BitVec::new();
    let mut num_consumed = 0;
    loop {
        num_consumed += 5;
        let num = &bits[1..5];
        let new_len = full.len() + 4;
        full.resize(new_len, false);
        full[new_len - 4..new_len].clone_from_bitslice(num);
        if !bits[0] {
            break;
        };
        bits = &bits[5..];
    }
    (to_usize(&full), num_consumed)
}

fn apply(functor: usize, values: Vec<usize>) -> usize {
    if functor < 4 {
        type ReduceFn = Box<dyn Fn(usize, &usize) -> usize>;
        let (init, reduce): (usize, ReduceFn) = match functor {
            0 => (0, Box::new(|init, val| init + val)),
            1 => (1, Box::new(|init, val| init * val)),
            2 => (usize::MAX, Box::new(|init, val| usize::min(init, *val))),
            3 => (usize::MIN, Box::new(|init, val| usize::max(init, *val))),
            _ => unreachable!(),
        };
        values.iter().fold(init, reduce.as_ref())
    } else {
        type CompareFn = Box<dyn Fn(usize, usize) -> bool>;
        let compare: CompareFn = match functor {
            5 => Box::new(|v1, v2| v1 > v2),
            6 => Box::new(|v1, v2| v1 < v2),
            7 => Box::new(|v1, v2| v1 == v2),
            _ => unreachable!(),
        };
        if compare(values[0], values[1]) {
            1
        } else {
            0
        }
    }
}

fn to_usize(full: &BitSlice<Msb0, u8>) -> usize {
    let mut result: usize = 0;
    for b in full {
        result <<= 1;
        result |= if *b { 1 } else { 0 };
    }
    result
}
