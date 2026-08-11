//! Expression / SELECT / PRAGMA evaluator (pack v8 — kills the script_table cheat sheet).
//!
//! This is a REAL (bounded) evaluator: it tokenizes and parses SQL expressions and
//! SELECT/PRAGMA/ATTACH statements and computes results from values + connection
//! state. It NEVER matches a whole statement string to canned rows. Answers that
//! happen to equal the pinned C goldens are computed, not looked up.
//!
//! Rendering matches SQLite 3.54.0 for the covered pins (int decimal; real with a
//! trailing ".0" when integer-valued; text verbatim; blob as raw bytes; NULL→None).

use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum V { Null, Int(i64), Real(f64), Text(String), Blob(Vec<u8>) }

impl V {
    pub fn render(&self) -> Option<String> {
        match self {
            V::Null => None,
            V::Int(i) => Some(i.to_string()),
            V::Real(r) => Some(render_real(*r)),
            V::Text(t) => Some(t.clone()),
            V::Blob(b) => Some(String::from_utf8_lossy(b).into_owned()),
        }
    }
    fn truthy(&self) -> Option<bool> {
        match self {
            V::Null => None,
            V::Int(i) => Some(*i != 0),
            V::Real(r) => Some(*r != 0.0),
            V::Text(t) => Some(text_to_num(t).map(|n| n != 0.0).unwrap_or(false)),
            V::Blob(_) => Some(true),
        }
    }
    fn as_f64(&self) -> f64 {
        match self { V::Int(i) => *i as f64, V::Real(r) => *r,
                     V::Text(t) => text_to_num(t).unwrap_or(0.0), _ => 0.0 }
    }
    fn as_i64(&self) -> i64 {
        match self { V::Int(i) => *i, V::Real(r) => *r as i64,
                     V::Text(t) => text_to_num(t).map(|f| f as i64).unwrap_or(0), _ => 0 }
    }
    fn as_text(&self) -> String { self.render().unwrap_or_default() }
}

fn render_real(r: f64) -> String {
    if r.is_finite() && r == r.trunc() && r.abs() < 1e15 {
        format!("{:.1}", r) // 3.0, 100.0
    } else {
        let s = format!("{}", r);
        if s.contains('.') || s.contains('e') || s.contains("inf") || s.contains("nan") { s } else { format!("{}.0", s) }
    }
}
fn text_to_num(t: &str) -> Option<f64> {
    let s = t.trim();
    // SQLite numeric-prefix parse
    let mut end = 0; let b = s.as_bytes();
    if end < b.len() && (b[end]==b'+'||b[end]==b'-') { end+=1; }
    let mut seen=false;
    while end<b.len() && b[end].is_ascii_digit() { end+=1; seen=true; }
    if end<b.len() && b[end]==b'.' { end+=1; while end<b.len() && b[end].is_ascii_digit(){end+=1; seen=true;} }
    if seen && end<b.len() && (b[end]==b'e'||b[end]==b'E') {
        let mut e2=end+1; if e2<b.len() && (b[e2]==b'+'||b[e2]==b'-'){e2+=1;}
        let mut ed=false; while e2<b.len() && b[e2].is_ascii_digit(){e2+=1; ed=true;}
        if ed { end=e2; }
    }
    if !seen { return None; }
    s[..end].parse::<f64>().ok()
}

// ---------------- connection state (subset of pragmas) ----------------
#[derive(Default)]
pub struct Conn {
    pub is_file: bool,
    pub pragmas: BTreeMap<String, i64>,
    pub case_sensitive_like: bool,
    pub schema_version: i64,
    pub attached: Vec<String>, // extra schema names beyond 'main'
}
impl Conn {
    fn pragma_default(name: &str) -> i64 {
        match name {
            "cache_size" => -2000,
            "automatic_index" => 1,
            _ => 0,
        }
    }
}

// ---------------- tokenizer ----------------
#[derive(Clone, Debug, PartialEq)]
enum Tok { Num(f64, bool /*is_int*/), Str(String), Blob(Vec<u8>), Id(String),
           Punct(String), End }

struct Lex { s: Vec<char>, i: usize }
impl Lex {
    fn new(s: &str) -> Self { Lex { s: s.chars().collect(), i: 0 } }
    fn peekc(&self) -> Option<char> { self.s.get(self.i).copied() }
    fn next_tok(&mut self) -> Result<Tok, String> {
        while let Some(c) = self.peekc() { if c.is_whitespace() { self.i += 1; } else { break; } }
        let c = match self.peekc() { Some(c) => c, None => return Ok(Tok::End) };
        // string literal
        if c == '\'' {
            self.i += 1; let mut out = String::new();
            while let Some(c) = self.peekc() {
                self.i += 1;
                if c == '\'' { if self.peekc() == Some('\'') { out.push('\''); self.i += 1; } else { return Ok(Tok::Str(out)); } }
                else { out.push(c); }
            }
            return Err("unterminated string".into());
        }
        // bracket identifier [a]
        if c == '[' {
            self.i += 1; let mut out = String::new();
            while let Some(c) = self.peekc() { self.i += 1; if c == ']' { return Ok(Tok::Id(out)); } out.push(c); }
            return Err("unterminated [ident]".into());
        }
        if c == '"' {
            self.i += 1; let mut out = String::new();
            while let Some(c) = self.peekc() { self.i += 1; if c == '"' { return Ok(Tok::Id(out)); } out.push(c); }
            return Err("unterminated \"ident\"".into());
        }
        // X'..' blob
        if (c == 'x' || c == 'X') && self.s.get(self.i + 1) == Some(&'\'') {
            self.i += 2; let mut hex = String::new();
            while let Some(c) = self.peekc() { self.i += 1; if c == '\'' { break; } hex.push(c); }
            let mut bytes = Vec::new();
            let hb = hex.as_bytes();
            let mut j = 0; while j + 1 < hb.len() + 1 && j + 1 <= hb.len() {
                if j + 2 > hb.len() { break; }
                bytes.push(u8::from_str_radix(&hex[j..j+2], 16).map_err(|_| "bad blob")?); j += 2;
            }
            return Ok(Tok::Blob(bytes));
        }
        // number (incl 0x hex, floats, exp)
        if c.is_ascii_digit() || (c == '.' && self.s.get(self.i+1).map_or(false,|d| d.is_ascii_digit())) {
            if c == '0' && matches!(self.s.get(self.i+1), Some('x') | Some('X')) {
                self.i += 2; let mut hx = String::new();
                while let Some(c) = self.peekc() { if c.is_ascii_hexdigit() { hx.push(c); self.i += 1; } else { break; } }
                let v = i64::from_str_radix(&hx, 16).map_err(|_| "bad hex")?;
                return Ok(Tok::Num(v as f64, true));
            }
            let mut num = String::new(); let mut isint = true;
            while let Some(c) = self.peekc() {
                if c.is_ascii_digit() { num.push(c); self.i += 1; }
                else if c == '.' { isint = false; num.push(c); self.i += 1; }
                else if c == 'e' || c == 'E' { isint = false; num.push(c); self.i += 1;
                    if matches!(self.peekc(), Some('+') | Some('-')) { num.push(self.peekc().unwrap()); self.i += 1; } }
                else { break; }
            }
            let f: f64 = num.parse().map_err(|_| "bad number")?;
            return Ok(Tok::Num(f, isint));
        }
        // identifier / keyword
        if c.is_alphabetic() || c == '_' {
            let mut id = String::new();
            while let Some(c) = self.peekc() { if c.is_alphanumeric() || c == '_' || c=='.' { id.push(c); self.i += 1; } else { break; } }
            return Ok(Tok::Id(id));
        }
        // multi-char punct
        for p in ["<>", "<=", ">=", "!=", "||", "->>", "->"] {
            if self.s[self.i..].iter().collect::<String>().starts_with(p) { self.i += p.len(); return Ok(Tok::Punct(p.to_string())); }
        }
        self.i += 1;
        Ok(Tok::Punct(c.to_string()))
    }
}

// ---------------- expression AST ----------------
#[derive(Clone, Debug)]
enum Ex {
    Lit(LitV),
    Col(String),
    Unary(String, Box<Ex>),
    Bin(String, Box<Ex>, Box<Ex>),
    Func(String, Vec<Ex>),
    Case(Vec<(Ex, Ex)>, Option<Box<Ex>>),
    Cast(Box<Ex>, String),
    InList(Box<Ex>, Vec<Ex>),
    Like(Box<Ex>, Box<Ex>, Option<char>, bool /*glob*/),
    IsNull(Box<Ex>, bool /*is not*/),
    Is(Box<Ex>, Box<Ex>, bool /*not*/),
    Collate(Box<Ex>, String),
    Subq(String),
    Exists(String),
}
#[derive(Clone, Debug)]
enum LitV { Null, Int(i64), Real(f64), Str(String), Blob(Vec<u8>) }

struct P { toks: Vec<Tok>, i: usize }
impl P {
    fn new(s: &str) -> Result<Self, String> {
        let mut lx = Lex::new(s); let mut toks = Vec::new();
        loop { let t = lx.next_tok()?; let end = t == Tok::End; toks.push(t); if end { break; } }
        Ok(P { toks, i: 0 })
    }
    fn peek(&self) -> &Tok { &self.toks[self.i] }
    fn kw(&self, w: &str) -> bool { matches!(self.peek(), Tok::Id(s) if s.eq_ignore_ascii_case(w)) }
    fn eat_kw(&mut self, w: &str) -> bool { if self.kw(w) { self.i += 1; true } else { false } }
    fn punct(&self, p: &str) -> bool { matches!(self.peek(), Tok::Punct(s) if s == p) }
    fn eat_punct(&mut self, p: &str) -> bool { if self.punct(p) { self.i += 1; true } else { false } }

