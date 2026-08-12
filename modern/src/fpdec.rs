//! Faithful port of SQLite's binary→decimal rendering (src/util.c
//! sqlite3FpDecode / sqlite3Fp2Convert10 / sqlite3Fp10Convert2 + the printf
//! `%!.17g` assembly used by sqlite3VdbeMemStringify). REAL column text must
//! match the pinned C byte-for-byte, including C's double-rounding artifacts
//! (e.g. 1.0/3.0 → "0.33333333333333332"), which shortest-round-trip printing
//! does not reproduce.

const POWERSOF10_FIRST: i32 = -348;
const POWERSOF10_LAST: i32 = 347;

const A_BASE: [u64; 27] = [
    0x8000000000000000, 0xa000000000000000, 0xc800000000000000, 0xfa00000000000000,
    0x9c40000000000000, 0xc350000000000000, 0xf424000000000000, 0x9896800000000000,
    0xbebc200000000000, 0xee6b280000000000, 0x9502f90000000000, 0xba43b74000000000,
    0xe8d4a51000000000, 0x9184e72a00000000, 0xb5e620f480000000, 0xe35fa931a0000000,
    0x8e1bc9bf04000000, 0xb1a2bc2ec5000000, 0xde0b6b3a76400000, 0x8ac7230489e80000,
    0xad78ebc5ac620000, 0xd8d726b7177a8000, 0x878678326eac9000, 0xa968163f0a57b400,
    0xd3c21bcecceda100, 0x84595161401484a0, 0xa56fa5b99019a5c8,
];
const A_SCALE: [u64; 26] = [
    0x8049a4ac0c5811ae, 0xcf42894a5dce35ea, 0xa76c582338ed2621, 0x873e4f75e2224e68,
    0xda7f5bf590966848, 0xb080392cc4349dec, 0x8e938662882af53e, 0xe65829b3046b0afa,
    0xba121a4650e4ddeb, 0x964e858c91ba2655, 0xf2d56790ab41c2a2, 0xc428d05aa4751e4c,
    0x9e74d1b791e07e48, 0xcccccccccccccccc, 0xcecb8f27f4200f3a, 0xa70c3c40a64e6c51,
    0x86f0ac99b4e8dafd, 0xda01ee641a708de9, 0xb01ae745b101e9e4, 0x8e41ade9fbebc27d,
    0xe5d3ef282a242e81, 0xb9a74a0637ce2ee1, 0x95f83d0a1fb69cd9, 0xf24a01a73cf2dccf,
    0xc3b8358109e84f07, 0x9e19db92b4e31ba9,
];
const A_SCALE_LO: [u32; 26] = [
    0x205b896d, 0x52064cad, 0xaf2af2b8, 0x5a7744a7, 0xaf39a475, 0xbd8d794e, 0x547eb47b,
    0x0cb4a5a3, 0x92f34d62, 0x3a6a07f9, 0xfae27299, 0xaa97e14c, 0x775ea265, 0xcccccccc,
    0x00000000, 0x999090b6, 0x69a028bb, 0xe80e6f48, 0x5ec05dd0, 0x14588f14, 0x8f1668c9,
    0x6d953e2c, 0x4abdaf10, 0xbc633b39, 0x0a862f81, 0x6c07a2c2,
];

fn mul128(a: u64, b: u64) -> (u64, u64) {
    let r = (a as u128) * (b as u128);
    ((r >> 64) as u64, r as u64)
}
fn mul160(a: u64, a_lo: u32, b: u64) -> (u64, u32) {
    let mut r = (a as u128) * (b as u128);
    r += ((a_lo as u128) * (b as u128)) >> 32;
    ((r >> 64) as u64, ((r >> 32) & 0xffff_ffff) as u32)
}

fn power_of_ten(p: i32) -> (u64, u32) {
    debug_assert!((POWERSOF10_FIRST..=POWERSOF10_LAST).contains(&p));
    let (g, n) = if p < 0 {
        if p == -1 { return (A_SCALE[13], A_SCALE_LO[13]); }
        let (mut g, mut n) = (p / 27, p % 27);
        if n != 0 { g -= 1; n += 27; }
        (g, n)
    } else if p < 27 {
        return (A_BASE[p as usize], 0);
    } else {
        (p / 27, p % 27)
    };
    let s = A_SCALE[(g + 13) as usize];
    if n == 0 { return (s, A_SCALE_LO[(g + 13) as usize]); }
    let (mut x, mut lo) = mul160(s, A_SCALE_LO[(g + 13) as usize], A_BASE[n as usize]);
    if x & (1u64 << 63) == 0 {
        x = (x << 1) | ((lo as u64 >> 31) & 1);
        lo = (lo << 1) | 1;
    }
    (x, lo)
}

