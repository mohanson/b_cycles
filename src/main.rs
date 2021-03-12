pub fn gorc32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x01) != 0 {
        x |= ((x & 0x5555_5555) << 0x01) | ((x & 0xAAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x |= ((x & 0x3333_3333) << 0x02) | ((x & 0xCCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x |= ((x & 0x0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x |= ((x & 0x00FF_00FF) << 0x08) | ((x & 0xFF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x |= ((x & 0x0000_FFFF) << 0x10) | ((x & 0xFFFF_0000) >> 0x10);
    }
    x
}

pub fn gorc64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x3f;
    if (shamt & 0x01) != 0 {
        x |= ((x & 0x5555_5555_5555_5555) << 0x01) | ((x & 0xAAAA_AAAA_AAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x |= ((x & 0x3333_3333_3333_3333) << 0x02) | ((x & 0xCCCC_CCCC_CCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x |= ((x & 0x0F0F_0F0F_0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0_F0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x |= ((x & 0x00FF_00FF_00FF_00FF) << 0x08) | ((x & 0xFF00_FF00_FF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x |= ((x & 0x0000_FFFF_0000_FFFF) << 0x10) | ((x & 0xFFFF_0000_FFFF_0000) >> 0x10);
    }
    if (shamt & 0x20) != 0 {
        x |= ((x & 0x0000_0000_FFFF_FFFF) << 0x20) | ((x & 0xFFFF_FFFF_0000_0000) >> 0x20);
    }
    x
}

pub fn grev32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x01) != 0 {
        x = ((x & 0x5555_5555) << 0x01) | ((x & 0xAAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x = ((x & 0x3333_3333) << 0x02) | ((x & 0xCCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x = ((x & 0x0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x = ((x & 0x00FF_00FF) << 0x08) | ((x & 0xFF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x = ((x & 0x0000_FFFF) << 0x10) | ((x & 0xFFFF_0000) >> 0x10);
    }
    x
}

pub fn grev64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x3f;
    if (shamt & 0x01) != 0 {
        x = ((x & 0x5555_5555_5555_5555) << 0x01) | ((x & 0xAAAA_AAAA_AAAA_AAAA) >> 0x01);
    }
    if (shamt & 0x02) != 0 {
        x = ((x & 0x3333_3333_3333_3333) << 0x02) | ((x & 0xCCCC_CCCC_CCCC_CCCC) >> 0x02);
    }
    if (shamt & 0x04) != 0 {
        x = ((x & 0x0F0F_0F0F_0F0F_0F0F) << 0x04) | ((x & 0xF0F0_F0F0_F0F0_F0F0) >> 0x04);
    }
    if (shamt & 0x08) != 0 {
        x = ((x & 0x00FF_00FF_00FF_00FF) << 0x08) | ((x & 0xFF00_FF00_FF00_FF00) >> 0x08);
    }
    if (shamt & 0x10) != 0 {
        x = ((x & 0x0000_FFFF_0000_FFFF) << 0x10) | ((x & 0xFFFF_0000_FFFF_0000) >> 0x10);
    }
    if (shamt & 0x20) != 0 {
        x = ((x & 0x0000_0000_FFFF_FFFF) << 0x20) | ((x & 0xFFFF_FFFF_0000_0000) >> 0x20);
    }
    x
}

pub fn bext32(rs1: u32, rs2: u32) -> u32 {
    let mut r: u32 = 0;
    let mut j = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> i) & 1) != 0 {
                r |= 1 << j;
            }
            j += 1;
        }
    }
    r
}

pub fn bext64(rs1: u64, rs2: u64) -> u64 {
    let mut r: u64 = 0;
    let mut j = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> i) & 1) != 0 {
                r |= 1 << j;
            }
            j += 1;
        }
    }
    r
}

pub fn bdep32(rs1: u32, rs2: u32) -> u32 {
    let mut r: u32 = 0;
    let mut j = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> j) & 1) != 0 {
                r |= 1 << i;
            }
            j += 1;
        }
    }
    r
}

pub fn bdep64(rs1: u64, rs2: u64) -> u64 {
    let mut r: u64 = 0;
    let mut j = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            if ((rs1 >> j) & 1) != 0 {
                r |= 1 << i;
            }
            j += 1;
        }
    }
    r
}

pub fn clmul32(rs1: u32, rs2: u32) -> u32 {
    let mut x: u32 = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 << i;
        }
    }
    x
}

pub fn clmul64(rs1: u64, rs2: u64) -> u64 {
    let mut x: u64 = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 << i;
        }
    }
    x
}

pub fn clmulh32(rs1: u32, rs2: u32) -> u32 {
    let mut x: u32 = 0;
    for i in 1..32 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (32 - i);
        }
    }
    x
}