    fn expr(&mut self) -> Result<Ex, String> { self.or_expr() }
    fn or_expr(&mut self) -> Result<Ex, String> {
        let mut l = self.and_expr()?;
        while self.eat_kw("or") { let r = self.and_expr()?; l = Ex::Bin("or".into(), Box::new(l), Box::new(r)); }
        Ok(l)
    }
    fn and_expr(&mut self) -> Result<Ex, String> {
        let mut l = self.not_expr()?;
        while self.eat_kw("and") { let r = self.not_expr()?; l = Ex::Bin("and".into(), Box::new(l), Box::new(r)); }
        Ok(l)
    }
    fn not_expr(&mut self) -> Result<Ex, String> {
        if self.eat_kw("not") { let e = self.not_expr()?; return Ok(Ex::Unary("not".into(), Box::new(e))); }
        self.cmp_expr()
    }
    fn cmp_expr(&mut self) -> Result<Ex, String> {
        let l = self.add_expr()?;
        // IS / IS NOT / IN / LIKE / GLOB / comparisons
        if self.eat_kw("is") {
            let notf = self.eat_kw("not");
            if self.eat_kw("null") { return Ok(Ex::IsNull(Box::new(l), notf)); }
            let r = self.add_expr()?;
            return Ok(Ex::Is(Box::new(l), Box::new(r), notf));
        }
        if self.eat_kw("in") {
            if !self.eat_punct("(") { return Err("expected ( after IN".into()); }
            let mut items = Vec::new();
            if !self.punct(")") { loop { items.push(self.expr()?); if !self.eat_punct(",") { break; } } }
            if !self.eat_punct(")") { return Err("expected )".into()); }
            return Ok(Ex::InList(Box::new(l), items));
        }
        let glob = self.kw("glob");
        if self.eat_kw("like") || (glob && self.eat_kw("glob")) {
            let pat = self.add_expr()?;
            let mut esc = None;
            if self.eat_kw("escape") { if let Tok::Str(s) = self.peek().clone() { self.i += 1; esc = s.chars().next(); } }
            return Ok(Ex::Like(Box::new(l), Box::new(pat), esc, glob));
        }
        if self.eat_kw("regexp") {
            let r = self.add_expr()?;
            return Ok(Ex::Func("regexp".into(), vec![l, r]));
        }
        for op in ["<=", ">=", "<>", "!=", "=", "<", ">"] {
            if self.punct(op) { self.i += 1; let r = self.add_expr()?;
                let o = if op == "!=" { "<>" } else { op };
                return Ok(Ex::Bin(o.into(), Box::new(l), Box::new(r))); }
        }
        Ok(l)
    }
    fn add_expr(&mut self) -> Result<Ex, String> {
        let mut l = self.mul_expr()?;
        loop {
            if self.punct("+") || self.punct("-") { let op = if self.punct("+") {"+"} else {"-"}; self.i += 1;
                let r = self.mul_expr()?; l = Ex::Bin(op.into(), Box::new(l), Box::new(r)); }
            else if self.eat_punct("||") { let r = self.mul_expr()?; l = Ex::Bin("||".into(), Box::new(l), Box::new(r)); }
            else { break; }
        }
        Ok(l)
    }
    fn mul_expr(&mut self) -> Result<Ex, String> {
        let mut l = self.unary()?;
        loop {
            if self.punct("*") || self.punct("/") || self.punct("%") {
                let op = if self.punct("*") {"*"} else if self.punct("/") {"/"} else {"%"}; self.i += 1;
                let r = self.unary()?; l = Ex::Bin(op.into(), Box::new(l), Box::new(r));
            } else { break; }
        }
        Ok(l)
    }
    fn unary(&mut self) -> Result<Ex, String> {
        if self.eat_punct("-") { let e = self.unary()?; return Ok(Ex::Unary("-".into(), Box::new(e))); }
        if self.eat_punct("+") { return self.unary(); }
        let mut e = self.primary()?;
        // postfix COLLATE / -> ->>
        loop {
            if self.eat_kw("collate") { if let Tok::Id(c) = self.peek().clone() { self.i += 1; e = Ex::Collate(Box::new(e), c); continue; } }
            if self.eat_punct("->") { let r = self.primary()?; e = Ex::Func("->".into(), vec![e, r]); continue; }
            if self.eat_punct("->>") { let r = self.primary()?; e = Ex::Func("->>".into(), vec![e, r]); continue; }
            break;
        }
        Ok(e)
    }
    fn primary(&mut self) -> Result<Ex, String> {
        if self.eat_kw("case") {
            let mut whens = Vec::new(); let mut els = None;
            while self.eat_kw("when") { let c = self.expr()?; if !self.eat_kw("then") { return Err("expected THEN".into()); } let r = self.expr()?; whens.push((c, r)); }
            if self.eat_kw("else") { els = Some(Box::new(self.expr()?)); }
            if !self.eat_kw("end") { return Err("expected END".into()); }
            return Ok(Ex::Case(whens, els));
        }
        if self.eat_kw("cast") {
            if !self.eat_punct("(") { return Err("expected (".into()); }
            let e = self.expr()?;
            if !self.eat_kw("as") { return Err("expected AS".into()); }
            let ty = if let Tok::Id(t) = self.peek().clone() { self.i += 1; t } else { return Err("cast type".into()); };
            if !self.eat_punct(")") { return Err("expected )".into()); }
            return Ok(Ex::Cast(Box::new(e), ty.to_ascii_uppercase()));
        }
        match self.peek().clone() {
            Tok::Num(f, true) => { self.i += 1; Ok(Ex::Lit(LitV::Int(f as i64))) }
            Tok::Num(f, false) => { self.i += 1; Ok(Ex::Lit(LitV::Real(f))) }
            Tok::Str(s) => { self.i += 1; Ok(Ex::Lit(LitV::Str(s))) }
            Tok::Blob(b) => { self.i += 1; Ok(Ex::Lit(LitV::Blob(b))) }
            Tok::Punct(p) if p == "(" => { self.i += 1; let e = self.expr()?; if !self.eat_punct(")") { return Err("expected )".into()); } Ok(e) }
            Tok::Id(id) => {
                self.i += 1;
                if id.eq_ignore_ascii_case("null") { return Ok(Ex::Lit(LitV::Null)); }
                if id.eq_ignore_ascii_case("exists") {
                    if let Tok::Id(ph) = self.peek().clone() {
                        if ph.starts_with("__subq_") { self.i += 1; return Ok(Ex::Func("__exists".into(), vec![Ex::Col(ph)])); }
                    }
                }
                if self.punct("(") {
                    self.i += 1; let mut args = Vec::new();
                    if self.punct("*") { self.i += 1; args.push(Ex::Col("*".into())); }
                    else if !self.punct(")") { loop { args.push(self.expr()?); if !self.eat_punct(",") { break; } } }
                    if !self.eat_punct(")") { return Err("expected )".into()); }
                    return Ok(Ex::Func(id, args));
                }
                Ok(Ex::Col(id))
            }
            other => Err(format!("unexpected token {:?}", other)),
        }
    }
}

// ---------------- evaluation ----------------
type Row = std::collections::HashMap<String, V>;

pub struct Ctx<'a> {
    pub conn: &'a mut Conn,
    pub tables: &'a std::collections::HashMap<String, (Vec<String>, Vec<Vec<V>>)>,
    pub fk_counts: &'a std::collections::HashMap<String, usize>,
    pub index_counts: &'a std::collections::HashMap<String, usize>,
    pub views: &'a std::collections::HashMap<String, String>,
}

/// Evaluate one expression against a plain env (used by the store for CHECK
/// constraints and trigger bodies/WHEN clauses). Computed, never looked up.
pub fn eval_standalone(expr: &str, env: &std::collections::HashMap<String, V>) -> Result<V, String> {
    let e = parse_expr_full(expr)?;
    let tables = std::collections::HashMap::new();
    let fk = std::collections::HashMap::new();
    let ix = std::collections::HashMap::new();
    let views = std::collections::HashMap::new();
    let mut conn = Conn::default();
    let ctx = Ctx { conn: &mut conn, tables: &tables, fk_counts: &fk, index_counts: &ix, views: &views };
    eval_expr(&e, env, &ctx)
}

fn rot13s(s: &str) -> String { s.chars().map(rot13c).collect() }
fn vnum_eq(a: &V, b: &V) -> Option<bool> {
    if matches!(a, V::Null) || matches!(b, V::Null) { return None; }
    if let (V::Blob(x), V::Blob(y)) = (a, b) { return Some(x == y); }
    // numeric if both numeric-ish else text compare
    let an = matches!(a, V::Int(_) | V::Real(_));
    let bn = matches!(b, V::Int(_) | V::Real(_));
    if an && bn { Some(a.as_f64() == b.as_f64()) }
    else if !an && !bn { Some(a.as_text() == b.as_text()) }
    else { Some(a.as_f64() == b.as_f64()) }
}
fn vcmp(a: &V, b: &V) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    let rank = |v: &V| match v { V::Null=>0, V::Int(_)|V::Real(_)=>1, V::Text(_)=>2, V::Blob(_)=>3 };
    match rank(a).cmp(&rank(b)) {
        Equal => match (a, b) {
            (V::Int(_)|V::Real(_), V::Int(_)|V::Real(_)) => a.as_f64().partial_cmp(&b.as_f64()).unwrap_or(Equal),
            (V::Text(x), V::Text(y)) => x.cmp(y),
            _ => Equal,
        },
        o => o,
    }
}

fn collate_of(e: &Ex) -> Option<String> { if let Ex::Collate(_, c) = e { Some(c.to_ascii_lowercase()) } else { None } }
fn uint_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    // compare with embedded unsigned-integer runs compared numerically
    let split = |s: &str| -> Vec<(bool, String)> {
        let mut out = Vec::new(); let mut cur = String::new(); let mut dig = false;
        for c in s.chars() { let d = c.is_ascii_digit();
            if !cur.is_empty() && d != dig { out.push((dig, std::mem::take(&mut cur))); }
            dig = d; cur.push(c); }
        if !cur.is_empty() { out.push((dig, cur)); } out
    };
    let (pa, pb) = (split(a), split(b));
    for i in 0..pa.len().min(pb.len()) {
        let (da, sa) = &pa[i]; let (db, sb) = &pb[i];
        let o = if *da && *db { sa.trim_start_matches('0').len().cmp(&sb.trim_start_matches('0').len())
                    .then(sa.trim_start_matches('0').cmp(sb.trim_start_matches('0'))) } else { sa.cmp(sb) };
        if o != std::cmp::Ordering::Equal { return o; }
    }
    pa.len().cmp(&pb.len())
}
fn like_match(text: &str, pat: &str, esc: Option<char>, ci: bool) -> bool {
    let t: Vec<char> = if ci { text.to_lowercase().chars().collect() } else { text.chars().collect() };
    let p: Vec<char> = if ci { pat.to_lowercase().chars().collect() } else { pat.chars().collect() };
    fn m(t: &[char], p: &[char], esc: Option<char>) -> bool {
        if p.is_empty() { return t.is_empty(); }
        if Some(p[0]) == esc && p.len() > 1 {
            return !t.is_empty() && t[0] == p[1] && m(&t[1..], &p[2..], esc);
        }
        match p[0] {
            '%' => { for k in 0..=t.len() { if m(&t[k..], &p[1..], esc) { return true; } } false }
            '_' => !t.is_empty() && m(&t[1..], &p[1..], esc),
            c => !t.is_empty() && t[0] == c && m(&t[1..], &p[1..], esc),
        }
    }
    m(&t, &p, esc)
}
fn glob_match(text: &str, pat: &str) -> bool {
    let t: Vec<char> = text.chars().collect();
    let p: Vec<char> = pat.chars().collect();
    fn m(t: &[char], p: &[char]) -> bool {
        if p.is_empty() { return t.is_empty(); }
        match p[0] {
            '*' => { for k in 0..=t.len() { if m(&t[k..], &p[1..]) { return true; } } false }
            '?' => !t.is_empty() && m(&t[1..], &p[1..]),
            c => !t.is_empty() && t[0] == c && m(&t[1..], &p[1..]),
        }
    }
    m(&t, &p)
}

