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
            if let J::Obj(o) = j {
                let exists = o.iter().any(|(k, _)| k == key);
                match (mode, exists) {
                    ("json_insert", true) => {}
                    ("json_replace", false) => {}
                    _ => { if let Some(e) = o.iter_mut().find(|(k, _)| k == key) { e.1 = nv; } else { o.push((key.clone(), nv)); } }
                }
            }
            return;
        }
        if let J::Obj(o) = j { if let Some(e) = o.iter_mut().find(|(k, _)| k == key) { put(&mut e.1, &steps[1..], nv, mode); } }
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
        if steps.len() == 1 { if let J::Obj(o) = j { o.retain(|(k, _)| k != key); } return; }
        if let J::Obj(o) = j { if let Some(e) = o.iter_mut().find(|(k, _)| k == key) { rm(&mut e.1, &steps[1..]); } }
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