fn pwr10to2(p: i32) -> i32 { (p * 108853) >> 15 }
fn pwr2to10(p: i32) -> i32 { (p * 78913) >> 18 }

/// binary (m * 2^e) → decimal (d * 10^p) with n significant digits
fn fp2_convert10(m: u64, e: i32, n: i32) -> (u64, i32) {
    let p = n - 1 - pwr2to10(e + 63);
    let (h, _d1) = mul128(m, power_of_ten(p).0);
    let d = if n == 18 {
        let h = h >> (-(e + pwr10to2(p) + 2));
        (h + ((h << 1) & 2)) >> 1
    } else {
        h >> (-(e + pwr10to2(p) + 1))
    };
    (d, -p)
}

/// decimal (d * 10^p) → nearest IEEE754 double (round-trip check helper)
fn fp10_convert2(d: u64, p: i32) -> f64 {
    if p < POWERSOF10_FIRST { return 0.0; }
    if p > POWERSOF10_LAST { return f64::INFINITY; }
    let b = 64 - d.leading_zeros() as i32;
    let lp = pwr10to2(p);
    let mut e = 53 - b - lp;
    if e > 1074 {
        if e >= 1130 { return 0.0; }
        e = 1074;
    }
    let s = -(e - (64 - b) + lp + 3);
    let (mut pwr10h, mut pwr10l) = power_of_ten(p);
    if pwr10l != 0 {
        pwr10h = pwr10h.wrapping_add(1);
        pwr10l = !pwr10l;
    }
    let x = d << (64 - b);
    let (mut hi, lo) = mul128(x, pwr10h);
    let mid1 = (lo >> 32) as u64;
    let mut sticky: u64 = 1;
    if hi & ((1u64 << s) - 1) == 0 {
        let (h2, _l2) = mul128(x, (pwr10l as u64) << 32);
        let mid2 = h2 >> 32;
        sticky = (mid1.wrapping_sub(mid2) > 1) as u64;
        hi -= (mid1 < mid2) as u64;
    }
    let mut u = (hi >> s) | sticky;
    let adj = (u >= (1u64 << 55) - 2) as i32;
    if adj != 0 {
        u = (u >> adj) | (u & 1);
        e -= adj;
    }
    let mut m = (u + 1 + ((u >> 2) & 1)) >> 2;
    if e <= -972 { return f64::INFINITY; }
    if m & (1u64 << 52) != 0 {
        m = (m & !(1u64 << 52)) | (((1075 - e) as u64) << 52);
    }
    f64::from_bits(m)
}

struct FpDec { sign: char, digits: Vec<u8>, i_dp: i32, special: u8 }