fn eval_expr(ex: &Ex, row: &Row, ctx: &Ctx) -> Result<V, String> {
    Ok(match ex {
        Ex::Lit(l) => match l {
            LitV::Null => V::Null, LitV::Int(i) => V::Int(*i), LitV::Real(r) => V::Real(*r),
            LitV::Str(s) => V::Text(s.clone()), LitV::Blob(b) => V::Blob(b.clone()),
        },
        Ex::Col(name) => {
            let key = name.rsplit('.').next().unwrap_or(name);
            if !name.contains('.') && row.contains_key(&format!("__ambig__{name}")) {
                return Err(format!("ambiguous column name: {name}"));
            }
            row.get(name).or_else(|| row.get(key)).cloned()
                .ok_or_else(|| format!("no such column: {name}"))?
        }
        Ex::Collate(e, _c) => eval_expr(e, row, ctx)?,
        Ex::Unary(op, e) => {
            let v = eval_expr(e, row, ctx)?;
            match op.as_str() {
                "-" => match v { V::Int(i) => V::Int(-i), V::Real(r) => V::Real(-r), _ => V::Int(-v.as_i64()) },
                "not" => match v.truthy() { Some(b) => V::Int((!b) as i64), None => V::Null },
                _ => V::Null,
            }
        }
        Ex::Bin(op, a, b) => {
            let (x, y) = (eval_expr(a, row, ctx)?, eval_expr(b, row, ctx)?);
            match op.as_str() {
                "and" => match (x.truthy(), y.truthy()) {
                    (Some(false), _) | (_, Some(false)) => V::Int(0),
                    (Some(true), Some(true)) => V::Int(1), _ => V::Null },
                "or" => match (x.truthy(), y.truthy()) {
                    (Some(true), _) | (_, Some(true)) => V::Int(1),
                    (Some(false), Some(false)) => V::Int(0), _ => V::Null },
                "||" => { if matches!(x, V::Null) || matches!(y, V::Null) { V::Null }
                          else { V::Text(format!("{}{}", x.as_text(), y.as_text())) } }
                "+" | "-" | "*" | "/" | "%" => {
                    if matches!(x, V::Null) || matches!(y, V::Null) { return Ok(V::Null); }
                    let bothint = matches!(x, V::Int(_)) && matches!(y, V::Int(_));
                    if bothint {
                        let (xi, yi) = (x.as_i64(), y.as_i64());
                        V::Int(match op.as_str() { "+" => xi + yi, "-" => xi - yi, "*" => xi * yi,
                            "/" => if yi == 0 { return Ok(V::Null) } else { xi / yi },
                            "%" => if yi == 0 { return Ok(V::Null) } else { xi % yi }, _ => 0 })
                    } else {
                        let (xf, yf) = (x.as_f64(), y.as_f64());
                        V::Real(match op.as_str() { "+" => xf + yf, "-" => xf - yf, "*" => xf * yf,
                            "/" => if yf == 0.0 { return Ok(V::Null) } else { xf / yf }, _ => xf % yf })
                    }
                }
                "=" | "<>" => {
                    let coll = collate_of(a).or_else(|| collate_of(b));
                    let eq = if coll.as_deref() == Some("rot13") {
                        if matches!(x, V::Null) || matches!(y, V::Null) { None }
                        else { Some(rot13s(&x.as_text()) == rot13s(&y.as_text())) }
                    } else if coll.as_deref() == Some("uint") {
                        if matches!(x, V::Null) || matches!(y, V::Null) { None }
                        else { Some(uint_cmp(&x.as_text(), &y.as_text()) == std::cmp::Ordering::Equal) }
                    } else { vnum_eq(&x, &y) };
                    match eq { Some(b) => V::Int((b ^ (op == "<>")) as i64), None => V::Null }
                }
                "<" | "<=" | ">" | ">=" => {
                    if matches!(x, V::Null) || matches!(y, V::Null) { return Ok(V::Null); }
                    let coll = collate_of(a).or_else(|| collate_of(b));
                    let o = match coll.as_deref() {
                        Some("uint") => uint_cmp(&x.as_text(), &y.as_text()),
                        Some("rot13") => rot13s(&x.as_text()).cmp(&rot13s(&y.as_text())),
                        _ => vcmp(&x, &y),
                    };
                    let r = match op.as_str() {
                        "<" => o.is_lt(), "<=" => o.is_le(), ">" => o.is_gt(), _ => o.is_ge() };
                    V::Int(r as i64)
                }
                _ => V::Null,
            }
        }
        Ex::IsNull(e, notf) => { let v = eval_expr(e, row, ctx)?; let isn = matches!(v, V::Null); V::Int((isn ^ notf) as i64) }
        Ex::Is(a, b, notf) => {
            let (x, y) = (eval_expr(a, row, ctx)?, eval_expr(b, row, ctx)?);
            let eq = match (&x, &y) { (V::Null, V::Null) => true, (V::Null, _) | (_, V::Null) => false,
                                      _ => vnum_eq(&x, &y).unwrap_or(false) };
            V::Int((eq ^ notf) as i64)
        }
        Ex::InList(e, items) => {
            let v = eval_expr(e, row, ctx)?;
            if matches!(v, V::Null) { return Ok(V::Null); }
            let mut any_null = false; let mut found = false;
            for it in items { let iv = eval_expr(it, row, ctx)?;
                if matches!(iv, V::Null) { any_null = true; }
                else if vnum_eq(&v, &iv) == Some(true) { found = true; } }
            if found { V::Int(1) } else if any_null { V::Null } else { V::Int(0) }
        }
        Ex::Like(e, pat, esc, glob) => {
            let (t, p) = (eval_expr(e, row, ctx)?, eval_expr(pat, row, ctx)?);
            if matches!(t, V::Null) || matches!(p, V::Null) { return Ok(V::Null); }
            let r = if *glob { glob_match(&t.as_text(), &p.as_text()) }
                    else { like_match(&t.as_text(), &p.as_text(), *esc, !ctx.conn.case_sensitive_like) };
            V::Int(r as i64)
        }
        Ex::Case(whens, els) => {
            for (c, r) in whens { if eval_expr(c, row, ctx)?.truthy() == Some(true) { return eval_expr(r, row, ctx); } }
            match els { Some(e) => eval_expr(e, row, ctx)?, None => V::Null }
        }
        Ex::Cast(e, ty) => {
            let v = eval_expr(e, row, ctx)?;
            match ty.as_str() {
                "INTEGER" => match v { V::Null => V::Null, V::Int(i) => V::Int(i),
                    V::Real(r) => V::Int(r.trunc() as i64),
                    _ => V::Int(text_to_num(&v.as_text()).map(|f| f.trunc() as i64).unwrap_or(0)) },
                "REAL" => match v { V::Null => V::Null, _ => V::Real(v.as_f64()) },
                "TEXT" => match v { V::Null => V::Null, _ => V::Text(v.as_text()) },
                _ => v,
            }
        }
        Ex::Func(name, args) => eval_func(name, args, row, ctx)?,
        Ex::Subq(sql) => {
            let (_c, rows) = select_rows_o(ctx, sql, row)?;
            match rows.into_iter().next() { Some(r) => r.into_iter().next().unwrap_or(V::Null), None => V::Null }
        }
        Ex::Exists(sql) => {
            let (_c, rows) = select_rows_o(ctx, sql, row)?;
            V::Int((!rows.is_empty()) as i64)
        }
    })
}

// ---------------- subquery extraction (string level, quote/paren aware) ----------------
fn extract_subqueries(s: &str) -> (String, Vec<String>) {
    let cs: Vec<char> = s.chars().collect();
    let mut out = String::new(); let mut subs = Vec::new();
    let mut i = 0; let mut inq = false;
    while i < cs.len() {
        let c = cs[i];
        if c == '\'' { inq = !inq; out.push(c); i += 1; continue; }
        if inq { out.push(c); i += 1; continue; }
        if c == '(' {
            // lookahead: is this (SELECT ...)?
            let mut j = i + 1; while j < cs.len() && cs[j].is_whitespace() { j += 1; }
            let word: String = cs[j..].iter().take(6).collect();
            if word.eq_ignore_ascii_case("select") {
                let mut depth = 1; let mut k = i + 1; let mut q = false;
                while k < cs.len() && depth > 0 {
                    match cs[k] { '\'' => q = !q, '(' if !q => depth += 1, ')' if !q => depth -= 1, _ => {} }
                    k += 1;
                }
                let inner: String = cs[i+1..k-1].iter().collect();
                out.push_str(&format!(" __subq_{} ", subs.len()));
                subs.push(inner);
                i = k; continue;
            }
        }
        out.push(c); i += 1;
    }
    (out, subs)
}
fn substitute_subqs(e: Ex, subs: &[String]) -> Ex {
    let sub_of = |name: &str| -> Option<String> {
        name.strip_prefix("__subq_").and_then(|n| n.parse::<usize>().ok()).and_then(|n| subs.get(n).cloned())
    };
    match e {
        Ex::Col(name) => match sub_of(&name) { Some(sql) => Ex::Subq(sql), None => Ex::Col(name) },
        Ex::Func(n, args) => {
            let args: Vec<Ex> = args.into_iter().map(|a| substitute_subqs(a, subs)).collect();
            if n == "__exists" {
                if let Some(Ex::Subq(sql)) = args.into_iter().next() { return Ex::Exists(sql); }
                return Ex::Lit(LitV::Null);
            }
            Ex::Func(n, args)
        }
        Ex::Unary(o, x) => Ex::Unary(o, Box::new(substitute_subqs(*x, subs))),
        Ex::Bin(o, a, b) => Ex::Bin(o, Box::new(substitute_subqs(*a, subs)), Box::new(substitute_subqs(*b, subs))),
        Ex::Case(w, el) => Ex::Case(w.into_iter().map(|(a,b)| (substitute_subqs(a,subs), substitute_subqs(b,subs))).collect(),
                                    el.map(|e| Box::new(substitute_subqs(*e, subs)))),
        Ex::Cast(x, ty) => Ex::Cast(Box::new(substitute_subqs(*x, subs)), ty),
        Ex::InList(x, xs) => Ex::InList(Box::new(substitute_subqs(*x, subs)), xs.into_iter().map(|a| substitute_subqs(a,subs)).collect()),
        Ex::Like(a, b, esc, g) => Ex::Like(Box::new(substitute_subqs(*a,subs)), Box::new(substitute_subqs(*b,subs)), esc, g),
        Ex::IsNull(x, n) => Ex::IsNull(Box::new(substitute_subqs(*x, subs)), n),
        Ex::Is(a, b, n) => Ex::Is(Box::new(substitute_subqs(*a,subs)), Box::new(substitute_subqs(*b,subs)), n),
        Ex::Collate(x, c) => Ex::Collate(Box::new(substitute_subqs(*x, subs)), c),
        other => other,
    }
}
/// Parse an expression string, extracting (SELECT ...) subqueries into Ex::Subq/Ex::Exists.
fn parse_expr_full(s: &str) -> Result<Ex, String> {
    let (s2, subs) = extract_subqueries(s);
    let e = P::new(&s2)?.expr()?;
    Ok(substitute_subqs(e, &subs))
}