pub fn clmulh64(rs1: u64, rs2: u64) -> u64 {
    let mut x: u64 = 0;
    for i in 1..64 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (64 - i);
        }
    }
    x
}

pub fn clmulr32(rs1: u32, rs2: u32) -> u32 {
    let mut x: u32 = 0;
    for i in 0..32 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (31 - i);
        }
    }
    x
}

pub fn clmulr64(rs1: u64, rs2: u64) -> u64 {
    let mut x: u64 = 0;
    for i in 0..64 {
        if ((rs2 >> i) & 1) != 0 {
            x ^= rs1 >> (63 - i);
        }
    }
    x
}

fn shuffle32_stage(src: u32, maskl: u32, maskr: u32, n: u32) -> u32 {
    let mut x = src & !(maskl | maskr);
    x |= ((src << n) & maskl) | ((src >> n) & maskr);
    x
}

fn shuffle64_stage(src: u64, maskl: u64, maskr: u64, n: u64) -> u64 {
    let mut x = src & !(maskl | maskr);
    x |= ((src << n) & maskl) | ((src >> n) & maskr);
    x
}

pub fn shfl32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 15;
    if (shamt & 8) != 0 {
        x = shuffle32_stage(x, 0x00FF_0000, 0x0000_FF00, 8);
    }
    if (shamt & 4) != 0 {
        x = shuffle32_stage(x, 0x0F00_0F00, 0x00F0_00F0, 4);
    }
    if (shamt & 2) != 0 {
        x = shuffle32_stage(x, 0x3030_3030, 0x0C0C_0C0C, 2);
    }
    if (shamt & 1) != 0 {
        x = shuffle32_stage(x, 0x4444_4444, 0x2222_2222, 1);
    }
    x
}

pub fn shfl64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x10) != 0 {
        x = shuffle64_stage(x, 0x0000_FFFF_0000_0000, 0x0000_0000_FFFF_0000, 0x10);
    }
    if (shamt & 0x08) != 0 {
        x = shuffle64_stage(x, 0x00FF_0000_00FF_0000, 0x0000_FF00_0000_FF00, 0x08);
    }
    if (shamt & 0x04) != 0 {
        x = shuffle64_stage(x, 0x0F00_0F00_0F00_0F00, 0x00F0_00F0_00F0_00F0, 0x04);
    }
    if (shamt & 0x02) != 0 {
        x = shuffle64_stage(x, 0x3030_3030_3030_3030, 0x0C0C_0C0C_0C0C_0C0C, 0x02);
    }
    if (shamt & 0x01) != 0 {
        x = shuffle64_stage(x, 0x4444_4444_4444_4444, 0x2222_2222_2222_2222, 0x01);
    }
    x
}

pub fn unshfl32(rs1: u32, rs2: u32) -> u32 {
    let mut x = rs1;
    let shamt = rs2 & 15;
    if (shamt & 1) != 0 {
        x = shuffle32_stage(x, 0x4444_4444, 0x2222_2222, 1);
    }
    if (shamt & 2) != 0 {
        x = shuffle32_stage(x, 0x3030_3030, 0x0C0C_0C0C, 2);
    }
    if (shamt & 4) != 0 {
        x = shuffle32_stage(x, 0x0F00_0F00, 0x00F0_00F0, 4);
    }
    if (shamt & 8) != 0 {
        x = shuffle32_stage(x, 0x00FF_0000, 0x0000_FF00, 8);
    }
    x
}

