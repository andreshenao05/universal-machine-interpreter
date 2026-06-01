pub fn fitss(n: i64, width: u64) -> bool {
    if width == 0 {
        return n == 0;
    }

    if width >= 64 {
        return true;
    }

    let min = -(1i64 << (width - 1));
    let max = (1i64 << (width - 1)) - 1;
    n >= min && n <= max
}

pub fn fitsu(n: u64, width: u64) -> bool {
    if width == 0 {
        return n == 0;
    }

    if width >= 64 {
        return true;
    }

    n < (1u64 << width)
}

pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    if width > 64 || lsb > 64 || width > 64 - lsb {
        return None;
    }

    if width == 0 {
        return Some(0);
    }

    let unsigned = getu(word, width, lsb)?;

    if width == 64 {
        return Some(unsigned as i64);
    }

    let sign_bit = 1u64 << (width - 1);

    if unsigned & sign_bit == 0 {
        Some(unsigned as i64)
    } else {
        Some((unsigned | (!0u64 << width)) as i64)
    }
}

pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    if width > 64 || lsb > 64 || width > 64 - lsb {
        return None;
    }

    if width == 0 {
        return Some(0);
    }

    if width == 64 {
        return Some(word);
    }

    let mask = (1u64 << width) - 1;
    Some((word >> lsb) & mask)
}

pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if width > 64 || lsb > 64 || width > 64 - lsb {
        return None;
    }

    if !fitsu(value, width) {
        return None;
    }

    if width == 0 {
        return Some(word);
    }

    if width == 64 {
        return Some(value);
    }

    let field_mask = ((1u64 << width) - 1) << lsb;
    let cleared_word = word & !field_mask;
    let shifted_value = value << lsb;

    Some(cleared_word | shifted_value)
}

pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if width > 64 || lsb > 64 || width > 64 - lsb {
        return None;
    }

    if !fitss(value, width) {
        return None;
    }

    if width == 0 {
        return Some(word);
    }

    let unsigned_value = if width == 64 {
        value as u64
    } else {
        (value as u64) & ((1u64 << width) - 1)
    };

    newu(word, width, lsb, unsigned_value)
}