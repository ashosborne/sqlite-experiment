//! Minimal real JSON evaluator for the pack-v8 json_* pins (parse → path → mutate →
//! canonical serialize). Not a whole JSON1 clone; covers the pinned inputs honestly.

use crate::eval::V;

#[derive(Clone, Debug)]
pub enum J { Null, Bool(bool), Int(i64), Real(f64), Str(String), Arr(Vec<J>), Obj(Vec<(String, J)>) }

struct JP { b: Vec<char>, i: usize }
impl JP {
    fn ws(&mut self) { while self.i < self.b.len() && self.b[self.i].is_whitespace() { self.i += 1; } }
    fn parse(&mut self) -> Result<J, String> {
        self.ws();
        let c = *self.b.get(self.i).ok_or("eof")?;
        match c {
            '{' => { self.i += 1; let mut o = Vec::new(); self.ws();
                if self.b.get(self.i) == Some(&'}') { self.i += 1; return Ok(J::Obj(o)); }
                loop { self.ws(); let k = self.pstr()?; self.ws();
                    if self.b.get(self.i) != Some(&':') { return Err("expected :".into()); } self.i += 1;
                    let v = self.parse()?; o.push((k, v)); self.ws();
                    match self.b.get(self.i) { Some(',') => { self.i += 1; } Some('}') => { self.i += 1; break; } _ => return Err("bad obj".into()) } }
                Ok(J::Obj(o)) }
            '[' => { self.i += 1; let mut a = Vec::new(); self.ws();
                if self.b.get(self.i) == Some(&']') { self.i += 1; return Ok(J::Arr(a)); }
                loop { let v = self.parse()?; a.push(v); self.ws();
                    match self.b.get(self.i) { Some(',') => { self.i += 1; } Some(']') => { self.i += 1; break; } _ => return Err("bad arr".into()) } }
                Ok(J::Arr(a)) }
            '"' => Ok(J::Str(self.pstr()?)),
            't' => { self.expect("true")?; Ok(J::Bool(true)) }
            'f' => { self.expect("false")?; Ok(J::Bool(false)) }
            'n' => { self.expect("null")?; Ok(J::Null) }
            _ => self.pnum(),
        }
    }
    fn expect(&mut self, w: &str) -> Result<(), String> {
        for c in w.chars() { if self.b.get(self.i) != Some(&c) { return Err("bad literal".into()); } self.i += 1; } Ok(())
    }
    fn pstr(&mut self) -> Result<String, String> {
        if self.b.get(self.i) != Some(&'"') { return Err("expected string".into()); }
        self.i += 1; let mut s = String::new();
        while let Some(&c) = self.b.get(self.i) { self.i += 1;
            match c { '"' => return Ok(s), '\\' => { let e = *self.b.get(self.i).ok_or("bad esc")?; self.i += 1;
                s.push(match e { 'n'=>'\n','t'=>'\t','r'=>'\r','"'=>'"','\\'=>'\\','/'=>'/', _=>e }); } _ => s.push(c) } }
        Err("unterminated string".into())
    }
    fn pnum(&mut self) -> Result<J, String> {
        let start = self.i; let mut isint = true;
        if self.b.get(self.i) == Some(&'-') { self.i += 1; }
        // run-46: strict JSON numbers need a digit before any '.' (C rejects '.5')
        if !matches!(self.b.get(self.i), Some(c) if c.is_ascii_digit()) { return Err("bad number".into()); }
        while let Some(&c) = self.b.get(self.i) { if c.is_ascii_digit() { self.i += 1; }
            else if c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' { isint = false; self.i += 1; } else { break; } }
        let t: String = self.b[start..self.i].iter().collect();
        if t.is_empty() { return Err("bad number".into()); }
        if isint { t.parse::<i64>().map(J::Int).map_err(|_| "bad int".into()) }
        else { t.parse::<f64>().map(J::Real).map_err(|_| "bad real".into()) }
    }
}

pub fn parse(doc: &str) -> Result<J, String> { let mut p = JP { b: doc.chars().collect(), i: 0 }; let v = p.parse()?; p.ws(); Ok(v) }
pub fn valid(doc: &str) -> bool { let mut p = JP { b: doc.chars().collect(), i: 0 }; match p.parse() { Ok(_) => { p.ws(); p.i >= p.b.len() } Err(_) => false } }

// ---------------- run-46: JSON5 validity (json_valid flags=2) ----------------
// A validity-only recursive-descent scanner for the JSON5 forms the pin exercises:
// // and /* */ comments, bare identifier object keys, single-quoted strings,
// trailing commas, hex integers, leading/trailing-dot decimals, signed Infinity/NaN.
struct J5 { b: Vec<char>, i: usize }
impl J5 {
    fn ws(&mut self) {
        loop {
            while self.i < self.b.len() && self.b[self.i].is_whitespace() { self.i += 1; }
            if self.i + 1 < self.b.len() && self.b[self.i] == '/' && self.b[self.i + 1] == '/' {
                while self.i < self.b.len() && self.b[self.i] != '\n' { self.i += 1; }
                continue;
            }
            if self.i + 1 < self.b.len() && self.b[self.i] == '/' && self.b[self.i + 1] == '*' {
                self.i += 2;
                while self.i + 1 < self.b.len() && !(self.b[self.i] == '*' && self.b[self.i + 1] == '/') { self.i += 1; }
                if self.i + 1 >= self.b.len() { self.i = self.b.len(); return; }
                self.i += 2;
                continue;
            }
            return;
        }
    }
    fn peek(&self) -> Option<char> { self.b.get(self.i).copied() }
    fn string(&mut self, q: char) -> bool {
        self.i += 1;
        while let Some(c) = self.peek() {
            if c == q { self.i += 1; return true; }
            if c == '\\' { self.i += 1; }
            self.i += 1;
        }
        false
    }
    fn ident_key(&mut self) -> bool {
        let start = self.i;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '$' { self.i += 1; } else { break; }
        }
        self.i > start && !self.b[start].is_ascii_digit()
    }
    fn number(&mut self) -> bool {
        if matches!(self.peek(), Some('+') | Some('-')) { self.i += 1; }
        let rest: String = self.b[self.i..].iter().collect();
        if rest.starts_with("Infinity") { self.i += 8; return true; }
        if rest.starts_with("NaN") { self.i += 3; return true; }
        if rest.starts_with("0x") || rest.starts_with("0X") {
            self.i += 2;
            let s = self.i;
            while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) { self.i += 1; }
            return self.i > s;
        }
        let s = self.i;
        let mut digits = false;
        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) { self.i += 1; digits = true; }
        if self.peek() == Some('.') {
            self.i += 1;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) { self.i += 1; digits = true; }
        }
        if !digits { self.i = s; return false; }
        if matches!(self.peek(), Some('e') | Some('E')) {
            self.i += 1;
            if matches!(self.peek(), Some('+') | Some('-')) { self.i += 1; }
            let e = self.i;
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) { self.i += 1; }
            if self.i == e { return false; }
        }
        true
    }
    fn value(&mut self) -> bool {
        self.ws();
        match self.peek() {
            Some('{') => {
                self.i += 1;
                loop {
                    self.ws();
                    if self.peek() == Some('}') { self.i += 1; return true; }
                    let ok = match self.peek() {
                        Some('"') => self.string('"'),
                        Some('\'') => self.string('\''),
                        _ => self.ident_key(),
                    };
                    if !ok { return false; }
                    self.ws();
                    if self.peek() != Some(':') { return false; }
                    self.i += 1;
                    if !self.value() { return false; }
                    self.ws();
                    match self.peek() {
                        Some(',') => { self.i += 1; }
                        Some('}') => { self.i += 1; return true; }
                        _ => return false,
                    }
                }
            }
            Some('[') => {
                self.i += 1;
                loop {
                    self.ws();
                    if self.peek() == Some(']') { self.i += 1; return true; }
                    if !self.value() { return false; }
                    self.ws();
                    match self.peek() {
                        Some(',') => { self.i += 1; }
                        Some(']') => { self.i += 1; return true; }
                        _ => return false,
                    }
                }
            }
            Some('"') => self.string('"'),
            Some('\'') => self.string('\''),
            Some(c) if c == '+' || c == '-' || c == '.' || c.is_ascii_digit()
                || c == 'I' || c == 'N' => {
                if c == '.' { // leading-dot decimal
                    self.i += 1;
                    let s = self.i;
                    while matches!(self.peek(), Some(d) if d.is_ascii_digit()) { self.i += 1; }
                    return self.i > s;
                }
                self.number()
            }
            Some('t') => { let r: String = self.b[self.i..].iter().collect();
                if r.starts_with("true") { self.i += 4; true } else { false } }
            Some('f') => { let r: String = self.b[self.i..].iter().collect();
                if r.starts_with("false") { self.i += 5; true } else { false } }
            Some('n') => { let r: String = self.b[self.i..].iter().collect();
                if r.starts_with("null") { self.i += 4; true } else { false } }
            _ => false,
        }
    }
}
pub fn valid5(doc: &str) -> bool {
    let mut p = J5 { b: doc.chars().collect(), i: 0 };
    if !p.value() { return false; }
    p.ws();
    p.i >= p.b.len()
}