pub fn unshfl64(rs1: u64, rs2: u64) -> u64 {
    let mut x = rs1;
    let shamt = rs2 & 0x1f;
    if (shamt & 0x01) != 0 {
        x = shuffle64_stage(x, 0x4444_4444_4444_4444, 0x2222_2222_2222_2222, 0x01);
    }
    if (shamt & 0x02) != 0 {
        x = shuffle64_stage(x, 0x3030_3030_3030_3030, 0x0C0C_0C0C_0C0C_0C0C, 0x02);
    }
    if (shamt & 0x04) != 0 {
        x = shuffle64_stage(x, 0x0F00_0F00_0F00_0F00, 0x00F0_00F0_00F0_00F0, 0x04);
    }
    if (shamt & 0x08) != 0 {
        x = shuffle64_stage(x, 0x00FF_0000_00FF_0000, 0x0000_FF00_0000_FF00, 0x08);
    }
    if (shamt & 0x10) != 0 {
        x = shuffle64_stage(x, 0x0000_FFFF_0000_0000, 0x0000_0000_FFFF_0000, 0x10);
    }
    x
}

pub fn slo32(rs1: u32, rs2: u32) -> u32 {
    let shamt = rs2 & 31;
    !(!rs1 << shamt)
}

pub fn slo64(rs1: u64, rs2: u64) -> u64 {
    let shamt = rs2 & 63;
    !(!rs1 << shamt)
}

pub fn bfp32(rs1: u32, rs2: u32) -> u32 {
    let mut cfg: u32 = rs2 >> 16;
    if (cfg >> 30) == 2 {
        cfg >>= 16;
    }
    let mut len = (cfg >> 8) & 15;
    let off = cfg & 31;
    if len == 0 {
        len = 16
    }
    let mask = slo32(0, len) << off;
    let data = rs2 << off;
    (data & mask) | (rs1 & !mask)
}

pub fn bfp64(rs1: u64, rs2: u64) -> u64 {
    let mut cfg: u64 = rs2 >> 32;
    if (cfg >> 30) == 2 {
        cfg >>= 16;
    }
    let mut len = (cfg >> 8) & 31;
    let off = cfg & 63;
    if len == 0 {
        len = 32
    }
    let mask = slo64(0, len) << off;
    let data = rs2 << off;
    (data & mask) | (rs1 & !mask)
}

pub fn crc3232(x: u32, nbits: u32) -> u32 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0xEDB8_8320 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn crc3264(x: u64, nbits: u64) -> u64 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0xEDB8_8320 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn crc32c32(x: u32, nbits: u32) -> u32 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0x82F6_3B78 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn crc32c64(x: u64, nbits: u64) -> u64 {
    let mut r = x;
    for _ in 0..nbits {
        r = (r >> 1) ^ (0x82F6_3B78 & !((r & 1).overflowing_sub(1).0));
    }
    r
}

pub fn bmatflip(rs1: u64) -> u64 {
    let mut x = rs1;
    x = shfl64(x, 31);
    x = shfl64(x, 31);
    x = shfl64(x, 31);
    x
}

pub fn bmatxor(rs1: u64, rs2: u64) -> u64 {
    let rs2t = bmatflip(rs2);
    let mut u: [u8; 8] = [0u8; 8];
    let mut v: [u8; 8] = [0u8; 8];

    for i in 0..8 {
        u[i] = (rs1 >> (i * 8)) as u8;
        v[i] = (rs2t >> (i * 8)) as u8;
    }
    let mut x = 0;
    for i in 0..64 {
        if (u[i / 8] & v[i % 8]).count_ones() & 1 != 0 {
            x |= 1 << i;
        }
    }
    x
}