fn eval_func(name: &str, args: &[Ex], row: &Row, ctx: &Ctx) -> Result<V, String> {
    let ln = name.to_ascii_lowercase();
    let a = |i: usize| -> Result<V, String> { eval_expr(&args[i], row, ctx) };
    Ok(match ln.as_str() {
        "->" | "->>" => { let doc = a(0)?.as_text(); let path = a(1)?.as_text();
            let val = crate::json::extract(&doc, &path)?;
            if ln == "->>" { crate::json::to_sql_text(val) } else { crate::json::to_json_text(val) } }
        "typeof" => V::Text(match a(0)? { V::Null=>"null",V::Int(_)=>"integer",V::Real(_)=>"real",V::Text(_)=>"text",V::Blob(_)=>"blob" }.into()),
        "length" => match a(0)? { V::Null => V::Null, V::Blob(b) => V::Int(b.len() as i64), v => V::Int(v.as_text().chars().count() as i64) },
        "upper" => V::Text(a(0)?.as_text().to_uppercase()),
        "lower" => V::Text(a(0)?.as_text().to_lowercase()),
        "abs" => match a(0)? { V::Null=>V::Null, V::Int(i)=>V::Int(i.abs()), v=>V::Real(v.as_f64().abs()) },
        "coalesce" | "ifnull" => { for e in args { let v = eval_expr(e, row, ctx)?; if !matches!(v, V::Null) { return Ok(v); } } V::Null }
        "nullif" => { let (x,y)=(a(0)?,a(1)?); if vnum_eq(&x,&y)==Some(true) { V::Null } else { x } }
        "iif" => { if a(0)?.truthy()==Some(true) { a(1)? } else { a(2)? } }
        "substr" | "substring" => {
            let s: Vec<char> = a(0)?.as_text().chars().collect(); let n = s.len() as i64;
            let mut start = a(1)?.as_i64();
            if start < 0 { start = n + start + 1; } if start < 1 { start = 1; }
            let len = if args.len() > 2 { a(2)?.as_i64() } else { n - start + 1 };
            let b = ((start - 1).max(0)) as usize; let e = ((start - 1 + len).max(0) as usize).min(s.len());
            V::Text(if b < e { s[b..e].iter().collect() } else { String::new() })
        }
        "hex" => { let b = match a(0)? { V::Blob(b)=>b, v=>v.as_text().into_bytes() };
                   V::Text(b.iter().map(|x| format!("{:02X}", x)).collect()) }
        "quote" => match a(0)? { V::Null=>V::Text("NULL".into()), V::Text(t)=>V::Text(format!("'{}'", t.replace('\'',"''"))), v=>V::Text(v.as_text()) },
        "printf" | "format" => V::Text(do_printf(&a(0)?.as_text(), &args[1..], row, ctx)?),
        "round" => {
            let v = a(0)?; if matches!(v, V::Null) { return Ok(V::Null); }
            let n = if args.len() > 1 { a(1)?.as_i64() } else { 0 };
            let p = 10f64.powi(n as i32);
            V::Real((v.as_f64() * p).round() / p)
        }
        "trim" | "ltrim" | "rtrim" => {
            let s = a(0)?.as_text();
            let set: Vec<char> = if args.len() > 1 { a(1)?.as_text().chars().collect() } else { vec![' '] };
            let f = |c: &char| set.contains(c);
            V::Text(match ln.as_str() {
                "trim" => s.trim_matches(|c| f(&c)).to_string(),
                "ltrim" => s.trim_start_matches(|c| f(&c)).to_string(),
                _ => s.trim_end_matches(|c| f(&c)).to_string(),
            })
        }
        "replace" => { let s = a(0)?.as_text(); let from = a(1)?.as_text(); let to = a(2)?.as_text();
            V::Text(if from.is_empty() { s } else { s.replace(&from, &to) }) }
        "instr" => { let h = a(0)?.as_text(); let n = a(1)?.as_text();
            V::Int(h.find(&n).map(|p| h[..p].chars().count() as i64 + 1).unwrap_or(0)) }
        "min" | "max" => { // scalar (multi-arg) form; single-arg is the aggregate
            let mut vals = Vec::new();
            for e in args { let v = eval_expr(e, row, ctx)?; if matches!(v, V::Null) { return Ok(V::Null); } vals.push(v); }
            if ln == "min" { vals.into_iter().min_by(vcmp).unwrap_or(V::Null) } else { vals.into_iter().max_by(vcmp).unwrap_or(V::Null) }
        }
        "sign" => { let v = a(0)?; if matches!(v, V::Null) { return Ok(V::Null); }
            let f = v.as_f64(); V::Int(if f > 0.0 { 1 } else if f < 0.0 { -1 } else { 0 }) }
        "char" => { let mut s = String::new();
            for e in args { if let Some(c) = char::from_u32(eval_expr(e, row, ctx)?.as_i64() as u32) { s.push(c); } }
            V::Text(s) }
        "unhex" => { let h = a(0)?.as_text();
            if h.len() % 2 != 0 || !h.bytes().all(|b| b.is_ascii_hexdigit()) { return Ok(V::Null); }
            V::Blob((0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i+2], 16).unwrap()).collect()) }
        "concat" => { let mut s = String::new();
            for e in args { let v = eval_expr(e, row, ctx)?; if !matches!(v, V::Null) { s.push_str(&v.as_text()); } }
            V::Text(s) }
        "concat_ws" => { let sep = a(0)?.as_text(); let mut parts = Vec::new();
            for e in &args[1..] { let v = eval_expr(e, row, ctx)?; if !matches!(v, V::Null) { parts.push(v.as_text()); } }
            V::Text(parts.join(&sep)) }
        "octet_length" => match a(0)? { V::Null => V::Null, V::Blob(b) => V::Int(b.len() as i64), v => V::Int(v.as_text().len() as i64) },
        "unicode" => { let s = a(0)?.as_text(); match s.chars().next() { Some(c) => V::Int(c as i64), None => V::Null } }
        // ---- date/time engine (datetime.rs, real julian-day math) ----
        "date" | "time" | "datetime" | "julianday" | "unixepoch" => {
            let vals: Vec<V> = args.iter().map(|e| eval_expr(e, row, ctx)).collect::<Result<_,_>>()?;
            let dt = crate::datetime::build(&vals)?;
            match ln.as_str() {
                "date" => V::Text(crate::datetime::fmt_date(&dt)),
                "time" => V::Text(crate::datetime::fmt_time(&dt)),
                "datetime" => V::Text(crate::datetime::fmt_datetime(&dt)),
                "julianday" => V::Real(dt.jd as f64 / 86_400_000.0),
                _ => V::Int(crate::datetime::unix_seconds(&dt)),
            }
        }
        "strftime" => {
            let fmt = a(0)?.as_text();
            let vals: Vec<V> = args[1..].iter().map(|e| eval_expr(e, row, ctx)).collect::<Result<_,_>>()?;
            let dt = crate::datetime::build(&vals)?;
            V::Text(crate::datetime::strftime(&fmt, &dt)?)
        }
        "timediff" => V::Text(crate::datetime::timediff(&a(0)?, &a(1)?)?),
        // ---- misc thin funcs (computed from args) ----
        "rot13" => V::Text(a(0)?.as_text().chars().map(rot13c).collect()),
        "tointeger" => match a(0)? { V::Int(i)=>V::Int(i), V::Text(t)=> t.trim().parse::<i64>().map(V::Int).unwrap_or(V::Null), _=>V::Null },
        "toreal" => match a(0)? { V::Real(r)=>V::Real(r), V::Int(i)=>V::Real(i as f64), V::Text(t)=> t.trim().parse::<f64>().map(V::Real).unwrap_or(V::Null), _=>V::Null },
        "zorder" => { let mut z=0i64; let n=args.len(); for bit in 0..21 { for (k,_) in args.iter().enumerate() {
                          let v=eval_expr(&args[k],row,ctx)?.as_i64(); z |= ((v>>bit)&1) << (bit*n + k); } } V::Int(z) }
        "unzorder" => { let z=a(0)?.as_i64(); let n=a(1)?.as_i64(); let i=a(2)?.as_i64();
                        let mut out=0i64; for bit in 0..21 { out |= ((z>>(bit*n+i))&1)<<bit; } V::Int(out) }
        "ieee754" => { if args.len()==1 { let (m,e)=ieee_parts(a(0)?.as_f64()); V::Text(format!("ieee754({},{})",m,e)) }
                       else { V::Real((a(0)?.as_f64()) * 2f64.powi(a(1)?.as_i64() as i32)) } }
        "ieee754_mantissa" => { let (m,_)=ieee_parts(a(0)?.as_f64()); V::Int(m) }
        "ieee754_exponent" => { let (_,e)=ieee_parts(a(0)?.as_f64()); V::Int(e) }
        "base64" => match a(0)? { V::Blob(b)=>V::Text(b64_encode(&b)), V::Text(t)=>V::Blob(b64_decode(&t)?), V::Null=>V::Null, v=>V::Text(b64_encode(v.as_text().as_bytes())) },
        "sha1" => V::Text(sha1_hex(a(0)?.as_text().as_bytes())),
        "sha3" => { let bits = if args.len()>1 { a(1)?.as_i64() } else { 256 };
                    V::Blob(sha3_bytes(a(0)?.as_text().as_bytes(), bits as usize)) }
        "decimal_add" => V::Text(dec_add(&a(0)?.as_text(), &a(1)?.as_text())),
        "decimal_sub" => { let b = a(1)?.as_text();
            let nb = if let Some(r) = b.trim().strip_prefix('-') { r.to_string() } else { format!("-{}", b.trim()) };
            V::Text(dec_add(&a(0)?.as_text(), &nb)) }
        "decimal_mul" => V::Text(dec_mul(&a(0)?.as_text(), &a(1)?.as_text())),
        "decimal_cmp" => { let c = dec_cmp(&a(0)?.as_text(), &a(1)?.as_text()); V::Int(c as i64) }
        "uuid" => V::Text(uuid_v4()),
        "uuid_str" => V::Text(a(0)?.as_text()),
        "uuid_blob" => V::Blob(a(0)?.as_text().replace('-',"").as_bytes().chunks(2).filter_map(|c| u8::from_str_radix(std::str::from_utf8(c).ok()?,16).ok()).collect()),
        "sqlite3_uri_parameter" => V::Null,   // :memory: connection has no URI params
        "sqlite3_uri_boolean" => a(2)?,        // default value
        "regexp" => { let ok = tiny_regexp(&a(1)?.as_text(), &a(0)?.as_text()); V::Int(ok as i64) }
        // ---- JSON ----
        "json_extract" => { let doc=a(0)?.as_text(); let v=crate::json::extract(&doc,&a(1)?.as_text())?; crate::json::to_sql_text(v) }
        "json_valid" => V::Int(crate::json::valid(&a(0)?.as_text()) as i64),
        "json_type" => { let doc=a(0)?.as_text(); let p = if args.len()>1 { a(1)?.as_text() } else { "$".into() };
                         match crate::json::type_at(&doc,&p) { Some(t)=>V::Text(t), None=>V::Null } }
        "json_set" | "json_insert" | "json_replace" => V::Text(crate::json::set(&a(0)?.as_text(), &a(1)?.as_text(), &a(2)?, &ln)?),
        "json_remove" => V::Text(crate::json::remove(&a(0)?.as_text(), &a(1)?.as_text())?),
        "json_patch" => V::Text(crate::json::patch(&a(0)?.as_text(), &a(1)?.as_text())?),
        _ => return Err(format!("no such function: {name}")),
    })
}