// ---------------- run-46: JSONB byte validity (json_valid flags=4/8) ----------------
// Walks the binary element tree per the JSONB header rules: low nibble = element
// type (0..12), high nibble = payload size or a 12..15 marker for 1/2/4/8 size
// bytes; containers must be exactly filled by child elements.
fn jsonb_element(b: &[u8], pos: usize) -> Option<usize> {
    let h = *b.get(pos)?;
    let ty = h & 0x0f;
    if ty > 12 { return None; }
    let (psz, hdr) = match h >> 4 {
        n @ 0..=11 => (n as usize, 1usize),
        12 => (*b.get(pos + 1)? as usize, 2),
        13 => (u16::from_be_bytes([*b.get(pos + 1)?, *b.get(pos + 2)?]) as usize, 3),
        14 => (u32::from_be_bytes([*b.get(pos + 1)?, *b.get(pos + 2)?, *b.get(pos + 3)?, *b.get(pos + 4)?]) as usize, 5),
        _ => return None, // 8-byte sizes exceed any pinned payload
    };
    let start = pos + hdr;
    let end = start.checked_add(psz)?;
    if end > b.len() { return None; }
    if ty == 11 || ty == 12 {
        // ARRAY / OBJECT: children fill the payload exactly
        let mut p = start;
        while p < end { p = jsonb_element(b, p)?; if p > end { return None; } }
        if p != end { return None; }
    }
    if matches!(ty, 0 | 1 | 2) && psz != 0 { return None; } // NULL/TRUE/FALSE carry no payload
    Some(end)
}
pub fn jsonb_valid(b: &[u8]) -> bool {
    if b.is_empty() { return false; }
    jsonb_element(b, 0) == Some(b.len())
}

