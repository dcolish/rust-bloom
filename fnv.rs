// Port of FNV 32 to Rust
// ref: http://isthe.com/chongo/src/fnv/hash_32.c
use std::to_bytes::{ToBytes};

fn main() {
    // let s = "a";
    let m = hash32(&~"a");
    println(m.to_str());
}

pub fn hash32<T: ToBytes>(input: &T) -> u32 {
    let data: ~[u8] = input.to_bytes(false);
    data.iter().fold(0u32, |a, b| {
        (a + (a<<1) + (a<<4) + (a<<7) + (a<<8) + (a<<24)) ^ (*b as u32)
    })            
}

#[cfg(test)]
mod tests {

    use hash32; 

    #[test]
    fn testHash32() {
        let comp = ~[
              ~"a"
            , ~"b"
            , ~"c"
            , ~"d"
            , ~"e"
            , ~"f"
            , ~"fo"
            , ~"foo"
            , ~"foob"
            , ~"fooba"
            , ~"foobar"
        ];

        let spec: ~[u32] = ~[
            0x00000061
          , 0x00000062
          , 0x00000063
          , 0x00000064
          , 0x00000065
          , 0x00000066
          , 0x6600a0fd
          , 0x8ffd6e28
          , 0xd3f4689a
          , 0x43c0aa0f
        ];

        let res = comp.iter().zip(spec.iter())
            .transform(|(&x, &y)| {
                debug!("x=%s, y=%?, h=%b", x, y, hash32(&x) == y);
                hash32(&x) == y
            })
            .all(|x| x);

        assert!(res == true);
    }
}