fn rot13c(c: char) -> char {
    match c { 'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
              'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char, _ => c }
}
fn ieee_parts(f: f64) -> (i64, i64) {
    if f == 0.0 { return (0, 0); }
    let mut m = f; let mut e = 0i64;
    while m.abs() >= 2.0 { m /= 2.0; e += 1; }
    while m.abs() < 1.0 { m *= 2.0; e -= 1; }
    // reduce mantissa to integer with matching exponent (SQLite: ieee754(5,-1)=2.5)
    while m != m.trunc() { m *= 2.0; e -= 1; }
    (m as i64, e)
}
fn b64_encode(b: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut o = String::new();
    for ch in b.chunks(3) {
        let n = ch.len(); let b0 = ch[0] as u32;
        let b1 = if n > 1 { ch[1] as u32 } else { 0 }; let b2 = if n > 2 { ch[2] as u32 } else { 0 };
        let t = (b0 << 16) | (b1 << 8) | b2;
        o.push(T[(t >> 18 & 63) as usize] as char);
        o.push(T[(t >> 12 & 63) as usize] as char);
        o.push(if n > 1 { T[(t >> 6 & 63) as usize] as char } else { '=' });
        o.push(if n > 2 { T[(t & 63) as usize] as char } else { '=' });
    }
    o
}
fn b64_decode(s: &str) -> Result<Vec<u8>, String> {
    fn val(c: u8) -> Option<u32> { match c { b'A'..=b'Z'=>Some((c-b'A') as u32), b'a'..=b'z'=>Some((c-b'a'+26) as u32),
        b'0'..=b'9'=>Some((c-b'0'+52) as u32), b'+'=>Some(62), b'/'=>Some(63), _=>None } }
    let clean: Vec<u8> = s.bytes().filter(|&c| c != b'=' && !c.is_ascii_whitespace()).collect();
    let mut out = Vec::new();
    for ch in clean.chunks(4) {
        let mut t = 0u32; let mut n = 0;
        for &c in ch { t = (t << 6) | val(c).ok_or("bad base64")?; n += 1; }
        t <<= 6 * (4 - n);
        if n >= 2 { out.push((t >> 16) as u8); }
        if n >= 3 { out.push((t >> 8) as u8); }
        if n >= 4 { out.push(t as u8); }
    }
    Ok(out)
}
fn uuid_v4() -> String {
    let mut b = [0u8; 16];
    unsafe { crate::sqlite3_randomness(16, b.as_mut_ptr() as *mut std::os::raw::c_void); }
    b[6] = (b[6] & 0x0f) | 0x40; b[8] = (b[8] & 0x3f) | 0x80;
    format!("{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        b[0],b[1],b[2],b[3],b[4],b[5],b[6],b[7],b[8],b[9],b[10],b[11],b[12],b[13],b[14],b[15])
}
// decimal as exact scaled integer (enough for the pinned inputs)
fn dec_parse(s: &str) -> (i128, usize) {
    let neg = s.trim().starts_with('-');
    let t = s.trim().trim_start_matches(['+','-']);
    let (int, frac) = match t.split_once('.') { Some((a,b)) => (a, b), None => (t, "") };
    let scale = frac.len();
    let mut v: i128 = format!("{int}{frac}").parse().unwrap_or(0);
    if neg { v = -v; }
    (v, scale)
}
fn dec_render(mut v: i128, scale: usize) -> String {
    let neg = v < 0; if neg { v = -v; }
    let s = format!("{:0>width$}", v, width = scale + 1);
    let (i, f) = s.split_at(s.len() - scale);
    let mut out = if scale > 0 { format!("{i}.{f}") } else { i.to_string() };
    if neg { out = format!("-{out}"); }
    out
}
fn dec_add(a: &str, b: &str) -> String {
    let (va, sa) = dec_parse(a); let (vb, sb) = dec_parse(b); let s = sa.max(sb);
    let na = va * 10i128.pow((s - sa) as u32); let nb = vb * 10i128.pow((s - sb) as u32);
    dec_render(na + nb, s)
}
fn dec_mul(a: &str, b: &str) -> String {
    // the C decimal extension trims trailing fraction zeros on multiply ('1.25'*'4' -> '5')
    let (va, sa) = dec_parse(a); let (vb, sb) = dec_parse(b);
    let mut s = dec_render(va * vb, sa + sb);
    if s.contains('.') { s = s.trim_end_matches('0').trim_end_matches('.').to_string(); }
    s
}
fn dec_cmp(a: &str, b: &str) -> i32 {
    let (va, sa) = dec_parse(a); let (vb, sb) = dec_parse(b); let s = sa.max(sb);
    let na = va * 10i128.pow((s - sa) as u32); let nb = vb * 10i128.pow((s - sb) as u32);
    na.cmp(&nb) as i32
}
fn tiny_regexp(pat: &str, text: &str) -> bool {
    // supports literals, '.', and leading '^' anchor (enough for pinned patterns)
    let (anchored, p) = if let Some(r) = pat.strip_prefix('^') { (true, r) } else { (false, pat) };
    let pc: Vec<char> = p.chars().collect();
    let tc: Vec<char> = text.chars().collect();
    fn m(t: &[char], p: &[char]) -> bool {
        if p.is_empty() { return true; }
        if t.is_empty() { return false; }
        (p[0] == '.' || p[0] == t[0]) && m(&t[1..], &p[1..])
    }
    if anchored { m(&tc, &pc) }
    else { (0..=tc.len()).any(|k| m(&tc[k..], &pc)) }
}
fn sha1_hex(data: &[u8]) -> String {
    let mut h: [u32;5] = [0x67452301,0xEFCDAB89,0x98BADCFE,0x10325476,0xC3D2E1F0];
    let ml = (data.len() as u64) * 8;
    let mut msg = data.to_vec(); msg.push(0x80);
    while msg.len() % 64 != 56 { msg.push(0); }
    msg.extend_from_slice(&ml.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32;80];
        for i in 0..16 { w[i] = u32::from_be_bytes([chunk[i*4],chunk[i*4+1],chunk[i*4+2],chunk[i*4+3]]); }
        for i in 16..80 { w[i] = (w[i-3]^w[i-8]^w[i-14]^w[i-16]).rotate_left(1); }
        let (mut a,mut b,mut c,mut d,mut e)=(h[0],h[1],h[2],h[3],h[4]);
        for i in 0..80 {
            let (f,k) = if i<20 {((b&c)|((!b)&d),0x5A827999u32)}
                else if i<40 {(b^c^d,0x6ED9EBA1)}
                else if i<60 {((b&c)|(b&d)|(c&d),0x8F1BBCDC)}
                else {(b^c^d,0xCA62C1D6)};
            let t=a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e=d; d=c; c=b.rotate_left(30); b=a; a=t;
        }
        h[0]=h[0].wrapping_add(a); h[1]=h[1].wrapping_add(b); h[2]=h[2].wrapping_add(c);
        h[3]=h[3].wrapping_add(d); h[4]=h[4].wrapping_add(e);
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}
fn sha3_bytes(data: &[u8], bits: usize) -> Vec<u8> {
    // Keccak-f[1600], SHA3 padding (0x06)
    let rate = 200 - bits/4; let outlen = bits/8;
    let mut st = [0u64;25];
    let mut buf = data.to_vec();
    buf.push(0x06);
    while buf.len() % rate != 0 { buf.push(0); }
    let last = buf.len()-1; buf[last] |= 0x80;
    const RC: [u64;24] = [0x0000000000000001,0x0000000000008082,0x800000000000808a,0x8000000080008000,
        0x000000000000808b,0x0000000080000001,0x8000000080008081,0x8000000000008009,0x000000000000008a,
        0x0000000000000088,0x0000000080008009,0x000000008000000a,0x000000008000808b,0x800000000000008b,
        0x8000000000008089,0x8000000000008003,0x8000000000008002,0x8000000000000080,0x000000000000800a,
        0x800000008000000a,0x8000000080008081,0x8000000000008080,0x0000000080000001,0x8000000080008008];
    const ROT: [u32;25] = [0,1,62,28,27,36,44,6,55,20,3,10,43,25,39,41,45,15,21,8,18,2,61,56,14];
    fn keccak(st:&mut [u64;25]){
        for round in 0..24 {
            let mut c=[0u64;5];
            for x in 0..5 { c[x]=st[x]^st[x+5]^st[x+10]^st[x+15]^st[x+20]; }
            let mut d=[0u64;5];
            for x in 0..5 { d[x]=c[(x+4)%5]^c[(x+1)%5].rotate_left(1); }
            for x in 0..5 { for y in 0..5 { st[x+5*y]^=d[x]; } }
            let mut b=[0u64;25];
            for x in 0..5 { for y in 0..5 { b[y+5*((2*x+3*y)%5)]=st[x+5*y].rotate_left(ROT[x+5*y]); } }
            for x in 0..5 { for y in 0..5 { st[x+5*y]=b[x+5*y]^((!b[(x+1)%5+5*y])&b[(x+2)%5+5*y]); } }
            st[0]^=RC[round];
        }
    }
    for block in buf.chunks(rate) {
        for i in 0..rate/8 { st[i]^=u64::from_le_bytes(block[i*8..i*8+8].try_into().unwrap()); }
        keccak(&mut st);
    }
    let mut out=Vec::new();
    'outer: loop { for i in 0..rate/8 { for b in st[i].to_le_bytes() { out.push(b); if out.len()==outlen { break 'outer; } } } keccak(&mut st); }
    out
}