pub fn serialize(j: &J) -> String {
    match j {
        J::Null => "null".into(), J::Bool(b) => if *b { "true".into() } else { "false".into() },
        J::Int(i) => i.to_string(), J::Real(r) => format!("{}", r),
        J::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
        J::Arr(a) => format!("[{}]", a.iter().map(serialize).collect::<Vec<_>>().join(",")),
        J::Obj(o) => format!("{{{}}}", o.iter().map(|(k, v)| format!("\"{}\":{}", k, serialize(v))).collect::<Vec<_>>().join(",")),
    }
}

fn path_steps(path: &str) -> Vec<String> {
    // "$.a.b" / "$[0]" / "$.a" -> ["a","b"] / ["0"] / ["a"]
    let mut out = Vec::new(); let mut cur = String::new(); let cs: Vec<char> = path.chars().collect(); let mut i = 0;
    if cs.first() == Some(&'$') { i = 1; }
    while i < cs.len() {
        match cs[i] {
            '.' => { if !cur.is_empty() { out.push(std::mem::take(&mut cur)); } i += 1; }
            '[' => { if !cur.is_empty() { out.push(std::mem::take(&mut cur)); } i += 1;
                     let mut idx = String::new(); while i < cs.len() && cs[i] != ']' { idx.push(cs[i]); i += 1; } i += 1; out.push(idx); }
            c => { cur.push(c); i += 1; }
        }
    }
    if !cur.is_empty() { out.push(cur); }
    out
}

/// run-53: resolve an array-path step against a length: "N", "#" (== len) or "#-K"
fn arr_idx(step: &str, len: usize) -> Option<usize> {
    if let Some(rest) = step.strip_prefix('#') {
        if rest.is_empty() { return Some(len); }
        if let Some(k) = rest.strip_prefix('-') {
            return k.trim().parse::<usize>().ok().and_then(|k| len.checked_sub(k));
        }
        return None;
    }
    step.parse::<usize>().ok()
}

