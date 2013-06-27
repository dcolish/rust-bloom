use std::option::{None};
use std::to_bytes::{ToBytes};
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
    println(m.to_str());
}


fn murmur32 <T : ToBytes>(input: &T, l: &Option<uint>, s: &Option<uint>) -> u32 {
    
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
        let mut k: u32 = (data[i4 + 0] & 0xff) as u32;
        k += ((data[i4 + 1] & 0xff) << 8) as u32;
        k += ((data[i4 + 2] & 0xff) << 16) as u32;
        k += ((data[i4 + 3] & 0xff) << 24) as u32;
            
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
            3 => { ((data[length - 3] & 0xff) << 16) as u32 },
            2 => { ((data[length - 2] & 0xff) << 8) as u32 },
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