pub fn bmator(rs1: u64, rs2: u64) -> u64 {
    let rs2t = bmatflip(rs2);
    let mut u: [u8; 8] = [0u8; 8];
    let mut v: [u8; 8] = [0u8; 8];

    for i in 0..8 {
        u[i] = (rs1 >> (i * 8)) as u8;
        v[i] = (rs2t >> (i * 8)) as u8;
    }
    let mut x = 0;
    for i in 0..64 {
        if (u[i / 8] & v[i % 8]) != 0 {
            x |= 1 << i;
        }
    }
    x
}

const eight: u64 = 8;
const args: [u64; 128] = [
    0x0000000000000000,
    0x0000000000000001,
    0xffffffffffffffff,
    0x8000000000000000,
    0x0000000000000004,
    0x0000000000000040,
    0x0000000000000080,
    0x0000000000002000,
    0x0000000000010000,
    0x0000000000400000,
    0x0000000001000000,
    0x0000000100000000,
    0x0000008000000000,
    0x0000020000000000,
    0x0000800000000000,
    0x0010000000000000,
    0x0100000000000000,
    0x1000000000000000,
    0x000000000000000d,
    0x0000000000000000,
    0x0000000000000067,
    0x2e00000000000000,
    0x000000000000015b,
    0x9420000000000000,
    0x00000000000075da,
    0xe35a000000000000,
    0x000000000003ed82,
    0x8b4eb00000000000,
    0x000000000000714c,
    0xfad4dc0000000000,
    0x000000000dd2966b,
    0x686f332000000000,
    0x00000000a865d7d4,
    0x6edd225600000000,
    0x0000000380f3cf69,
    0xaf29109cc0000000,
    0x000000a3714b9ad2,
    0x7dc2ae94e4000000,
    0x00000bea6a6af755,
    0xea2177d8d5100000,
    0x00004a9e26b7f794,
    0x6d159abfb3030000,
    0x00020e6dfbb7c441,
    0xd251a40a022b9000,
    0x00129af7f2440efe,
    0xc7dee68fffbaf900,
    0x05ada4e53975b451,
    0x63eb500cce126b70,
    0x314320aa7da5b1ef,
    0xd27d2fde3497614c,
    0xbe55668178139c8e,
    0x9480583abdfb5837,
    0x9d8dbb3a5bde4347,
    0x61fd04828c93ce01,
    0xdf9a26c8470349dd,
    0xca9d54bd4e78980e,
    0xb1db9b0fecbfaabe,
    0xe79541e25d0dba6b,
    0xff98837fda2a5bdf,
    0xc3bd5e2cd52318a8,
    0x02ab7bb54e687499,
    0xbebf0929f41aa230,
    0x58aee9fdc3f41b74,
    0x62daff171a9fae42,
    0xe5baa16ee5b5419e,
    0x16b3a918e4278c9d,
    0x4ab9cfc9a41744c4,
    0x86ddce906c8cdb4d,
    0x867e3492977cb1bb,
    0x3d0e482377794618,
    0x90e1bc8ba22d3294,
    0xf48119b103954df1,
    0x79780d4e5b2b3b2a,
    0xb36eb1caa58ee7dc,
    0xf0fe55be95a18d13,
    0x1234769364d9eac9,
    0x31a7445bdf8bcb5c,
    0x1735808ee4398bca,
    0x8f09996552504a5d,
    0x4fcf7212bebfdd89,
    0xdfd3a0870f60e072,
    0x25474d793f2c7d32,
    0xb9e2a99fdb7b2948,
    0x0da24e08451a8d1a,
    0x44a705073f90be80,
    0x7f2e6910bdea3ffd,
    0x7fc92593c865b4c2,
    0x0f812a265e560f2b,
    0xfecee737556609f5,
    0x996d1b60923c18a6,
    0x2c1fb5204d248917,
    0x4cf560811e3465c5,
    0xf2a6b292a535dc4e,
    0x3b4de2fabe6d6476,
    0xa6a669d1baba633e,
    0xa73c905bcbc01878,
    0x38be984c83ce8648,
    0x262a15662b298944,
    0xdf09e5c90a990b56,
    0xa8519a5b46242cc0,
    0x14d93f0c55095499,
    0xbad28e0ca5854070,
    0x93d7d7a9d87056f0,
    0x3b0d936889b10a5d,
    0x0ec6680cabb95f09,
    0x27429c30e8b6cff7,
    0x6465f271027abfa8,
    0xd0abd7d3688aa0d7,
    0x986a686578456056,
    0xc10a152d71cb3f16,
    0x4a6c986967d5ace8,
    0x37269c228e8e3db1,
    0xf5bad73c74be6d8a,
    0x68323fe289df33d1,
    0xcb9848f06e9659f6,
    0x5052886f7169c8c5,
    0xb040414dd8c98a14,
    0xea59a91078581c00,
    0x7c6bcb08155fac38,
    0xbd6192029dd91d60,
    0x8a4a182923bdf75a,
    0x8c91e2fe14041a34,
    0xc9d368e6546c1f00,
    0xdfd83d690e5f073e,
    0x34f2a050c605b6b0,
    0xf3fbe985738811dd,
    0x2d21e3da342cd6be,
    0x31523358d080e093,
];