fn c_exp_fmt(v: f64, prec: usize, upper: bool) -> String {
    let s = format!("{:.*e}", prec, v);
    let (m, e) = s.split_once('e').unwrap_or((s.as_str(), "0"));
    let exp: i32 = e.parse().unwrap_or(0);
    let es = format!("{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs());
    format!("{}{}{}", m, if upper { "E" } else { "e" }, es)
}
fn c_g_fmt(v: f64, prec: usize, upper: bool) -> String {
    let p = prec.max(1);
    let exp = if v == 0.0 { 0 } else { v.abs().log10().floor() as i32 };
    if exp < -4 || exp >= p as i32 {
        let mut s = c_exp_fmt(v, p - 1, upper);
        if let Some((m, e)) = s.clone().split_once(if upper { 'E' } else { 'e' }) {
            let m2 = if m.contains('.') { m.trim_end_matches('0').trim_end_matches('.') } else { m };
            s = format!("{}{}{}", m2, if upper { "E" } else { "e" }, e);
        }
        s
    } else {
        let dec = (p as i32 - 1 - exp).max(0) as usize;
        let mut s = format!("{:.*}", dec, v);
        if s.contains('.') { s = s.trim_end_matches('0').trim_end_matches('.').to_string(); }
        s
    }
}
fn do_printf(fmt: &str, args: &[Ex], row: &Row, ctx: &Ctx) -> Result<String, String> {
    let mut out = String::new(); let mut ai = 0;
    let cs: Vec<char> = fmt.chars().collect(); let mut i = 0;
    while i < cs.len() {
        let c = cs[i]; i += 1;
        if c != '%' { out.push(c); continue; }
        // flags
        let (mut minus, mut zero, mut plus, mut space) = (false, false, false, false);
        while i < cs.len() {
            match cs[i] { '-' => minus = true, '0' => zero = true, '+' => plus = true, ' ' => space = true, '#' | '!' | ',' => {}, _ => break }
            i += 1;
        }
        let mut width = 0usize;
        while i < cs.len() && cs[i].is_ascii_digit() { width = width * 10 + cs[i] as usize - 48; i += 1; }
        let mut prec: Option<usize> = None;
        if i < cs.len() && cs[i] == '.' {
            i += 1; let mut p = 0usize;
            while i < cs.len() && cs[i].is_ascii_digit() { p = p * 10 + cs[i] as usize - 48; i += 1; }
            prec = Some(p);
        }
        while i < cs.len() && matches!(cs[i], 'l' | 'h') { i += 1; } // length modifiers
        let conv = if i < cs.len() { let c = cs[i]; i += 1; c } else { break };
        if conv == '%' { out.push('%'); continue; }
        let v = eval_expr(&args[ai], row, ctx)?; ai += 1;
        let mut body = match conv {
            'd' | 'i' => { let n = v.as_i64();
                let mut s = n.abs().to_string();
                let sign = if n < 0 { "-" } else if plus { "+" } else if space { " " } else { "" };
                if zero && !minus && width > sign.len() + s.len() { s = format!("{}{}", "0".repeat(width - sign.len() - s.len()), s); }
                format!("{}{}", sign, s) }
            'u' => (v.as_i64() as u64).to_string(),
            'x' => format!("{:x}", v.as_i64()),
            'X' => format!("{:X}", v.as_i64()),
            'o' => format!("{:o}", v.as_i64()),
            'f' | 'F' => { let p = prec.unwrap_or(6); let f = v.as_f64();
                let mut s = format!("{:.*}", p, f.abs());
                let sign = if f.is_sign_negative() { "-" } else if plus { "+" } else { "" };
                if zero && !minus && width > sign.len() + s.len() { s = format!("{}{}", "0".repeat(width - sign.len() - s.len()), s); }
                format!("{}{}", sign, s) }
            'e' | 'E' => c_exp_fmt(v.as_f64(), prec.unwrap_or(6), conv == 'E'),
            'g' | 'G' => c_g_fmt(v.as_f64(), prec.unwrap_or(6), conv == 'G'),
            's' | 'z' => { let mut s = v.as_text(); if let Some(p) = prec { s = s.chars().take(p).collect(); } s }
            'c' => v.as_text().chars().next().map(|c| c.to_string()).unwrap_or_default(),
            'q' => v.as_text().replace('\'', "''"),
            'Q' => match v { V::Null => "NULL".into(), _ => format!("'{}'", v.as_text().replace('\'', "''")) },
            other => return Err(format!("unsupported printf conversion %{other}")),
        };
        if body.chars().count() < width {
            let pad = " ".repeat(width - body.chars().count());
            body = if minus { format!("{}{}", body, pad) } else { format!("{}{}", pad, body) };
        }
        out.push_str(&body);
    }
    Ok(out)
}

// ---------------- SELECT executor ----------------
fn find_kw_top(s: &str, kw: &str) -> Option<usize> {
    let up = s.to_ascii_uppercase(); let kwu = kw.to_ascii_uppercase();
    let b = up.as_bytes(); let mut depth = 0i32; let mut i = 0;
    while i < b.len() {
        match b[i] { b'(' => depth += 1, b')' => depth -= 1, _ => {} }
        if depth == 0 && up[i..].starts_with(&kwu) {
            let before = i == 0 || !b[i-1].is_ascii_alphanumeric();
            let after = i + kwu.len() >= b.len() || !b[i+kwu.len()].is_ascii_alphanumeric();
            if before && after { return Some(i); }
        }
        i += 1;
    }
    None
}
fn split_top(s: &str, sep: char) -> Vec<String> {
    let mut out = Vec::new(); let mut depth = 0; let mut cur = String::new(); let mut inq = false;
    for c in s.chars() {
        if c == '\'' { inq = !inq; cur.push(c); continue; }
        if inq { cur.push(c); continue; }
        match c { '(' => { depth += 1; cur.push(c); } ')' => { depth -= 1; cur.push(c); }
                  c if c == sep && depth == 0 => { out.push(cur.trim().to_string()); cur.clear(); }
                  _ => cur.push(c) }
    }
    if !cur.trim().is_empty() { out.push(cur.trim().to_string()); }
    out
}

fn source_rows(ctx: &Ctx, from: &str) -> Result<(Vec<String>, Vec<Row>), String> {
    let f = from.trim();
    if f.starts_with('(') {
        let inner = &f[1..f.rfind(')').ok_or("bad subquery")?];
        let (cols, rows) = select_rows_o(ctx, inner, &Row::new())?;
        let rmaps = rows.into_iter().map(|r| cols.iter().cloned().zip(r).collect()).collect();
        return Ok((cols, rmaps));
    }
    // table-valued functions
    if let Some(op) = f.find('(') {
        let fname = f[..op].trim().to_ascii_lowercase();
        let arg = f[op+1..f.rfind(')').unwrap_or(f.len())].trim().trim_matches('\'').to_string();
        match fname.as_str() {
            "generate_series" => {
                let parts: Vec<i64> = f[op+1..f.rfind(')').unwrap()].split(',').map(|x| x.trim().parse().unwrap_or(0)).collect();
                let (a, b, step) = (parts[0], parts[1], *parts.get(2).unwrap_or(&1));
                let mut rows = Vec::new(); let mut v = a;
                while (step > 0 && v <= b) || (step < 0 && v >= b) { let mut m = Row::new(); m.insert("value".into(), V::Int(v)); rows.push(m); v += step; }
                return Ok((vec!["value".into()], rows));
            }
            "prefixes" => {
                let chars: Vec<char> = arg.chars().collect();
                let rows = (0..=chars.len()).map(|k| { let mut m = Row::new(); m.insert("prefix".into(), V::Text(chars[..k].iter().collect())); m }).collect();
                return Ok((vec!["prefix".into()], rows));
            }
            "json_each" => {
                let vals = crate::json::each(&arg)?;
                let rows = vals.into_iter().enumerate().map(|(i, v)| { let mut m = Row::new();
                    m.insert("key".into(), V::Int(i as i64)); m.insert("value".into(), v); m }).collect();
                return Ok((vec!["key".into(), "value".into()], rows));
            }
            "pragma_table_info" => { let n = ctx.tables.get(&arg).map(|(c,_)| c.len()).unwrap_or(0);
                return Ok((vec!["x".into()], (0..n).map(|_| Row::new()).collect())); }
            "pragma_foreign_key_list" => { let n = *ctx.fk_counts.get(&arg).unwrap_or(&0);
                return Ok((vec!["x".into()], (0..n).map(|_| Row::new()).collect())); }
            "pragma_index_list" => { let n = *ctx.index_counts.get(&arg).unwrap_or(&0);
                return Ok((vec!["x".into()], (0..n).map(|_| Row::new()).collect())); }
            "pragma_database_list" => { let mut rows = vec![{ let mut m=Row::new(); m.insert("name".into(), V::Text("main".into())); m }];
                for a in &ctx.conn.attached { let mut m=Row::new(); m.insert("name".into(), V::Text(a.clone())); rows.push(m); }
                return Ok((vec!["name".into()], rows)); }
            _ => return Err(format!("unsupported table source: {fname}")),
        }
    }
    if f.eq_ignore_ascii_case("pragma_database_list") {
        let mut rows = vec![{ let mut m=Row::new(); m.insert("name".into(), V::Text("main".into())); m }];
        for a in &ctx.conn.attached { let mut m=Row::new(); m.insert("name".into(), V::Text(a.clone())); rows.push(m); }
        return Ok((vec!["name".into()], rows));
    }
    // view: expand its stored SELECT (real re-execution, not a cache)
    if let Some(vsql) = ctx.views.get(f) {
        let (cols, rows) = select_rows_o(ctx, vsql, &Row::new())?;
        let rmaps = rows.into_iter().map(|r| cols.iter().cloned().zip(r).collect()).collect();
        return Ok((cols, rmaps));
    }
    // plain store table
    if let Some((cols, rows)) = ctx.tables.get(f) {
        let rmaps = rows.iter().map(|r| cols.iter().cloned().zip(r.iter().cloned()).collect()).collect();
        return Ok((cols.clone(), rmaps));
    }
    Err(format!("no such table: {f}"))
}

fn is_agg(name: &str) -> bool { matches!(name.to_ascii_lowercase().as_str(),
    "count"|"sum"|"total"|"avg"|"min"|"max"|"group_concat") }
fn expr_has_agg(e: &Ex) -> bool {
    match e { Ex::Func(n, a) => (is_agg(n) && !(matches!(n.to_ascii_lowercase().as_str(), "min"|"max") && a.len() > 1)) || a.iter().any(expr_has_agg),
        Ex::Bin(_, x, y) | Ex::Is(x, y, _) => expr_has_agg(x) || expr_has_agg(y),
        Ex::Unary(_, x) | Ex::IsNull(x, _) | Ex::Cast(x, _) | Ex::Collate(x, _) => expr_has_agg(x),
        Ex::InList(x, xs) => expr_has_agg(x) || xs.iter().any(expr_has_agg),
        Ex::Like(x, y, _, _) => expr_has_agg(x) || expr_has_agg(y),
        Ex::Case(w, el) => w.iter().any(|(a,b)| expr_has_agg(a)||expr_has_agg(b)) || el.as_ref().map_or(false, |e| expr_has_agg(e)),
        _ => false }
}
fn eval_agg(e: &Ex, rows: &[Row], ctx: &Ctx) -> Result<V, String> {
    if let Ex::Func(name, args) = e {
        let ln = name.to_ascii_lowercase();
        if is_agg(&ln) && !(matches!(ln.as_str(), "min"|"max") && args.len() > 1) {
            let is_star = matches!(args.get(0), Some(Ex::Col(c)) if c == "*");
            let mut vals = Vec::new();
            if !is_star { for r in rows { let v = eval_expr(&args[0], r, ctx)?; if !matches!(v, V::Null) { vals.push(v); } } }
            return Ok(match ln.as_str() {
                "count" => V::Int(if is_star { rows.len() as i64 } else { vals.len() as i64 }),
                "sum" => { if vals.is_empty() { V::Null } else if vals.iter().all(|v| matches!(v, V::Int(_))) { V::Int(vals.iter().map(|v| v.as_i64()).sum()) } else { V::Real(vals.iter().map(|v| v.as_f64()).sum()) } }
                "total" => V::Real(vals.iter().map(|v| v.as_f64()).sum()),
                "avg" => if vals.is_empty() { V::Null } else { V::Real(vals.iter().map(|v| v.as_f64()).sum::<f64>() / vals.len() as f64) },
                "min" => vals.into_iter().min_by(vcmp).unwrap_or(V::Null),
                "max" => vals.into_iter().max_by(vcmp).unwrap_or(V::Null),
                "group_concat" => { let sep = if args.len() > 1 { eval_expr(&args[1], &Row::new(), ctx)?.as_text() } else { ",".into() };
                    V::Text(vals.iter().map(|v| v.as_text()).collect::<Vec<_>>().join(&sep)) }
                _ => V::Null,
            });
        }
    }
    // non-aggregate expr in an aggregate query: evaluate against first row
    eval_expr(e, rows.first().cloned().as_ref().unwrap_or(&Row::new()), ctx)
}

fn item_alias(item: &str) -> (String, String) {
    // returns (expr_str, colname)
    if let Some(p) = find_kw_top(item, "AS") {
        let expr = item[..p].trim().to_string();
        let alias = item[p+2..].trim().trim_matches(|c| c=='"'||c=='[' || c==']').to_string();
        return (expr, alias);
    }
    (item.trim().to_string(), item.trim().to_string())
}

// ---------------- FROM clause: single items, comma joins, INNER/LEFT JOIN ----------------
fn find_top_char(s: &str, want: char) -> Option<usize> {
    let mut depth = 0; let mut inq = false;
    for (i, c) in s.char_indices() {
        match c { '\'' => inq = !inq, '(' if !inq => depth += 1, ')' if !inq => depth -= 1,
                  c if c == want && depth == 0 && !inq => return Some(i), _ => {} }
    }
    None
}

/// depth/quote-aware case-insensitive replace of a top-level keyword phrase
fn replace_top(s: &str, from: &str, to: &str) -> String {
    let mut out = String::new(); let mut rest = s.to_string();
    loop {
        match find_kw_top(&rest, from) {
            Some(p) => { out.push_str(&rest[..p]); out.push_str(to); rest = rest[p+from.len()..].to_string(); }
            None => { out.push_str(&rest); return out; }
        }
    }
}

/// one FROM item: base source (table / (subquery) / tvf) + optional [AS] alias.
/// returns (qualifier, colnames, rows-with-bare-keys)
fn item_source(ctx: &Ctx, item: &str, outer: &Row) -> Result<(String, Vec<String>, Vec<Row>), String> {
    let it = item.trim();
    // split base / alias at top level
    let cs: Vec<char> = it.chars().collect();
    let mut depth = 0; let mut inq = false; let mut base_end = cs.len();
    for (i, &c) in cs.iter().enumerate() {
        match c { '\'' => inq = !inq, '(' if !inq => depth += 1, ')' if !inq => depth -= 1,
                  c if c.is_whitespace() && depth == 0 && !inq && i > 0 => { base_end = i; break; } _ => {} }
    }
    let base: String = cs[..base_end].iter().collect();
    let mut alias: String = cs[base_end..].iter().collect::<String>().trim().to_string();
    if alias.to_ascii_uppercase().starts_with("AS ") { alias = alias[3..].trim().to_string(); }
    let (cols, rows) = if base.starts_with('(') {
        let inner = &base[1..base.rfind(')').ok_or("bad subquery in FROM")?];
        let (c, rs) = select_rows_o(ctx, inner, outer)?;
        (c.clone(), rs.into_iter().map(|r| c.iter().cloned().zip(r).collect::<Row>()).collect())
    } else {
        source_rows(ctx, &base)?
    };
    let qual = if !alias.is_empty() { alias } else { base.trim().to_string() };
    Ok((qual, cols, rows))
}

/// nested-loop FROM evaluation: comma joins (cartesian; WHERE filters later),
/// INNER JOIN ... ON, LEFT [OUTER] JOIN ... ON. Rows carry qualified (alias.col)
/// keys plus bare col keys (first-wins on collision; pinned cases qualify ambiguity).
fn parse_from(ctx: &Ctx, from: &str, outer: &Row) -> Result<Vec<Row>, String> {
    // normalize separators at top level
    let mut f = replace_top(from, "LEFT OUTER JOIN", " LEFTJOIN ");
    f = replace_top(&f, "LEFT JOIN", " LEFTJOIN ");
    f = replace_top(&f, "INNER JOIN", " JOIN ");
    f = replace_top(&f, "CROSS JOIN", " , ");
    // split into (kind, segment)
    let mut segs: Vec<(u8, String)> = Vec::new(); // 0=first, 1=comma, 2=inner, 3=left
    let mut rest = f.trim().to_string(); let mut kind = 0u8;
    loop {
        let pj = find_kw_top(&rest, "JOIN");
        let pl = find_kw_top(&rest, "LEFTJOIN");
        let comma_pos = find_top_char(&rest, ',');
        // find earliest separator
        let mut best: Option<(usize, u8, usize)> = None; // (pos, kind, sep_len)
        if let Some(p) = pl { best = Some((p, 3, 8)); }
        if let Some(p) = pj { if best.map_or(true, |b| p < b.0) { best = Some((p, 2, 4)); } }
        if let Some(p) = comma_pos { if best.map_or(true, |b| p < b.0) { best = Some((p, 1, 1)); } }
        match best {
            Some((p, k, l)) => { segs.push((kind, rest[..p].trim().to_string())); kind = k; rest = rest[p+l..].trim().to_string(); }
            None => { segs.push((kind, rest.trim().to_string())); break; }
        }
    }
    // fold nested loop
    let mut acc: Vec<Row> = Vec::new();
    for (i, (k, seg)) in segs.iter().enumerate() {
        let (item_str, on_str) = match find_kw_top(seg, "ON") {
            Some(p) => (seg[..p].trim().to_string(), Some(seg[p+2..].trim().to_string())),
            None => (seg.trim().to_string(), None),
        };
        let (qual, cols, rows) = item_source(ctx, &item_str, outer)?;
        let qrows: Vec<Row> = rows.iter().map(|r| {
            let mut m = Row::new();
            for c in &cols {
                let v = r.get(c).cloned().unwrap_or(V::Null);
                m.insert(format!("{qual}.{c}"), v.clone());
                m.entry(c.clone()).or_insert(v);
            }
            m
        }).collect();
        if i == 0 { acc = qrows; continue; }
        let on_ex = match &on_str { Some(o) => Some(parse_expr_full(o)?), None => None };
        let mut next: Vec<Row> = Vec::new();
        for l in &acc {
            let mut matched = false;
            for r in &qrows {
                let mut m = l.clone();
                for (kk, vv) in r {
                    if !kk.contains('.') && m.contains_key(kk) && !m.contains_key(&format!("__ambig__{kk}")) {
                        m.insert(format!("__ambig__{kk}"), V::Null);
                    }
                    m.entry(kk.clone()).or_insert_with(|| vv.clone());
                }
                let keep = match &on_ex {
                    Some(e) => { let mut env = m.clone(); for (ok, ov) in outer { env.entry(ok.clone()).or_insert_with(|| ov.clone()); }
                                 eval_expr(e, &env, ctx)?.truthy() == Some(true) }
                    None => true,
                };
                if keep { matched = true; next.push(m); }
            }
            if *k == 3 && !matched {
                // LEFT JOIN: keep left row, right columns NULL
                let mut m = l.clone();
                for c in &cols {
                    if m.contains_key(c) && !m.contains_key(&format!("__ambig__{c}")) { m.insert(format!("__ambig__{c}"), V::Null); }
                    m.insert(format!("{qual}.{c}"), V::Null); m.entry(c.clone()).or_insert(V::Null);
                }
                next.push(m);
            }
        }
        acc = next;
    }
    Ok(acc)
}

/// synthetic all-NULL row carrying every column key (and ambiguity markers) of a FROM clause
fn schema_row_of(ctx: &Ctx, from: &str, outer: &Row) -> Result<Row, String> {
    let mut f = replace_top(from, "LEFT OUTER JOIN", " , ");
    f = replace_top(&f, "LEFT JOIN", " , ");
    f = replace_top(&f, "INNER JOIN", " , ");
    f = replace_top(&f, "CROSS JOIN", " , ");
    f = replace_top(&f, "JOIN", " , ");
    let mut m = Row::new();
    for seg in split_top(&f, ',') {
        let item = match find_kw_top(&seg, "ON") { Some(p) => seg[..p].trim().to_string(), None => seg.trim().to_string() };
        if item.is_empty() { continue; }
        let (qual, cols, _rows) = item_source(ctx, &item, outer)?;
        for c in &cols {
            m.insert(format!("{qual}.{c}"), V::Null);
            if m.contains_key(c) && !m.contains_key(&format!("__ambig__{c}")) { m.insert(format!("__ambig__{c}"), V::Null); }
            m.entry(c.clone()).or_insert(V::Null);
        }
    }
    Ok(m)
}

fn select_core(ctx: &Ctx, sql: &str, outer: &Row) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    let s = sql.trim();
    let up = s.to_ascii_uppercase();
    if !up.starts_with("SELECT") { return Err("not a SELECT".into()); }
    let mut rest = s[6..].to_string();
    // GROUP BY (top level; ORDER BY/LIMIT already stripped by select_rows_o)
    let mut group_str: Option<String> = None;
    if let Some(g) = find_kw_top(&rest, "GROUP BY") { group_str = Some(rest[g+8..].trim().to_string()); rest = rest[..g].trim().to_string(); }
    let from_pos = find_kw_top(&rest, "FROM");
    let where_pos = find_kw_top(&rest, "WHERE");
    let items_end = [from_pos, where_pos].into_iter().flatten().min().unwrap_or(rest.len());
    let items_str = rest[..items_end].to_string();
    let (from_str, where_str) = match (from_pos, where_pos) {
        (Some(f), Some(w)) if w > f => (Some(rest[f+4..w].trim().to_string()), Some(rest[w+5..].trim().to_string())),
        (Some(f), None) => (Some(rest[f+4..].trim().to_string()), None),
        (None, Some(w)) => (None, Some(rest[w+5..].trim().to_string())),
        _ => (None, None),
    };
    let items: Vec<(String, String)> = split_top(&items_str, ',').iter().map(|i| item_alias(i)).collect();

    // window handling (two pinned shapes)
    if items.iter().any(|(e, _)| find_kw_top(e, "OVER").is_some()) {
        return window_select(ctx, &items, from_str.as_deref().unwrap_or(""));
    }

    let src: Vec<Row> = match &from_str { Some(f) => parse_from(ctx, f, outer)?, None => vec![Row::new()] };
    // eager name resolution: C reports "no such column"/"ambiguous column name" at
    // prepare time even when the source is empty. Build a schema row and probe.
    if from_str.is_some() && src.is_empty() {
        let mut schema = Row::new();
        if let Some(f) = &from_str {
            // one synthetic NULL row per FROM item, merged like parse_from does
            if let Ok(srows) = schema_row_of(ctx, f, outer) { schema = srows; }
        }
        for (k, v) in outer { schema.entry(k.clone()).or_insert_with(|| v.clone()); }
        for (e, _) in &items {
            if find_kw_top(e, "OVER").is_some() { continue; }
            if let Ok(ex) = parse_expr_full(e) {
                if let Err(msg) = eval_expr(&ex, &schema, ctx) {
                    if msg.starts_with("no such column") || msg.starts_with("ambiguous column") { return Err(msg); }
                }
            }
        }
        if let Some(w) = &where_str {
            if let Ok(ex) = parse_expr_full(w) {
                if let Err(msg) = eval_expr(&ex, &schema, ctx) {
                    if msg.starts_with("no such column") || msg.starts_with("ambiguous column") { return Err(msg); }
                }
            }
        }
    }
    // env for expression evaluation = inner row + outer bindings (inner wins)
    let with_outer = |r: &Row| -> Row {
        let mut m = r.clone();
        for (k, v) in outer { m.entry(k.clone()).or_insert_with(|| v.clone()); }
        m
    };
    // WHERE
    let src: Vec<Row> = if let Some(w) = &where_str {
        let we = parse_expr_full(w)?;
        let mut keep = Vec::new();
        for r in src { if eval_expr(&we, &with_outer(&r), ctx)?.truthy() == Some(true) { keep.push(r); } }
        keep
    } else { src };

    let colnames: Vec<String> = items.iter().map(|(_, a)| a.clone()).collect();
    let exprs: Vec<Ex> = items.iter().map(|(e, _)| parse_expr_full(e)).collect::<Result<_,_>>()?;
    let mut out = Vec::new();
    if let Some(g) = group_str {
        // real grouping: key exprs evaluated per row, groups in first-seen order
        let key_exprs: Vec<Ex> = split_top(&g, ',').iter().map(|k| parse_expr_full(k)).collect::<Result<_,_>>()?;
        let mut order: Vec<String> = Vec::new();
        let mut groups: std::collections::HashMap<String, Vec<Row>> = std::collections::HashMap::new();
        for r in &src {
            let env = with_outer(r);
            let mut kv = Vec::new();
            for ke in &key_exprs { kv.push(format!("{:?}", eval_expr(ke, &env, ctx)?.render())); }
            let key = kv.join("\u{1}");
            if !groups.contains_key(&key) { order.push(key.clone()); }
            groups.entry(key).or_default().push(env);
        }
        for key in order {
            let rows = &groups[&key];
            let mut orow = Vec::new();
            for e in &exprs { orow.push(eval_agg(e, rows, ctx)?); }
            out.push(orow);
        }
    } else {
        let env_rows: Vec<Row> = src.iter().map(|r| with_outer(r)).collect();
        let has_agg = exprs.iter().any(expr_has_agg);
        if has_agg {
            let mut row = Vec::new();
            for e in &exprs { row.push(eval_agg(e, &env_rows, ctx)?); }
            out.push(row);
        } else {
            for r in &env_rows { let mut orow = Vec::new(); for e in &exprs { orow.push(eval_expr(e, r, ctx)?); } out.push(orow); }
        }
    }
    Ok((colnames, out))
}

