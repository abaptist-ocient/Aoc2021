fn main() {
    let bits = include_str!("../input/16.txt")
        .trim()
        .chars()
        .flat_map(|c| {
            let n = c.to_digit(16).unwrap();
            (0..4).map(move |i| (1 & n >> (3 - i)) != 0)
        });

    let packet = parse_packet(&mut bits.collect::<Vec<_>>().into_iter());
    println!("part 1 {}", packet.version());
    println!("part 2 {}", packet.execute());
}

struct Packet {
    version: usize,
    children: Vec<Packet>,
    op: usize,
    literal: Option<usize>,
}

impl Packet {
    fn version(&self) -> usize {
        self.version
            + self
                .children
                .iter()
                .map(|child| child.version())
                .sum::<usize>()
    }

    fn execute(&self) -> usize {
        let ele: Vec<usize> = self.children.iter().map(|child| child.execute()).collect();
        match self.op {
            0 => ele.iter().sum(),
            1 => ele.iter().product(),
            2 => ele.into_iter().fold(usize::MAX, usize::min),
            3 => ele.into_iter().fold(usize::MIN, usize::max),
            4 => self.literal.unwrap(),
            5 => get_val(ele[0] < ele[1]),
            6 => get_val(ele[0] > ele[1]),
            7 => get_val(ele[0] == ele[1]),
            _ => unreachable!("{}", self.op),
        }
    }
}

fn get_val(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

fn parse_packet<I>(bits: &mut I) -> Packet
where
    I: ExactSizeIterator<Item = bool>,
{
    let version = to_usize(bits.take(3).collect());
    let op = to_usize(bits.take(3).collect());
    let literal;
    let children;
    if op == 4 {
        // handle literal special
        literal = Some(parse_num(bits));
        children = Vec::new();
    } else {
        literal = None;
        children = if bits.next().unwrap() {
            let num_ops = to_usize(bits.take(11).collect());
            (0..num_ops).map(|_| parse_packet(bits)).collect()
        } else {
            let remainder = bits.len() - to_usize(bits.take(15).collect());
            (0..)
                .map_while(|_| {
                    if remainder < bits.len() {
                        Some(parse_packet(bits))
                    } else {
                        None
                    }
                })
                .collect()
        }
    }
    Packet {
        version,
        children,
        op,
        literal,
    }
}

fn parse_num<I>(bits: &mut I) -> usize
where
    I: ExactSizeIterator<Item = bool>,
{
    let mut full: Vec<bool> = Vec::new();
    let mut done = false;
    while !done {
        done = !bits.next().unwrap();
        full.extend(bits.take(4));
    }
    to_usize(full)
}

fn to_usize(full: Vec<bool>) -> usize {
    full.into_iter()
        .fold(0, |init, next| (init << 1) | if next { 1 } else { 0 })
}