fn get<'a>(j: &'a J, steps: &[String]) -> Option<&'a J> {
    let mut cur = j;
    for s in steps {
        cur = match cur {
            J::Obj(o) => o.iter().find(|(k, _)| k == s).map(|(_, v)| v)?,
            J::Arr(a) => a.get(s.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(cur)
}

pub fn extract(doc: &str, path: &str) -> Result<J, String> {
    let j = parse(doc)?; Ok(get(&j, &path_steps(path)).cloned().unwrap_or(J::Null))
}
pub fn to_sql_text(j: J) -> V {
    match j { J::Null => V::Null, J::Bool(b) => V::Int(b as i64), J::Int(i) => V::Int(i),
              J::Real(r) => V::Real(r), J::Str(s) => V::Text(s), other => V::Text(serialize(&other)) }
}
pub fn to_json_text(j: J) -> V { V::Text(serialize(&j)) }
pub fn type_at(doc: &str, path: &str) -> Option<String> {
    let j = parse(doc).ok()?; let v = get(&j, &path_steps(path))?;
    Some(match v { J::Null=>"null",J::Bool(_)=>"true",J::Int(_)=>"integer",J::Real(_)=>"real",J::Str(_)=>"text",J::Arr(_)=>"array",J::Obj(_)=>"object" }.into())
}
pub fn each(doc: &str) -> Result<Vec<V>, String> {
    let j = parse(doc)?;
    Ok(match j { J::Arr(a) => a.into_iter().map(to_sql_text).collect(),
                 J::Obj(o) => o.into_iter().map(|(_, v)| to_sql_text(v)).collect(),
                 other => vec![to_sql_text(other)] })
}

// ---- run-50: full json_each/json_tree vtab columns with C's JSONB-offset ids ----

/// one json_each/json_tree row (all eight vtab columns)
pub struct WalkRow {
    pub key: Option<V>,
    pub value: V,
    pub jtype: String,
    pub atom: Option<V>,
    pub id: i64,
    pub parent: Option<i64>,
    pub fullkey: String,
    pub path: String,
}

/// size of a JSONB header for a payload of n bytes (1 / 2 / 3 / 5 / 9)
fn jsonb_hdr(n: usize) -> usize {
    if n <= 11 { 1 } else if n <= 0xFF { 2 } else if n <= 0xFFFF { 3 }
    else if n <= 0xFFFF_FFFF { 5 } else { 9 }
}
/// total encoded size of a node in C's JSONB format (ints/reals as ASCII text,
/// strings raw, true/false/null header-only, containers = header + children)
fn jsonb_size(j: &J) -> usize {
    match j {
        J::Null | J::Bool(_) => 1,
        J::Int(i) => { let p = i.to_string().len(); jsonb_hdr(p) + p }
        J::Real(r) => { let p = format!("{r}").len(); jsonb_hdr(p) + p }
        J::Str(s) => { let p = s.len(); jsonb_hdr(p) + p }
        J::Arr(a) => { let p: usize = a.iter().map(jsonb_size).sum(); jsonb_hdr(p) + p }
        J::Obj(o) => {
            let p: usize = o.iter().map(|(k, v)| jsonb_size(&J::Str(k.clone())) + jsonb_size(v)).sum();
            jsonb_hdr(p) + p
        }
    }
}
fn jtype_of(j: &J) -> &'static str {
    match j { J::Null => "null", J::Bool(true) => "true", J::Bool(false) => "false",
              J::Int(_) => "integer", J::Real(_) => "real", J::Str(_) => "text",
              J::Arr(_) => "array", J::Obj(_) => "object" }
}
fn jvalue_of(j: &J) -> V { to_sql_text(j.clone()) }
fn jatom_of(j: &J) -> Option<V> {
    match j { J::Arr(_) | J::Obj(_) => None, other => Some(to_sql_text(other.clone())) }
}

/// emit the row(s) for node `j` located at JSONB offset `off`, then (when
/// `recursive`) its children. `parent` is the id of the parent ROW like C.
fn walk_node(j: &J, off: usize, key: Option<V>, parent: Option<i64>,
             fullkey: &str, path: &str, recursive: bool, emit_self: bool,
             out: &mut Vec<WalkRow>) {
    let my_id = off as i64;
    if emit_self {
        out.push(WalkRow {
            key, value: jvalue_of(j), jtype: jtype_of(j).into(), atom: jatom_of(j),
            id: my_id, parent, fullkey: fullkey.to_string(), path: path.to_string(),
        });
    }
    let child_parent = if emit_self { Some(my_id) } else { parent };
    match j {
        J::Arr(a) => {
            let mut co = off + jsonb_hdr(a.iter().map(jsonb_size).sum());
            for (i, v) in a.iter().enumerate() {
                let fk = format!("{fullkey}[{i}]");
                if recursive {
                    walk_node(v, co, Some(V::Int(i as i64)), child_parent, &fk, fullkey, true, true, out);
                } else {
                    out.push(WalkRow { key: Some(V::Int(i as i64)), value: jvalue_of(v),
                        jtype: jtype_of(v).into(), atom: jatom_of(v), id: co as i64,
                        parent: None, fullkey: fk, path: fullkey.to_string() });
                }
                co += jsonb_size(v);
            }
        }
        J::Obj(o) => {
            let payload: usize = o.iter().map(|(k, v)| jsonb_size(&J::Str(k.clone())) + jsonb_size(v)).sum();
            let mut co = off + jsonb_hdr(payload);
            for (k, v) in o {
                // the member row's id is the offset of its KEY node (C shape)
                let key_off = co;
                let vo = co + jsonb_size(&J::Str(k.clone()));
                let fk = format!("{fullkey}.{k}");
                if recursive {
                    // recurse with the value's children, but the row id is the key offset
                    let my = key_off as i64;
                    out.push(WalkRow { key: Some(V::Text(k.clone())), value: jvalue_of(v),
                        jtype: jtype_of(v).into(), atom: jatom_of(v), id: my,
                        parent: child_parent, fullkey: fk.clone(), path: fullkey.to_string() });
                    match v {
                        J::Arr(_) | J::Obj(_) => walk_node(v, vo, None, Some(my), &fk, fullkey, true, false, out),
                        _ => {}
                    }
                } else {
                    out.push(WalkRow { key: Some(V::Text(k.clone())), value: jvalue_of(v),
                        jtype: jtype_of(v).into(), atom: jatom_of(v), id: key_off as i64,
                        parent: None, fullkey: fk, path: fullkey.to_string() });
                }
                co = vo + jsonb_size(v);
            }
        }
        _ => {}
    }
}

/// json_each (recursive=false) / json_tree (recursive=true) over `doc`, optionally
/// rooted at `root_path` ($ syntax). Ids are the node's byte offset in C's JSONB
/// encoding of the WHOLE document; parents reference the parent row's id.
pub fn walk(doc: &str, root_path: Option<&str>, recursive: bool) -> Result<Vec<WalkRow>, String> {
    let j = parse(doc)?;
    let (node, off, fullkey) = match root_path {
        None | Some("$") => (&j, 0usize, "$".to_string()),
        Some(p) => {
            let steps = path_steps(p);
            let mut cur = &j;
            let mut off = 0usize;
            let mut fk = "$".to_string();
            for s in &steps {
                match cur {
                    J::Obj(o) => {
                        let payload: usize = o.iter().map(|(k, v)| jsonb_size(&J::Str(k.clone())) + jsonb_size(v)).sum();
                        let mut co = off + jsonb_hdr(payload);
                        let mut found = false;
                        for (k, v) in o {
                            let vo = co + jsonb_size(&J::Str(k.clone()));
                            if k == s { cur = v; off = vo; fk = format!("{fk}.{k}"); found = true; break; }
                            co = vo + jsonb_size(v);
                        }
                        if !found { return Ok(Vec::new()); }
                    }
                    J::Arr(a) => {
                        let idx: usize = s.parse().map_err(|_| "bad path".to_string())?;
                        let mut co = off + jsonb_hdr(a.iter().map(jsonb_size).sum());
                        if idx >= a.len() { return Ok(Vec::new()); }
                        for v in a.iter().take(idx) { co += jsonb_size(v); }
                        cur = &a[idx]; off = co; fk = format!("{fk}[{idx}]");
                    }
                    _ => return Ok(Vec::new()),
                }
            }
            (cur, off, fk)
        }
    };
    let mut out = Vec::new();
    match node {
        J::Arr(_) | J::Obj(_) => {
            if recursive {
                // tree emits the container row itself (key NULL, path = fullkey)
                walk_node(node, off, None, None, &fullkey, &fullkey, true, true, &mut out);
            } else {
                walk_node(node, off, None, None, &fullkey, &fullkey, false, false, &mut out);
            }
        }
        scalar => {
            out.push(WalkRow { key: None, value: jvalue_of(scalar), jtype: jtype_of(scalar).into(),
                atom: jatom_of(scalar), id: off as i64, parent: None,
                fullkey: fullkey.clone(), path: fullkey });
        }
    }
    Ok(out)
}

fn v_to_j(v: &V) -> J {
    match v { V::Null => J::Null, V::Int(i) => J::Int(*i), V::Real(r) => J::Real(*r),
              V::Text(t) => parse(t).unwrap_or_else(|_| J::Str(t.clone())), V::Blob(_) => J::Null }
}

pub fn set(doc: &str, path: &str, val: &V, mode: &str) -> Result<String, String> {
    let mut j = parse(doc)?;
    let steps = path_steps(path);
    fn put(j: &mut J, steps: &[String], nv: J, mode: &str) {
        if steps.is_empty() { *j = nv; return; }
        let key = &steps[0];
        if steps.len() == 1 {
            match j {
                J::Obj(o) => {
                    let exists = o.iter().any(|(k, _)| k == key);
                    match (mode, exists) {
                        ("json_insert", true) => {}
                        ("json_replace", false) => {}
                        _ => { if let Some(e) = o.iter_mut().find(|(k, _)| k == key) { e.1 = nv; } else { o.push((key.clone(), nv)); } }
                    }
                }
                // run-53: array-index last step — an existing element overwrites
                // (set/replace), idx == len appends (set/insert), past-the-end is a NO-OP
                J::Arr(a) => {
                    if let Some(idx) = arr_idx(key, a.len()) {
                        let exists = idx < a.len();
                        match (mode, exists) {
                            ("json_insert", true) => {}
                            ("json_replace", false) => {}
                            (_, true) => a[idx] = nv,
                            (_, false) => { if idx == a.len() { a.push(nv); } }
                        }
                    }
                }
                _ => {}
            }
            return;
        }
        match j {
            J::Obj(o) => { if let Some(e) = o.iter_mut().find(|(k, _)| k == key) { put(&mut e.1, &steps[1..], nv, mode); } }
            J::Arr(a) => {
                let len = a.len();
                if let Some(idx) = arr_idx(key, len) {
                    if let Some(el) = a.get_mut(idx) { put(el, &steps[1..], nv, mode); }
                }
            }
            _ => {}
        }
    }
    put(&mut j, &steps, v_to_j(val), mode);
    Ok(serialize(&j))
}
pub fn remove(doc: &str, path: &str) -> Result<String, String> {
    let mut j = parse(doc)?;
    let steps = path_steps(path);
    fn rm(j: &mut J, steps: &[String]) {
        if steps.is_empty() { return; }
        let key = &steps[0];
        if steps.len() == 1 {
            match j {
                J::Obj(o) => { o.retain(|(k, _)| k != key); }
                // run-53: removing an array index shifts the remainder; OOB is a no-op
                J::Arr(a) => {
                    if let Some(idx) = arr_idx(key, a.len()) {
                        if idx < a.len() { a.remove(idx); }
                    }
                }
                _ => {}
            }
            return;
        }
        match j {
            J::Obj(o) => { if let Some(e) = o.iter_mut().find(|(k, _)| k == key) { rm(&mut e.1, &steps[1..]); } }
            J::Arr(a) => {
                let len = a.len();
                if let Some(idx) = arr_idx(key, len) {
                    if let Some(el) = a.get_mut(idx) { rm(el, &steps[1..]); }
                }
            }
            _ => {}
        }
    }
    rm(&mut j, &steps);
    Ok(serialize(&j))
}
pub fn patch(doc: &str, p: &str) -> Result<String, String> {
    let mut base = parse(doc)?; let patch = parse(p)?;
    fn merge(base: &mut J, patch: &J) {
        if let (J::Obj(bo), J::Obj(po)) = (&mut *base, patch) {
            for (k, v) in po {
                if matches!(v, J::Null) { bo.retain(|(bk, _)| bk != k); }
                else if let Some(e) = bo.iter_mut().find(|(bk, _)| bk == k) { merge(&mut e.1, v); }
                else { bo.push((k.clone(), v.clone())); }
            }
        } else { *base = patch.clone(); }
    }
    merge(&mut base, &patch);
    Ok(serialize(&base))
}