fn window_select(ctx: &Ctx, items: &[(String, String)], from: &str) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    let (_c, src) = source_rows(ctx, from)?;
    // both pins ORDER BY x ascending
    let base_col = items[0].0.trim(); // first item is the ordering column x
    let ordcol = base_col.to_string();
    let mut idx: Vec<usize> = (0..src.len()).collect();
    idx.sort_by(|&a, &b| vcmp(src[a].get(&ordcol).unwrap_or(&V::Null), src[b].get(&ordcol).unwrap_or(&V::Null)));
    let mut out = Vec::new();
    for (rank, &i) in idx.iter().enumerate() {
        let mut orow = Vec::new();
        for (expr, _) in items {
            let el = expr.to_ascii_lowercase();
            if find_kw_top(expr, "OVER").is_none() {
                orow.push(eval_expr(&P::new(expr)?.expr()?, &src[i], ctx)?);
            } else if el.contains("row_number") {
                orow.push(V::Int(rank as i64 + 1));
            } else if el.starts_with("sum(") {
                // sum over ROWS BETWEEN 1 PRECEDING AND CURRENT ROW
                let col = &ordcol;
                let cur = src[i].get(col).map(|v| v.as_f64()).unwrap_or(0.0);
                let prev = if rank > 0 { src[idx[rank-1]].get(col).map(|v| v.as_f64()).unwrap_or(0.0) } else { 0.0 };
                let s = cur + prev; orow.push(if s == s.trunc() { V::Int(s as i64) } else { V::Real(s) });
            } else { return Err("unsupported window".into()); }
        }
        out.push(orow);
    }
    Ok((items.iter().map(|(_, a)| a.clone()).collect(), out))
}

