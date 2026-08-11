//! Real date/time engine (pack v10) — julian-day arithmetic matching SQLite 3.54.0
//! date.c for the pinned scopes: date/time/datetime/julianday/unixepoch/strftime/
//! timediff plus modifiers (+/-N units, month/year with day-overflow, weekday N,
//! start of day/month/year, unixepoch). 'now'/'localtime' are NOT implemented
//! (nondeterministic / TZ-dependent) — they error honestly.

use crate::eval::V;

const MS_DAY: i64 = 86_400_000;
/// iJD of the unix epoch (matches date.c: unixepoch = (iJD - 21086676*10^9)/1000)
const EPOCH_JD_MS: i64 = 210_866_760_000_000;

#[derive(Clone, Copy)]
pub struct Dt {
    pub jd: i64,       // julian day in milliseconds (like date.c iJD)
    pub raw_num: bool, // value came from a bare numeric (eligible for 'unixepoch' modifier)
}

fn compute_jd(y: i64, mo: i64, d: i64, ms_of_day: i64) -> i64 {
    // integer-exact version: (X1+X2+D+B-1524.5) days -> *MS_DAY
    let (mut yy, mut mm) = (y, mo);
    if mm <= 2 { yy -= 1; mm += 12; }
    let a = yy / 100;
    let b = 2 - a + a / 4;
    let x1 = 36525 * (yy + 4716) / 100;
    let x2 = 306001 * (mm + 1) / 10000;
    let days2 = 2 * (x1 + x2 + d + b) - 3049; // 2*(days - 1524.5)
    days2 * (MS_DAY / 2) + ms_of_day
}

pub fn jd_to_ymd(jd: i64) -> (i64, i64, i64) {
    // date.c computeYMD
    let z = (jd + 43_200_000) / MS_DAY;
    let mut a = ((z as f64 - 1_867_216.25) / 36524.25) as i64;
    a = z + 1 + a - a / 4;
    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25) as i64;
    let d = 36525 * c / 100;
    let e = ((b - d) as f64 / 30.6001) as i64;
    let x1 = (30.6001 * e as f64) as i64;
    let day = b - d - x1;
    let month = if e < 14 { e - 1 } else { e - 13 };
    let year = if month > 2 { c - 4716 } else { c - 4715 };
    (year, month, day)
}

pub fn jd_to_hms(jd: i64) -> (i64, i64, i64, i64) {
    // date.c computeHMS: (h, m, s, ms)
    let mut s = (jd + 43_200_000) % MS_DAY;
    let ms = s % 1000;
    s /= 1000;
    let h = s / 3600;
    s -= h * 3600;
    let m = s / 60;
    s -= m * 60;
    (h, m, s, ms)
}

fn parse_text(t: &str) -> Option<i64> {
    let t = t.trim();
    let b: Vec<char> = t.chars().collect();
    let num = |s: &[char]| -> Option<i64> { s.iter().collect::<String>().parse().ok() };
    if b.len() < 10 || b[4] != '-' || b[7] != '-' { return None; }
    let y = num(&b[0..4])?; let mo = num(&b[5..7])?; let d = num(&b[8..10])?;
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) { return None; }
    let mut ms_of_day = 0i64;
    if b.len() > 10 {
        if b.len() < 16 || (b[10] != ' ' && b[10] != 'T') || b[13] != ':' { return None; }
        let h = num(&b[11..13])?; let mi = num(&b[14..16])?;
        let mut sec = 0i64; let mut frac_ms = 0i64;
        if b.len() >= 19 {
            if b[16] != ':' { return None; }
            sec = num(&b[17..19])?;
            if b.len() > 20 && b[19] == '.' {
                let fs: String = b[20..].iter().take(3).collect();
                frac_ms = format!("{:0<3}", fs).parse().ok()?;
            }
        }
        ms_of_day = ((h * 60 + mi) * 60 + sec) * 1000 + frac_ms;
    }
    Some(compute_jd(y, mo, d, ms_of_day))
}

