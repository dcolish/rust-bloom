// TODO:dc: test against reference implementation.
// TODO:dc: comments and usage


use std::option::{None};
use std::to_bytes::{ToBytes};
use std::iterator::{Counter};
use std::uint;


macro_rules! murmur32(
    ($inp:ident) => ( 
        murmur32(&($inp), &None, &None)
    );

    ($inp:ident, $length:ident) => (
        murmur32(&($inp), ($length), &None)
    );
)


fn main() {
    let s = "Hello";
    let m = murmur32!(s);

    let mut f:u32 = 111;
    f += (108 & 0xff) << 8;

    println(f.to_str());
    println(m.to_str());
}


fn chunk(data: &[u8], offset:uint, size: uint) -> u32 {
    Counter::new(0u, 1u)
        .take_while(|&a| a < size)
        .fold(0, |c:u32, i:uint| {
            let m = c + ((data[offset + i] & 0xff) as u32 << (i * 8));
            debug!("i=%?, c=%?", i, m);
            m
        })
}


#[test]
fn testchunk() {

    let data:~[u8] = "Hello".to_bytes(false);

    let i = 0;

    let k: u32 = chunk(data, i, 4);
    let mut r: u32 = 0;

    r += ((data[i + 0] & 0xff) as u32 << 0);
    debug!("r=%s", r.to_str());
    r += ((data[i + 1] & 0xff) as u32 << 8);
    debug!("r=%s", r.to_str());
    r += ((data[i + 2] & 0xff) as u32 << 16);
    debug!("r=%s", r.to_str());
    r += ((data[i + 3] & 0xff) as u32 << 24);
    debug!("r=%s", r.to_str());

    debug!("k=%?, r=%?", k, r);

    assert!( k == r);

}


pub fn murmur32 <T : ToBytes>(input: &T, l: &Option<uint>, s: &Option<uint>) -> u32 {
    
    let data: ~[u8] = input.to_bytes(false);
    let length: uint = l.get_or_default(data.len());
    let seed: uint =  s.get_or_default(0x9747b28c);

    let mut h: u32 = (seed ^ length) as u32;

    let c1 = 0xcc9e2d51;
    let c2 = 0x1b873593;
    let r1 = 15;
    let r2 = 13;
    let m = 5;
    let n = 0xe6546b64;

    let length4: uint = length/4;

    for uint::range(0, length4) |i| {
        let i4 : uint = i*4;

        // Build a 4 byte chunk
        let mut k: u32 = chunk(data, i4, 4);

        k *= c1;
        k = (k << r1) | (k >> (32 - r1));
        k *= c2;

        h ^= k;
        h = (h << r2) | (h >> (32 - r2));
        h = (h * m) + n
            
    }

    let mut remaining: u32 = 0;
    let mut rem = length % 4;
    
    while rem != 0  {
        remaining += match rem {
            3 => { ((data[length - 3] & 0xff) as u32 << 16) },
            2 => { ((data[length - 2] & 0xff) as u32 << 8) },
            1 => { (data[length - 1] & 0xff) as u32 },
            _ => { 0 }
        }; 
        rem -= 1;
    };

    // There were remaining bytes
    if remaining > 0 {
        remaining *= c1;
        remaining = (remaining << r1) | (remaining >> (32 - r1));
        remaining *= c2;
        h ^= remaining;
    };

    h ^= (h >> 16);
    h *= 0x85ebca6b;
    h ^= (h >> 13);
    h *= 0xc2b2ae35;
    h ^ (h >> 16)
}