fn main() {
    let mut s = 0;
    for i in args.iter() {
        for j in args.iter() {
            // 700995

            // s = s | (*i + *j)
            // 732797 1.9410400390625


            // s = s | gorc32(*i as u32, *j as u32);
            // 1280083 35.3447265625

            // s = s | gorc64(*i, *j);
            // 1329501 38.3609619140625

            // s = s | grev32(*i as u32, *j as u32);
            // 1270159 34.739013671875

            // s = s | grev64(*i, *j);
            // 1215431 31.398681640625

            // s = s | bext32(*i as u32, *j as u32);
            // 6217409 336.6951904296875

            // s = s | bext64(*i, *j);
            // 11054551 631.930908203125

            // s = s | bdep32(*i as u32, *j as u32);
            // 4894698 255.96331787109375

            // s = s | bdep64(*i, *j);
            // 9464593 534.8875732421875

            // s = s | clmul32(*i as u32, *j as u32);
            // 2410856 104.36163330078125

            // s = s | clmul64(*i, *j);
            // 10796986 616.2103881835938

            // s = s | clmulh32(*i as u32, *j as u32);
            // 2356962 101.07220458984375

            // s = s | clmulh64(*i, *j);
            // 13191525 762.3614501953125

            // s = s | clmulr32(*i as u32, *j as u32);
            // 2408729 104.2318115234375

            // s = s | clmulr64(*i, *j);
            // 12405141 714.3643798828125

            // s = s | shfl32(*i as u32, *j as u32);
            // 1096657 24.1492919921875

            // s = s | shfl64(*i, *j);
            // 1218376 31.57843017578125

            // s = s | unshfl32(*i as u32, *j as u32);
            // 1107998 24.84149169921875

            // s = s | unshfl64(*i, *j);
            // 1238577 32.8114013671875

            // s = s | bfp32(*i as u32, *j as u32);
            // 1082579 23.2900390625

            // s = s | bfp64(*i, *j);
            // 1180634 29.27484130859375

            // s = s | crc3232((*i | *j) as u32, 8);
            // 1170196 28.63775634765625

            // s = s | crc3264((*i|*j), 8);
            // 1160231 28.029541015625

            // s = s | crc32c32((*i | *j) as u32, 8);
            // 1168894 28.55828857421875

            // s = s | crc32c64((*i|*j), 8);
            // 1161276 28.09332275390625

            // s = s | bmatflip(*i | *j);
            // 1819745 68.2830810546875

            // s = s | bmatxor(*i, *j);
            // 30182574 1799.4127807617188

            // s = s | bmator(*i, *j);
            // 16331073 953.9842529296875
        }
    }
    println!("{}", s);
}