/// sqlite3FpDecode port (iRound > 0 path used by %g)
fn fp_decode(mut r: f64, mut i_round: i32, mx_round: i32) -> FpDec {
    let mut out = FpDec { sign: '+', digits: Vec::new(), i_dp: 0, special: 0 };
    if r < 0.0 { out.sign = '-'; r = -r; }
    else if r == 0.0 { out.digits = vec![b'0']; out.i_dp = 1; return out; }
    let bits = r.to_bits();
    let mut e = ((bits >> 52) & 0x7ff) as i32;
    let mut v = bits & 0x000f_ffff_ffff_ffff;
    if e == 0x7ff {
        out.special = 1 + (bits != 0x7ff0_0000_0000_0000) as u8;
        return out;
    }
    if e == 0 {
        let nn = v.leading_zeros() as i32;
        v <<= nn;
        e = -1074 - nn;
    } else {
        v = (v << 11) | (1u64 << 63);
        e -= 1086;
    }
    let want = if i_round <= 0 || i_round >= 18 { 18 } else { i_round + 1 };
    let (v10, exp) = fp2_convert10(v, e, want);
    let mut z: Vec<u8> = v10.to_string().into_bytes();
    let mut n = z.len() as i32;
    out.i_dp = n + exp;
    if i_round <= 0 {
        i_round = out.i_dp - i_round;
        if i_round == 0 && z[0] >= b'5' {
            i_round = 1;
            z.insert(0, b'0');
            n += 1;
            out.i_dp += 1;
        }
    }
    if i_round > 0 && (i_round < n || n > mx_round) {
        if i_round > mx_round { i_round = mx_round; }
        if i_round == 17 {
            // %!.17g precision-reduction: shorter text that round-trips wins
            if n > 15 && z[15] == b'9' && z[14] == b'9' {
                let mut jj = 14usize;
                while jj > 0 && z[jj - 1] == b'9' { jj -= 1; }
                let v2: u64 = if jj == 0 { 1 } else {
                    z[..jj].iter().fold(0u64, |a, &c| a * 10 + (c - b'0') as u64) + 1
                };
                if r == fp10_convert2(v2, exp + n - jj as i32) { i_round = jj as i32 + 1; }
            } else if out.i_dp >= n || (n > 15 && z[15] == b'0' && z[14] == b'0' && z[13] == b'0') {
                let mut jj = 13usize;
                while jj > 0 && z[jj - 1] == b'0' { jj -= 1; }
                let v2: u64 = z[..jj].iter().fold(0u64, |a, &c| a * 10 + (c - b'0') as u64);
                if r == fp10_convert2(v2, exp + n - jj as i32) { i_round = jj as i32 + 1; }
            }
        }
        n = i_round;
        if z.get(i_round as usize).map_or(false, |&c| c >= b'5') {
            let mut j = (i_round - 1) as usize;
            loop {
                z[j] += 1;
                if z[j] <= b'9' { break; }
                z[j] = b'0';
                if j == 0 {
                    z.insert(0, b'1');
                    n += 1;
                    out.i_dp += 1;
                    break;
                }
                j -= 1;
            }
        }
    }
    while n > 1 && z[(n - 1) as usize] == b'0' { n -= 1; }
    z.truncate(n as usize);
    out.digits = z;
    out
}

/// the `%!.17g` text SQLite produces for a REAL value (vdbemem stringify path)
pub fn render_f64_c(r: f64) -> String {
    let s = fp_decode(r, 17, 20); // '!' flag → mxRound 20
    if s.special != 0 {
        // NaN never reaches SQL text (stored NULL); Inf renders like C's %!.17g
        let body = if s.special == 2 { "NaN" } else { "Inf" };
        return if s.sign == '-' && s.special == 1 { format!("-{body}") } else { body.into() };
    }
    let exp10 = s.i_dp - 1;
    let mut precision: i32 = 17 - 1; // etGENERIC decrements
    let use_exp = exp10 < -4 || exp10 > precision;
    let mut outb = String::new();
    if s.sign == '-' { outb.push('-'); }
    let z = &s.digits;
    let sn = z.len() as i32;
    let e2 = if use_exp { 0 } else { s.i_dp - 1 };
    if !use_exp { precision -= exp10; }
    let mut j: i32 = 0;
    if e2 < 0 {
        outb.push('0');
    } else {
        j = (e2 + 1).min(sn);
        outb.push_str(std::str::from_utf8(&z[..j as usize]).unwrap());
        let fill = e2 - j;
        if fill >= 0 { for _ in 0..=fill { outb.push('0'); } }
    }
    let mut e2m = if e2 >= 0 && e2 + 1 > sn { -1 } else { e2 - j };
    if e2 < 0 { e2m = e2; }
    outb.push('.');
    let mut prec = precision;
    if e2m < -1 && prec > 0 {
        let nn = (-1 - e2m).min(prec);
        for _ in 0..nn { outb.push('0'); }
        prec -= nn;
    }
    if prec > 0 {
        let nn = (sn - j).min(prec);
        if nn > 0 {
            outb.push_str(std::str::from_utf8(&z[j as usize..(j + nn) as usize]).unwrap());
        }
        // flag_rtz is set for %!g: trailing zeros are stripped below, so no zero-fill
    }
    // rtz: strip trailing zeros; altform2 keeps one digit after '.'
    while outb.ends_with('0') { outb.pop(); }
    if outb.ends_with('.') { outb.push('0'); }
    if use_exp {
        let mut e = s.i_dp - 1;
        outb.push('e');
        if e < 0 { outb.push('-'); e = -e; } else { outb.push('+'); }
        if e >= 100 {
            outb.push((b'0' + (e / 100) as u8) as char);
            e %= 100;
        }
        outb.push((b'0' + (e / 10) as u8) as char);
        outb.push((b'0' + (e % 10) as u8) as char);
    }
    outb
}