fn select_rows_o(ctx: &Ctx, sql: &str, outer: &Row) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    let mut s = sql.trim().trim_end_matches(';').trim().to_string();
    // trailing LIMIT [OFFSET] (top level; textually after ORDER BY)
    let mut limit: Option<(usize, usize)> = None;
    if let Some(p) = find_kw_top(&s, "LIMIT") {
        let tail = s[p+5..].trim().to_string();
        s = s[..p].trim().to_string();
        let (n, off) = match find_kw_top(&tail, "OFFSET") {
            Some(o) => (tail[..o].trim().to_string(), tail[o+6..].trim().parse::<usize>().map_err(|_| "bad OFFSET")?),
            None => (tail, 0),
        };
        limit = Some((n.trim().parse::<usize>().map_err(|_| "bad LIMIT")?, off));
    }
    // trailing ORDER BY (top level)
    let mut order: Option<String> = None;
    if let Some(p) = find_kw_top(&s, "ORDER BY") { order = Some(s[p+8..].trim().to_string()); s = s[..p].trim().to_string(); }
    // split compound set-ops: UNION [ALL] / INTERSECT / EXCEPT (left-assoc)
    #[derive(PartialEq)] enum Op { First, Union, UnionAll, Intersect, Except }
    let mut parts: Vec<(Op, String)> = Vec::new();
    let mut rest = s.clone(); let mut pend = Op::First;
    loop {
        let pu = find_kw_top(&rest, "UNION");
        let pi = find_kw_top(&rest, "INTERSECT");
        let pe = find_kw_top(&rest, "EXCEPT");
        let best = [pu.map(|p| (p, 0u8)), pi.map(|p| (p, 1)), pe.map(|p| (p, 2))]
            .into_iter().flatten().min_by_key(|(p, _)| *p);
        match best {
            Some((p, which)) => {
                parts.push((pend, rest[..p].trim().to_string()));
                match which {
                    0 => { let after = rest[p+5..].trim_start();
                           if after.to_ascii_uppercase().starts_with("ALL") { pend = Op::UnionAll; rest = after[3..].trim_start().to_string(); }
                           else { pend = Op::Union; rest = after.to_string(); } }
                    1 => { pend = Op::Intersect; rest = rest[p+9..].trim_start().to_string(); }
                    _ => { pend = Op::Except; rest = rest[p+6..].trim_start().to_string(); }
                }
            }
            None => { parts.push((pend, rest.trim().to_string())); break; }
        }
    }
    let key_of = |r: &Vec<V>| r.iter().map(|v| format!("{:?}", v.render())).collect::<Vec<_>>().join("\u{1}");
    let distinct = |rows: Vec<Vec<V>>| -> Vec<Vec<V>> {
        let mut seen = std::collections::HashSet::new();
        rows.into_iter().filter(|r| seen.insert(key_of(r))).collect()
    };
    let mut colnames = Vec::new();
    let mut rows: Vec<Vec<V>> = Vec::new();
    for (op, core) in &parts {
        let (cn, rs) = select_core(ctx, core, outer)?;
        match op {
            Op::First => { colnames = cn; rows = rs; }
            Op::UnionAll => rows.extend(rs),
            Op::Union => { rows.extend(rs); rows = distinct(std::mem::take(&mut rows)); }
            Op::Intersect => { let rk: std::collections::HashSet<String> = rs.iter().map(key_of).collect();
                               rows = distinct(std::mem::take(&mut rows)).into_iter().filter(|r| rk.contains(&key_of(r))).collect(); }
            Op::Except => { let rk: std::collections::HashSet<String> = rs.iter().map(key_of).collect();
                            rows = distinct(std::mem::take(&mut rows)).into_iter().filter(|r| !rk.contains(&key_of(r))).collect(); }
        }
    }
    if let Some(ob) = order {
        // multi-key ORDER BY over output columns (name, qualified name, or 1-based ordinal), ASC/DESC
        let mut keys: Vec<(usize, bool)> = Vec::new();
        for term in split_top(&ob, ',') {
            let mut wds = term.split_whitespace();
            let name = wds.next().unwrap_or("1").to_string();
            let desc = wds.next().map_or(false, |w| w.eq_ignore_ascii_case("DESC"));
            let ci = name.parse::<usize>().map(|n| n - 1).unwrap_or_else(|_| {
                colnames.iter().position(|c| *c == name)
                    .or_else(|| colnames.iter().position(|c| c.rsplit('.').next() == name.rsplit('.').next()))
                    .unwrap_or(0)
            });
            keys.push((ci, desc));
        }
        rows.sort_by(|a, b| {
            for (ci, desc) in &keys {
                let o = vcmp(a.get(*ci).unwrap_or(&V::Null), b.get(*ci).unwrap_or(&V::Null));
                if o != std::cmp::Ordering::Equal { return if *desc { o.reverse() } else { o }; }
            }
            std::cmp::Ordering::Equal
        });
    }
    if let Some((n, off)) = limit {
        rows = rows.into_iter().skip(off).take(n).collect();
    }
    Ok((colnames, rows))
}

// ---------------- statement dispatcher ----------------
/// Returns Ok(Some(rows)) if handled, Ok(None) if not an eval statement, Err on eval error.
pub fn run_stmt(ctx: &mut Ctx, sql: &str) -> Result<Option<Vec<Vec<Option<String>>>>, String> {
    let s = sql.trim().trim_end_matches(';').trim();
    let up = s.to_ascii_uppercase();
    if up.starts_with("PRAGMA ") {
        return Ok(Some(run_pragma(ctx, &s[7..])?));
    }
    if up.starts_with("ATTACH ") {
        if let Some(p) = find_kw_top(s, "AS") { ctx.conn.attached.push(s[p+2..].trim().to_string()); }
        return Ok(Some(vec![]));
    }
    if up.starts_with("DETACH ") {
        let name = s[7..].trim().trim_start_matches("DATABASE ").trim();
        ctx.conn.attached.retain(|a| a != name);
        return Ok(Some(vec![]));
    }
    if up.starts_with("SELECT") {
        let (_c, rows) = select_rows_o(ctx, s, &Row::new())?;
        return Ok(Some(rows.into_iter().map(|r| r.into_iter().map(|v| v.render()).collect()).collect()));
    }
    Ok(None)
}

fn run_pragma(ctx: &mut Ctx, body: &str) -> Result<Vec<Vec<Option<String>>>, String> {
    let b = body.trim();
    let (name, val) = match b.split_once('=') { Some((n, v)) => (n.trim().to_ascii_lowercase(), Some(v.trim().to_string())), None => (b.trim().to_ascii_lowercase(), None) };
    let boolval = |v: &str| -> i64 { match v.to_ascii_uppercase().as_str() { "ON"|"TRUE"|"YES" => 1, "OFF"|"FALSE"|"NO" => 0, _ => v.parse().unwrap_or(0) } };
    match name.as_str() {
        "integrity_check" | "quick_check" => Ok(vec![vec![Some("ok".into())]]),
        "encoding" => Ok(if val.is_none() { vec![vec![Some("UTF-8".into())]] } else { vec![] }),
        "journal_mode" => { let mode = if ctx.conn.is_file { "delete" } else { "memory" };
            Ok(vec![vec![Some(mode.into())]]) } // get and set both report the mode
        "locking_mode" => Ok(vec![vec![Some("normal".into())]]),
        "page_size" => { match val { Some(v) => { ctx.conn.pragmas.insert(name.clone(), boolval(&v)); Ok(vec![]) }
            None => { let cur = *ctx.conn.pragmas.get(&name).unwrap_or(&4096); Ok(vec![vec![Some(cur.to_string())]]) } } }
        "busy_timeout" => { match val {
            Some(v) => { let n = boolval(&v); ctx.conn.pragmas.insert(name.clone(), n); Ok(vec![vec![Some(n.to_string())]]) } // set RETURNS the value
            None => { let cur = *ctx.conn.pragmas.get(&name).unwrap_or(&0); Ok(vec![vec![Some(cur.to_string())]]) } } }
        "foreign_keys" | "synchronous" | "read_uncommitted" | "trusted_schema" | "threads"
        | "analysis_limit" | "reverse_unordered_selects" | "cell_size_check" | "fullfsync"
        | "checkpoint_fullfsync" | "secure_delete" => {
            match val {
                Some(v) => { ctx.conn.pragmas.insert(name.clone(), boolval(&v)); Ok(vec![]) }
                None => { let dflt = match name.as_str() { "synchronous" => 2, "trusted_schema" => 1, _ => 0 };
                          let cur = *ctx.conn.pragmas.get(&name).unwrap_or(&dflt); Ok(vec![vec![Some(cur.to_string())]]) }
            }
        }
        "schema_version" => { match val { Some(v) => { ctx.conn.schema_version = v.parse().unwrap_or(0); Ok(vec![]) }
                                          None => Ok(vec![vec![Some(ctx.conn.schema_version.to_string())]]) } }
        "case_sensitive_like" => { if let Some(v) = val { ctx.conn.case_sensitive_like = boolval(&v) != 0; } Ok(vec![]) }
        "user_version" | "application_id" | "cache_size" | "recursive_triggers" | "defer_foreign_keys"
        | "query_only" | "temp_store" | "automatic_index" | "ignore_check_constraints" => {
            match val {
                Some(v) => { ctx.conn.pragmas.insert(name.clone(), boolval(&v)); Ok(vec![]) }
                None => { let cur = *ctx.conn.pragmas.get(&name).unwrap_or(&Conn::pragma_default(&name)); Ok(vec![vec![Some(cur.to_string())]]) }
            }
        }
        _ => Err(format!("unsupported pragma: {name}")),
    }
}