pub fn parse_value(v: &V) -> Result<Dt, String> {
    match v {
        V::Int(i) => Ok(Dt { jd: i * MS_DAY, raw_num: true }),
        V::Real(r) => Ok(Dt { jd: (r * MS_DAY as f64).round() as i64, raw_num: true }),
        V::Text(t) => {
            if let Some(jd) = parse_text(t) { return Ok(Dt { jd, raw_num: false }); }
            if let Ok(i) = t.trim().parse::<f64>() { return Ok(Dt { jd: (i * MS_DAY as f64).round() as i64, raw_num: true }); }
            Err(format!("unsupported date/time value: {t}"))
        }
        _ => Err("unsupported date/time value".into()),
    }
}

fn weekday_sun0(jd: i64) -> i64 {
    // unix epoch 1970-01-01 was a Thursday (Sunday=0 -> 4)
    let days = (jd - EPOCH_JD_MS).div_euclid(MS_DAY);
    (days.rem_euclid(7) + 4) % 7
}

pub fn apply_modifier(dt: &mut Dt, m: &str) -> Result<(), String> {
    let ml = m.trim().to_ascii_lowercase();
    if ml == "unixepoch" {
        if !dt.raw_num { return Err("'unixepoch' modifier requires a numeric value".into()); }
        // reinterpret the raw number as unix seconds
        let secs = dt.jd / MS_DAY; // we stored value*MS_DAY
        dt.jd = EPOCH_JD_MS + secs * 1000;
        dt.raw_num = false;
        return Ok(());
    }
    if let Some(rest) = ml.strip_prefix("weekday ") {
        let n: i64 = rest.trim().parse().map_err(|_| "bad weekday")?;
        let wd = weekday_sun0(dt.jd);
        dt.jd += ((n - wd + 7) % 7) * MS_DAY;
        return Ok(());
    }
    if let Some(rest) = ml.strip_prefix("start of ") {
        let (y, mo, _d) = jd_to_ymd(dt.jd);
        dt.jd = match rest.trim() {
            "day" => (dt.jd + 43_200_000).div_euclid(MS_DAY) * MS_DAY - 43_200_000,
            "month" => compute_jd(y, mo, 1, 0),
            "year" => compute_jd(y, 1, 1, 0),
            _ => return Err(format!("unsupported modifier: {m}")),
        };
        return Ok(());
    }
    // signed amount + unit
    let (num_str, unit) = {
        let t = ml.trim();
        let end = t.find(|c: char| c.is_ascii_alphabetic() && c != 'e')
            .or_else(|| t.find(' ')).unwrap_or(t.len());
        (t[..end].trim().to_string(), t[end..].trim().to_string())
    };
    let n: f64 = num_str.parse().map_err(|_| format!("unsupported modifier: {m}"))?;
    let unit = unit.trim_end_matches('s');
    match unit {
        "day" => { dt.jd += (n * MS_DAY as f64).round() as i64; }
        "hour" => { dt.jd += (n * 3_600_000.0).round() as i64; }
        "minute" => { dt.jd += (n * 60_000.0).round() as i64; }
        "second" => { dt.jd += (n * 1000.0).round() as i64; }
        "month" | "year" => {
            let (y, mo, d) = jd_to_ymd(dt.jd);
            let time_ms = (dt.jd + 43_200_000).rem_euclid(MS_DAY);
            let add_months = if unit == "month" { n as i64 } else { n as i64 * 12 };
            let ytot = y * 12 + (mo - 1) + add_months;
            let (ny, nmo) = (ytot.div_euclid(12), ytot.rem_euclid(12) + 1);
            // keep day-of-month; overflow normalizes via computeJD (Feb 31 -> Mar 3)
            dt.jd = compute_jd(ny, nmo, d, time_ms);
        }
        _ => return Err(format!("unsupported modifier: {m}")),
    }
    Ok(())
}

pub fn build(args: &[V]) -> Result<Dt, String> {
    let mut dt = parse_value(&args[0])?;
    for m in &args[1..] {
        apply_modifier(&mut dt, &m.render().unwrap_or_default())?;
    }
    Ok(dt)
}

pub fn fmt_date(dt: &Dt) -> String {
    let (y, m, d) = jd_to_ymd(dt.jd);
    format!("{:04}-{:02}-{:02}", y, m, d)
}
pub fn fmt_time(dt: &Dt) -> String {
    let (h, m, s, _) = jd_to_hms(dt.jd);
    format!("{:02}:{:02}:{:02}", h, m, s)
}
pub fn fmt_datetime(dt: &Dt) -> String {
    format!("{} {}", fmt_date(dt), fmt_time(dt))
}
pub fn unix_seconds(dt: &Dt) -> i64 {
    (dt.jd - EPOCH_JD_MS).div_euclid(1000)
}

pub fn strftime(fmt: &str, dt: &Dt) -> Result<String, String> {
    let (y, mo, d) = jd_to_ymd(dt.jd);
    let (h, mi, s, ms) = jd_to_hms(dt.jd);
    let mut out = String::new();
    let mut it = fmt.chars().peekable();
    while let Some(c) = it.next() {
        if c != '%' { out.push(c); continue; }
        match it.next() {
            Some('Y') => out.push_str(&format!("{:04}", y)),
            Some('m') => out.push_str(&format!("{:02}", mo)),
            Some('d') => out.push_str(&format!("{:02}", d)),
            Some('e') => out.push_str(&format!("{:2}", d)),
            Some('H') => out.push_str(&format!("{:02}", h)),
            Some('M') => out.push_str(&format!("{:02}", mi)),
            Some('S') => out.push_str(&format!("{:02}", s)),
            Some('f') => out.push_str(&format!("{:02}.{:03}", s, ms)),
            Some('s') => out.push_str(&unix_seconds(dt).to_string()),
            Some('w') => out.push_str(&weekday_sun0(dt.jd).to_string()),
            Some('j') => {
                let jan1 = compute_jd(y, 1, 1, 0);
                let doy = (dt.jd + 43_200_000).div_euclid(MS_DAY) - (jan1 + 43_200_000).div_euclid(MS_DAY) + 1;
                out.push_str(&format!("{:03}", doy));
            }
            Some('J') => out.push_str(&crate::eval::V::Real(dt.jd as f64 / MS_DAY as f64).render().unwrap_or_default()),
            Some('F') => out.push_str(&fmt_date(dt)),
            Some('T') => out.push_str(&fmt_time(dt)),
            Some('R') => out.push_str(&format!("{:02}:{:02}", h, mi)),
            Some('%') => out.push('%'),
            Some(o) => return Err(format!("unsupported strftime directive %{o}")),
            None => {}
        }
    }
    Ok(out)
}

pub fn timediff(a: &V, b: &V) -> Result<String, String> {
    let d1 = parse_value(a)?;
    let d2 = parse_value(b)?;
    let (sign, hi, lo) = if d1.jd >= d2.jd { ('+', d1, d2) } else { ('-', d2, d1) };
    // years: move lo forward by whole years without passing hi (date.c approach)
    let (mut ly, mut lm, ld) = jd_to_ymd(lo.jd);
    let ltime = (lo.jd + 43_200_000).rem_euclid(MS_DAY);
    let (hy, _hm, _hd) = jd_to_ymd(hi.jd);
    let mut years = hy - ly;
    while years > 0 && compute_jd(ly + years, lm, ld, ltime) > hi.jd { years -= 1; }
    ly += years;
    let mut cur = compute_jd(ly, lm, ld, ltime);
    let mut months = 0i64;
    loop {
        let mut ny = ly; let mut nm = lm + 1;
        if nm > 12 { nm = 1; ny += 1; }
        let nxt = compute_jd(ny, nm, ld, ltime);
        if nxt > hi.jd { break; }
        ly = ny; lm = nm; cur = nxt; months += 1;
    }
    let mut rem = hi.jd - cur;
    let days = rem / MS_DAY; rem %= MS_DAY;
    let hh = rem / 3_600_000; rem %= 3_600_000;
    let mm = rem / 60_000; rem %= 60_000;
    let ss = rem / 1000; let msx = rem % 1000;
    Ok(format!("{}{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}", sign, years, months, days, hh, mm, ss, msx))
}
