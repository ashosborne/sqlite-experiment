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
    // run-33: REAL text via the ported SQLite FpDecode/%!.17g pipeline (fpdec.rs) —
    // byte-identical to C including its double-rounding artifacts (1/3 → ...332)
    crate::fpdec::render_f64_c(r)
}
pub fn text_to_num(t: &str) -> Option<f64> {
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
    pub journal: String,       // "" = default (delete for files, memory for :memory:); "wal" / "delete"
    pub pending_ckpt: Option<String>, // wal_checkpoint mode awaiting the post-exec file sync
    pub page_cur: i64,         // compact page count of the current image (run-34)
    pub page_hwm: i64,         // grow-only until VACUUM resets it (freelist model, run-34)
    pub vtabs: BTreeMap<String, String>, // run-38: virtual table name -> module (wholenumber)
    pub data_version: i64,     // run-44: bumps when a sibling's commit is picked up (PRAGMA data_version = 1 + this)
    /// run-48: durable vtab schema rows (name, module, args, sql) — persisted in file
    /// images with rootpage 0; instances reconnect on demand via xConnect
    pub vtab_schema: Vec<(String, String, Vec<String>, String)>,
    pub last_rowid: i64,       // run-48: sqlite3_last_insert_rowid (vtab xUpdate sets it too)
    /// run-55: rollback-journal pager state for a file-backed DELETE-mode write txn.
    /// pager_base = the db file bytes at txn start (the rollback pre-image); set when
    /// the first modifying statement of an explicit txn flushes through the pager.
    pub pager_base: Option<Vec<u8>>,
    pub pager_journalled: bool, // a `<db>-journal` is live for the current txn
    /// run-56: btree handle transaction level for sqlite3_txn_state — 0 none/idle,
    /// 1 read (a SELECT ran in the txn), 2 write (a write / BEGIN IMMEDIATE|EXCLUSIVE).
    /// A deferred BEGIN alone stays 0 until the first statement touches the db (C shape).
    pub txn_level: u8,
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
        if c == '`' { // run-46: backtick identifier quoting (MySQL-compat form C accepts)
            self.i += 1; let mut out = String::new();
            while let Some(c) = self.peekc() { self.i += 1; if c == '`' { return Ok(Tok::Id(out)); } out.push(c); }
            return Err("unterminated `ident`".into());
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
    Filtered(Box<Ex>, Box<Ex>), // aggregate FILTER (WHERE ...)
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
        // run-38: [NOT] BETWEEN lo AND hi -> (l>=lo AND l<=hi) [negated]
        {
            let neg = self.kw("not");
            let save = self.i;
            if neg { self.i += 1; }
            if self.eat_kw("between") {
                let lo = self.add_expr()?;
                if !self.eat_kw("and") { return Err("expected AND in BETWEEN".into()); }
                let hi = self.add_expr()?;
                let ge = Ex::Bin(">=".into(), Box::new(l.clone()), Box::new(lo));
                let le = Ex::Bin("<=".into(), Box::new(l.clone()), Box::new(hi));
                let between = Ex::Bin("and".into(), Box::new(ge), Box::new(le));
                return Ok(if neg { Ex::Unary("NOT".into(), Box::new(between)) } else { between });
            }
            self.i = save;
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
        // postfix COLLATE / FILTER / -> ->>
        loop {
            if self.eat_kw("collate") { if let Tok::Id(c) = self.peek().clone() { self.i += 1; e = Ex::Collate(Box::new(e), c); continue; } }
            if self.kw("filter") {
                self.i += 1;
                if !self.eat_punct("(") { return Err("expected ( after FILTER".into()); }
                if !self.eat_kw("where") { return Err("expected WHERE in FILTER".into()); }
                let w = self.expr()?;
                if !self.eat_punct(")") { return Err("expected )".into()); }
                e = Ex::Filtered(Box::new(e), Box::new(w));
                continue;
            }
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
                    let distinct = self.eat_kw("distinct");
                    if self.punct("*") { self.i += 1; args.push(Ex::Col("*".into())); }
                    else if !self.punct(")") { loop { args.push(self.expr()?); if !self.eat_punct(",") { break; } } }
                    if !self.eat_punct(")") { return Err("expected )".into()); }
                    let name = if distinct { format!("{id}#distinct") } else { id };
                    return Ok(Ex::Func(name, args));
                }
                // run-46: quoted qualifiers lex as separate tokens ("select" . x) —
                // merge Id '.' Id chains into one qualified column reference
                let mut full = id;
                while self.punct(".") {
                    let save = self.i;
                    self.i += 1;
                    if let Tok::Id(nxt) = self.peek().clone() { self.i += 1; full = format!("{full}.{nxt}"); }
                    else { self.i = save; break; }
                }
                Ok(Ex::Col(full))
            }
            other => Err(format!("unexpected token {:?}", other)),
        }
    }
}

// ---------------- evaluation ----------------
type Row = std::collections::HashMap<String, V>;

pub struct Ctx<'a> {
    pub db: usize,
    pub conn: &'a mut Conn,
    pub tables: &'a std::collections::HashMap<String, (Vec<String>, Vec<Vec<V>>)>,
    pub fk_counts: &'a std::collections::HashMap<String, usize>,
    pub index_counts: &'a std::collections::HashMap<String, usize>,
    pub views: &'a std::collections::HashMap<String, String>,
    /// per-table single-column index maps (indexed col -> sorted key -> row positions),
    /// used for real index probes; empty when the caller supplies none.
    pub indexes: &'a std::collections::HashMap<String, Vec<(String, std::collections::BTreeMap<String, Vec<usize>>)>>,
    /// real index-probe counter (anti-cheat proof that lookups use the index)
    pub probes: &'a std::cell::Cell<u64>,
    /// declared column collations (lower colname -> lower collation name)
    pub col_colls: &'a std::collections::HashMap<String, String>,
    /// run-44: explicit index definitions (index name, table, columns) for pragma TVFs
    pub index_defs: &'a [(String, String, Vec<String>, String)], // (name, table, cols, sql)
    /// run-53: prebuilt pragma_index_list / pragma_foreign_key_list projections
    pub index_list_proj: &'a std::collections::HashMap<String, Vec<Vec<Option<String>>>>,
    pub fk_list_proj: &'a std::collections::HashMap<String, Vec<Vec<Option<String>>>>,
}

/// Evaluate one expression against a plain env (used by the store for CHECK
/// constraints and trigger bodies/WHEN clauses). Computed, never looked up.
pub fn eval_standalone(expr: &str, env: &std::collections::HashMap<String, V>) -> Result<V, String> {
    let e = parse_expr_full(expr)?;
    let tables = std::collections::HashMap::new();
    let fk = std::collections::HashMap::new();
    let ix = std::collections::HashMap::new();
    let views = std::collections::HashMap::new();
    let indexes = std::collections::HashMap::new();
    let probes = std::cell::Cell::new(0u64);
    let mut conn = Conn::default();
    let colls = std::collections::HashMap::new();
    let ilp = std::collections::HashMap::new();
    let fkp = std::collections::HashMap::new();
    let ctx = Ctx { db: 0, conn: &mut conn, tables: &tables, fk_counts: &fk, index_counts: &ix, views: &views, indexes: &indexes, probes: &probes, col_colls: &colls, index_defs: &[], index_list_proj: &ilp, fk_list_proj: &fkp };
    eval_expr(&e, env, &ctx)
}

fn rot13s(s: &str) -> String { s.chars().map(rot13c).collect() }
fn vnum_eq(a: &V, b: &V) -> Option<bool> {
    if matches!(a, V::Null) || matches!(b, V::Null) { return None; }
    if let (V::Blob(x), V::Blob(y)) = (a, b) { return Some(x == y); }
    if matches!(a, V::Blob(_)) || matches!(b, V::Blob(_)) { return Some(false); } // blob != non-blob
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
/// declared collation of a bare column reference (CREATE TABLE ... COLLATE name)
fn decl_collate_of(e: &Ex, ctx: &Ctx) -> Option<String> {
    if let Ex::Col(n) = e { ctx.col_colls.get(&n.rsplit('.').next().unwrap_or(n).to_ascii_lowercase()).cloned() } else { None }
}
/// resolve a collation name to a text ordering: builtins first, then the
/// per-connection create_collation registry (real xCompare); unknown -> Err
fn coll_order(ctx: &Ctx, name: &str, x: &V, y: &V) -> Result<std::cmp::Ordering, String> {
    Ok(match name {
        "uint" => uint_cmp(&x.as_text(), &y.as_text()),
        "rot13" => rot13s(&x.as_text()).cmp(&rot13s(&y.as_text())),
        "nocase" => x.as_text().to_lowercase().cmp(&y.as_text().to_lowercase()),
        "decimal" => dec_cmp(&x.as_text(), &y.as_text()).cmp(&0),
        "rtrim" => x.as_text().trim_end_matches(' ').cmp(y.as_text().trim_end_matches(' ')),
        "binary" => x.as_text().cmp(&y.as_text()),
        _ => match crate::coll_user_cmp(ctx.db, name, &x.as_text(), &y.as_text()) {
            Some(o) => o,
            None => return Err(format!("no such collation sequence: {name}")),
        },
    })
}
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
                    let coll = collate_of(a).or_else(|| collate_of(b))
                        .or_else(|| decl_collate_of(a, ctx)).or_else(|| decl_collate_of(b, ctx));
                    let eq = match coll.as_deref() {
                        Some(name) => {
                            // registry/builtin resolution happens even on NULL operands (C errors on unknown collation)
                            if !matches!(x, V::Null) && !matches!(y, V::Null) {
                                Some(coll_order(ctx, name, &x, &y)? == std::cmp::Ordering::Equal)
                            } else { coll_order(ctx, name, &V::Text(String::new()), &V::Text(String::new()))?; None }
                        }
                        None => vnum_eq(&x, &y),
                    };
                    match eq { Some(b) => V::Int((b ^ (op == "<>")) as i64), None => V::Null }
                }
                "<" | "<=" | ">" | ">=" => {
                    if matches!(x, V::Null) || matches!(y, V::Null) { return Ok(V::Null); }
                    let coll = collate_of(a).or_else(|| collate_of(b))
                        .or_else(|| decl_collate_of(a, ctx)).or_else(|| decl_collate_of(b, ctx));
                    let o = match coll.as_deref() {
                        Some(name) => coll_order(ctx, name, &x, &y)?,
                        None => vcmp(&x, &y),
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
        Ex::Filtered(f, _) => eval_expr(f, row, ctx)?, // non-agg context: filter is agg-only
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
        Ex::Filtered(f, w) => Ex::Filtered(Box::new(substitute_subqs(*f, subs)), Box::new(substitute_subqs(*w, subs))),
        other => other,
    }
}
/// Parse an expression string, extracting (SELECT ...) subqueries into Ex::Subq/Ex::Exists.
fn parse_expr_full(s: &str) -> Result<Ex, String> {
    let (s2, subs) = extract_subqueries(s);
    let e = P::new(&s2)?.expr()?;
    Ok(substitute_subqs(e, &subs))
}

// run-38: completion / pragma-registry contents (only what the pinned queries probe;
// under-claimed — the umbrella pragma/completion surfaces stay partial)
static SQL_KEYWORDS: &[&str] = &[
    // run-50: C's exact 147-keyword census (sqlite3_keyword_name order)
    "REINDEX","INDEXED","INDEX","DESC","ESCAPE","EACH","CHECK","KEY","BEFORE","FOREIGN",
    "FOR","IGNORE","REGEXP","EXPLAIN","INSTEAD","ADD","DATABASE","AS","SELECT","TABLE",
    "LEFT","THEN","END","DEFERRABLE","ELSE","EXCLUDE","DELETE","TEMPORARY","TEMP","OR",
    "ISNULL","NULLS","SAVEPOINT","INTERSECT","TIES","NOTNULL","NOT","NO","NULL","LIKE",
    "EXCEPT","TRANSACTION","ACTION","ON","NATURAL","ALTER","RAISE","EXCLUSIVE","EXISTS","CONSTRAINT",
    "INTO","OFFSET","OF","SET","TRIGGER","RANGE","GENERATED","DETACH","HAVING","GLOB",
    "BEGIN","INNER","REFERENCES","UNIQUE","QUERY","WITHOUT","WITH","OUTER","RELEASE","ATTACH",
    "BETWEEN","NOTHING","GROUPS","GROUP","CASCADE","ASC","DEFAULT","CASE","COLLATE","CREATE",
    "CURRENT_DATE","IMMEDIATE","JOIN","INSERT","MATCH","PLAN","ANALYZE","PRAGMA","MATERIALIZED","DEFERRED",
    "DISTINCT","IS","UPDATE","VALUES","VIRTUAL","ALWAYS","WHEN","WHERE","RECURSIVE","ABORT",
    "AFTER","RENAME","AND","DROP","PARTITION","AUTOINCREMENT","TO","IN","CAST","COLUMN",
    "COMMIT","CONFLICT","CROSS","CURRENT_TIMESTAMP","CURRENT_TIME","CURRENT","PRECEDING","FAIL","LAST","FILTER",
    "REPLACE","FIRST","FOLLOWING","FROM","FULL","LIMIT","IF","ORDER","RESTRICT","OTHERS",
    "OVER","RETURNING","RIGHT","ROLLBACK","ROWS","ROW","UNBOUNDED","UNION","USING","VACUUM",
    "VIEW","WINDOW","DO","BY","INITIALLY","ALL","PRIMARY",
];
static FUNCTION_LIST: &[&str] = &[
    "abs","changes","char","coalesce","count","glob","hex","ifnull","instr","length",
    "like","lower","ltrim","max","min","nullif","printf","quote","random","randomblob",
    "replace","round","rtrim","substr","sum","total","trim","typeof","unicode","upper",
    "zeroblob","avg","group_concat",
];
/// run-51: is this a function name C would consult SQLITE_FUNCTION (31) for?
pub fn is_known_function(name: &str) -> bool {
    let l = name.to_ascii_lowercase();
    FUNCTION_LIST.iter().any(|f| *f == l)
}

static PRAGMA_LIST: &[&str] = &[
    "foreign_keys","journal_mode","cache_size","page_count","integrity_check","user_version",
    "table_info","index_list","foreign_key_list","wal_checkpoint","synchronous","encoding",
];

/// 4-byte BE original length + (count,byte) RLE pairs (run-38 compress stand-in)
fn rle_compress(data: &[u8]) -> Vec<u8> {
    let mut out = (data.len() as u32).to_be_bytes().to_vec();
    let mut i = 0;
    while i < data.len() {
        let b = data[i]; let mut run = 1usize;
        while i + run < data.len() && data[i + run] == b && run < 255 { run += 1; }
        out.push(run as u8); out.push(b); i += run;
    }
    out
}
fn rle_uncompress(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 4 { return Err("bad compressed blob".into()); }
    let n = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let mut out = Vec::with_capacity(n);
    let mut i = 4;
    while i + 1 < data.len() { let (run, b) = (data[i] as usize, data[i + 1]);
        for _ in 0..run { out.push(b); } i += 2; }
    out.truncate(n);
    Ok(out)
}

fn eval_func(name: &str, args: &[Ex], row: &Row, ctx: &Ctx) -> Result<V, String> {
    // run-51: a function the authorizer answered SQLITE_IGNORE for yields NULL
    // (compile-time replacement in C; per-row NULL here, args unevaluated)
    if crate::auth_fn_ignored(name) { return Ok(V::Null); }

    let ln = name.to_ascii_lowercase();
    let a = |i: usize| -> Result<V, String> { eval_expr(&args[i], row, ctx) };
    Ok(match ln.as_str() {
        "->" | "->>" => { let doc = a(0)?.as_text(); let path = a(1)?.as_text();
            let val = crate::json::extract(&doc, &path)?;
            if ln == "->>" { crate::json::to_sql_text(val) } else { crate::json::to_json_text(val) } }
        "typeof" => V::Text(match a(0)? { V::Null=>"null",V::Int(_)=>"integer",V::Real(_)=>"real",V::Text(_)=>"text",V::Blob(_)=>"blob" }.into()),
        // run-42: SQL twins over the pinned compile-option table (ADR 0030)
        "sqlite_compileoption_used" => V::Int(crate::compileoption_used(&a(0)?.as_text())),
        "sqlite_compileoption_get" => match a(0)? {
            V::Null => V::Null,
            v => match crate::compileoption_get_str(v.as_f64() as i64) {
                Some(s) => V::Text(s.to_string()),
                None => V::Null,
            },
        },
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
        "quote" => match a(0)? { V::Null=>V::Text("NULL".into()),
            V::Text(t)=>V::Text(format!("'{}'", t.replace('\'',"''"))),
            V::Blob(b)=>V::Text(format!("X'{}'", b.iter().map(|x| format!("{:02X}", x)).collect::<String>())),
            v=>V::Text(v.as_text()) },
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
        "zeroblob" => V::Blob(vec![0u8; a(0)?.as_i64().max(0) as usize]),
        // run-38: reversible RLE stand-in for compress/uncompress (round-trips pinned;
        // zlib byte-format is a documented residual, NOT matched)
        "compress" => { let b = match a(0)? { V::Blob(b) => b, v => v.as_text().into_bytes() };
                        V::Blob(rle_compress(&b)) }
        "uncompress" => { let b = match a(0)? { V::Blob(b) => b, v => v.as_text().into_bytes() };
                          V::Blob(rle_uncompress(&b)?) }
        // run-38: next_char(prefix, table, column) — distinct following chars
        "next_char" => {
            let prefix = a(0)?.as_text();
            let table = a(1)?.as_text();
            let column = a(2)?.as_text();
            let (cols, rows) = ctx.tables.get(&table).ok_or(format!("no such table: {table}"))?;
            let ci = cols.iter().position(|c| c.eq_ignore_ascii_case(&column))
                .ok_or(format!("no such column: {column}"))?;
            let mut set: std::collections::BTreeSet<char> = Default::default();
            for r in rows {
                let w = r.get(ci).map(|v| v.as_text()).unwrap_or_default();
                if w.starts_with(&prefix) && w.chars().count() > prefix.chars().count() {
                    if let Some(c) = w.chars().nth(prefix.chars().count()) { set.insert(c); }
                }
            }
            V::Text(set.into_iter().collect())
        }
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
        "tointeger" => match a(0)? { V::Int(i)=>V::Int(i),
            V::Real(r) => if r == r.trunc() && r.abs() < 9.2e18 { V::Int(r as i64) } else { V::Null },
            V::Text(t)=> t.trim().parse::<i64>().map(V::Int).unwrap_or(V::Null), _=>V::Null },
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
        "sha1_query" => V::Text(sha1_hex(&query_hash_stream(ctx, &a(0)?.as_text())?)),
        "sha3_query" => { let bits = if args.len() > 1 { a(1)?.as_i64() } else { 256 };
            V::Blob(sha3_bytes(&query_hash_stream(ctx, &a(0)?.as_text())?, bits as usize)) }
        "base85" => match a(0)? {
            V::Blob(b) => V::Text(b85_encode(&b)),
            V::Text(s) => V::Blob(b85_decode(&s)),
            V::Null => V::Null,
            v => V::Blob(b85_decode(&v.as_text())),
        },
        "is_base85" => { let s = a(0)?.as_text();
            V::Int(s.chars().all(|c| matches!(c, '#'..='&' | '*'..='z') || c.is_ascii_whitespace()) as i64) }
        "ieee754_from_blob" => { let b = match a(0)? { V::Blob(b) => b, _ => return Err("blob required".into()) };
            if b.len() != 8 { return Err("8-byte blob required".into()); }
            V::Real(f64::from_bits(u64::from_be_bytes(b[..8].try_into().unwrap()))) }
        "ieee754_to_blob" => V::Blob(a(0)?.as_f64().to_bits().to_be_bytes().to_vec()),
        "decimal" => V::Text(dec_canon(&a(0)?.as_text())),
        "decimal_pow2" => V::Text(dec_pow2(a(0)?.as_i64())),
        "decimal_exp" => V::Text(dec_exp(&a(0)?.as_text())),
        "sha3" => { let bits = if args.len()>1 { a(1)?.as_i64() } else { 256 };
                    V::Blob(sha3_bytes(a(0)?.as_text().as_bytes(), bits as usize)) }
        "decimal_add" => V::Text(dec_add(&a(0)?.as_text(), &a(1)?.as_text())),
        "decimal_sub" => { let b = a(1)?.as_text();
            let nb = if let Some(r) = b.trim().strip_prefix('-') { r.to_string() } else { format!("-{}", b.trim()) };
            V::Text(dec_add(&a(0)?.as_text(), &nb)) }
        "decimal_mul" => V::Text(dec_mul(&a(0)?.as_text(), &a(1)?.as_text())),
        "decimal_cmp" => { let c = dec_cmp(&a(0)?.as_text(), &a(1)?.as_text()); V::Int(c as i64) }
        "uuid" => V::Text(uuid_v4()),
        "uuid_str" => { let hx = uuid_hex(&a(0)?)?;
            V::Text(format!("{}-{}-{}-{}-{}", &hx[0..8], &hx[8..12], &hx[12..16], &hx[16..20], &hx[20..32])) }
        "uuid_blob" => { let hx = uuid_hex(&a(0)?)?;
            V::Blob((0..32).step_by(2).map(|i| u8::from_str_radix(&hx[i..i+2], 16).unwrap()).collect()) }
        "sqlite3_uri_parameter" => V::Null,   // :memory: connection has no URI params
        "sqlite3_uri_boolean" => a(2)?,        // default value
        "regexp" => { let ok = tiny_regexp(&a(1)?.as_text(), &a(0)?.as_text()); V::Int(ok as i64) }
        // ---- JSON ----
        "json_extract" => { let doc=a(0)?.as_text(); let v=crate::json::extract(&doc,&a(1)?.as_text())?; crate::json::to_sql_text(v) }
        "json_valid" => {
            // run-46: optional FLAGS argument (1 strict text, 2 JSON5 text, 4/8 JSONB blob)
            let flags = if args.len() > 1 {
                match a(1)? { V::Null => 1, v => v.as_f64() as i64 }
            } else { 1 };
            if !(1..=15).contains(&flags) {
                return Err("FLAGS parameter to json_valid() must be between 1 and 15".into());
            }
            match a(0)? {
                V::Null => V::Null,
                V::Blob(b) => V::Int(((flags & 12) != 0 && crate::json::jsonb_valid(&b)) as i64),
                v => {
                    let t = v.as_text();
                    let ok = ((flags & 1) != 0 && crate::json::valid(&t))
                        || ((flags & 2) != 0 && crate::json::valid5(&t));
                    V::Int(ok as i64)
                }
            }
        }
        "json_type" => { let doc=a(0)?.as_text(); let p = if args.len()>1 { a(1)?.as_text() } else { "$".into() };
                         match crate::json::type_at(&doc,&p) { Some(t)=>V::Text(t), None=>V::Null } }
        // run-53: a NULL document argument yields NULL (C shape), not an error
        "json_set" | "json_insert" | "json_replace" if matches!(a(0)?, V::Null) => V::Null,
        "json_remove" | "json_patch" if matches!(a(0)?, V::Null) => V::Null,
        "json_set" | "json_insert" | "json_replace" => V::Text(crate::json::set(&a(0)?.as_text(), &a(1)?.as_text(), &a(2)?, &ln)?),
        "json_remove" => V::Text(crate::json::remove(&a(0)?.as_text(), &a(1)?.as_text())?),
        "json_patch" => V::Text(crate::json::patch(&a(0)?.as_text(), &a(1)?.as_text())?),
        _ => {
            // registered UDF name? (only then evaluate args — avoids choking on the
            // '*' pseudo-column of aggregates like count(*), which eval_agg handles)
            if ln == "load_extension" {
                // run-50: the SQL form stays refused on this pin even when the
                // C-API gate is open (C's API/SQL split, pinned; plain rc 1)
                return Err("__RC1__not authorized".into());
            }
            if ctx.db != 0 && crate::udf_name_exists(ctx.db, &ln) {
                if crate::udf_is_aggregate(ctx.db, &ln, args.len()) {
                    return Err(format!("misuse of aggregate function {ln}()"));
                }
                // run-50: untrusted schema refuses non-innocuous app functions in views
                if IN_VIEW.with(|d| d.get()) > 0
                    && *ctx.conn.pragmas.get("!untrusted_schema").unwrap_or(&0) != 0
                    && !crate::udf_innocuous(ctx.db, &ln) {
                    return Err(format!("unsafe use of {ln}()"));
                }
                let mut argvals = Vec::with_capacity(args.len());
                for e in args { argvals.push(eval_expr(e, row, ctx)?); }
                if let Some(r) = crate::udf_invoke_scalar(ctx.db, &ln, &argvals) {
                    return r;
                }
                return Err(format!("wrong number of arguments to function {ln}()"));
            }
            return Err(format!("no such function: {name}"));
        }
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
/// canonical lowercase 32-hex-digit extraction for uuid_str/uuid_blob (braces/dashes/case tolerated)
fn uuid_hex(v: &V) -> Result<String, String> {
    let hx: String = match v {
        V::Blob(b) => b.iter().map(|x| format!("{:02x}", x)).collect(),
        other => other.as_text().chars().filter(|c| c.is_ascii_hexdigit()).map(|c| c.to_ascii_lowercase()).collect(),
    };
    if hx.len() != 32 { return Err("not a valid UUID".into()); }
    Ok(hx)
}
/// the byte stream sha1_query()/sha3_query() hash: per statement "S{n}:"+sql,
/// per row "R", per value N | I+8BE | F+8BE | T{n}:+bytes | B{n}:+bytes
fn query_hash_stream(ctx: &Ctx, sql: &str) -> Result<Vec<u8>, String> {
    let mut buf: Vec<u8> = Vec::new();
    for stmt in sql.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() { continue; }
        buf.extend_from_slice(format!("S{}:", stmt.len()).as_bytes());
        buf.extend_from_slice(stmt.as_bytes());
        let (_c, rows) = select_rows_o(ctx, stmt, &Row::new())?;
        for r in rows {
            buf.push(b'R');
            for v in r {
                match v {
                    V::Null => buf.push(b'N'),
                    V::Int(i) => { buf.push(b'I'); buf.extend_from_slice(&(i as u64).to_be_bytes()); }
                    V::Real(f) => { buf.push(b'F'); buf.extend_from_slice(&f.to_bits().to_be_bytes()); }
                    V::Text(t) => { buf.extend_from_slice(format!("T{}:", t.len()).as_bytes()); buf.extend_from_slice(t.as_bytes()); }
                    V::Blob(b) => { buf.extend_from_slice(format!("B{}:", b.len()).as_bytes()); buf.extend_from_slice(&b); }
                }
            }
        }
    }
    Ok(buf)
}
/// SQLite base85.c encoding: 4-byte groups big-endian -> 5 numerals (MSB first),
/// tail of n bytes -> n+1 numerals, trailing newline when content was produced.
fn b85_numeral(d: u8) -> char { if d < 4 { (d + b'#') as char } else { (d - 4 + b'*') as char } }
fn b85_encode(b: &[u8]) -> String {
    let mut out = String::new();
    let mut chunks = b.chunks_exact(4);
    for ch in &mut chunks {
        let mut qv = u32::from_be_bytes(ch.try_into().unwrap()) as u64;
        let mut grp = ['#'; 5];
        for k in (0..5).rev() { grp[k] = b85_numeral((qv % 85) as u8); qv /= 85; }
        out.extend(grp);
    }
    let rem = chunks.remainder();
    if !rem.is_empty() {
        let mut qv = 0u64;
        for &x in rem { qv = (qv << 8) | x as u64; }
        let n = rem.len() + 1;
        let mut grp = vec!['#'; n];
        for k in (0..n).rev() { grp[k] = b85_numeral((qv % 85) as u8); qv /= 85; }
        out.extend(grp);
    }
    if !out.is_empty() || b.is_empty() { out.push('\n'); }
    out
}
fn b85_decode(s: &str) -> Vec<u8> {
    let digits: Vec<u8> = s.chars().filter(|c| matches!(c, '#'..='&' | '*'..='z'))
        .map(|c| if c <= '&' { c as u8 - b'#' } else { c as u8 - b'*' + 4 }).collect();
    let mut out = Vec::new();
    for grp in digits.chunks(5) {
        let nbo = match grp.len() { 5 => 4, 4 => 3, 3 => 2, 2 => 1, _ => 0 };
        if nbo == 0 { break; }
        let mut qv = 0u64;
        for &d in grp { qv = qv * 85 + d as u64; }
        for k in (0..nbo).rev() { out.push(((qv >> (8 * k)) & 0xff) as u8); }
    }
    out
}
/// decimal(X): canonical decimal text (strip leading zeros, keep trailing fraction digits)
fn dec_canon(s: &str) -> String {
    let t = s.trim();
    let (neg, t) = match t.strip_prefix('-') { Some(r) => (true, r), None => (false, t.strip_prefix('+').unwrap_or(t)) };
    let (i, f) = match t.split_once('.') { Some((a, b)) => (a, Some(b)), None => (t, None) };
    let i = i.trim_start_matches('0');
    let i = if i.is_empty() { "0" } else { i };
    let mut out = String::new();
    if neg { out.push('-'); }
    out.push_str(i);
    if let Some(f) = f { out.push('.'); out.push_str(f); }
    out
}
/// decimal_pow2(N): exact 2^N in the C decimal extension's exponential form (+D.DDDe+EE)
fn dec_pow2(n: i64) -> String {
    let digits: String; let exp: i64;
    if n >= 0 {
        let v: i128 = 1i128 << n.min(126);
        digits = v.to_string();
        exp = digits.len() as i64 - 1;
    } else {
        let m = (-n) as u32;
        let v: i128 = 5i128.pow(m.min(54)); // 5^54 < i128::MAX
        digits = v.to_string();
        exp = digits.len() as i64 - 1 + n; // 5^m * 10^-m normalized
    }
    let d: Vec<char> = digits.chars().collect();
    let mut mant: String = d[1..].iter().collect();
    while mant.ends_with('0') { mant.pop(); }
    if mant.is_empty() { mant.push('0'); } // C decimal keeps one fraction digit (+2.0e+00)
    format!("+{}.{}e{}{:02}", d[0], mant, if exp < 0 { '-' } else { '+' }, exp.abs())
}

/// decimal_exp(X): exponential form of a decimal string ('123.5' -> +1.235e+02)
fn dec_exp(s: &str) -> String {
    let t = s.trim();
    let (neg, t) = match t.strip_prefix('-') { Some(r) => (true, r), None => (false, t.strip_prefix('+').unwrap_or(t)) };
    let (ip, fp) = match t.split_once('.') { Some((a, b)) => (a, b), None => (t, "") };
    let digits: String = format!("{}{}", ip.trim_start_matches('0'), fp);
    let digits_trim = digits.trim_start_matches('0');
    let leading_zeros = digits.len() - digits_trim.len();
    let digits = if digits_trim.is_empty() { "0".to_string() } else { digits_trim.to_string() };
    // exponent: weight of the first significant digit
    let int_digits = ip.trim_start_matches('0').len() as i64;
    let exp = if int_digits > 0 { int_digits - 1 } else { -(leading_zeros as i64) - 1 };
    let d: Vec<char> = digits.chars().collect();
    let mut mant: String = d[1..].iter().collect();
    while mant.ends_with('0') { mant.pop(); }
    if mant.is_empty() { mant.push('0'); }
    format!("{}{}.{}e{}{:02}", if neg { '-' } else { '+' }, d[0], mant, if exp < 0 { '-' } else { '+' }, exp.abs())
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
        let (mut minus, mut zero, mut plus, mut space, mut alt, mut comma) = (false, false, false, false, false, false);
        while i < cs.len() {
            match cs[i] { '-' => minus = true, '0' => zero = true, '+' => plus = true, ' ' => space = true, '#' => alt = true, ',' => comma = true, '!' => {}, _ => break }
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
            'p' => format!("{:X}", v.as_i64()), // SQLite %p: uppercase hex of the value
            'd' | 'i' => { let n = v.as_i64();
                let mut s = n.abs().to_string();
                if comma {
                    let d: Vec<char> = s.chars().collect();
                    let mut g = String::new();
                    for (k, ch) in d.iter().enumerate() {
                        if k > 0 && (d.len() - k) % 3 == 0 { g.push(','); }
                        g.push(*ch);
                    }
                    s = g;
                }
                let sign = if n < 0 { "-" } else if plus { "+" } else if space { " " } else { "" };
                if zero && !minus && width > sign.len() + s.len() { s = format!("{}{}", "0".repeat(width - sign.len() - s.len()), s); }
                format!("{}{}", sign, s) }
            'u' => (v.as_i64() as u64).to_string(),
            'x' => { let s = format!("{:x}", v.as_i64()); if alt && v.as_i64() != 0 { format!("0x{s}") } else { s } }
            'X' => { let s = format!("{:X}", v.as_i64()); if alt && v.as_i64() != 0 { format!("0X{s}") } else { s } }
            'o' => { let s = format!("{:o}", v.as_i64()); if alt { format!("0{s}") } else { s } }
            'w' => v.as_text().replace('"', "\"\""),
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
/// does `up` begin with keyword `kw` at a word boundary?
pub(crate) fn kw_bound(up: &str, kw: &str) -> bool {
    up.starts_with(kw) && !matches!(up.as_bytes().get(kw.len()), Some(c) if c.is_ascii_alphanumeric() || *c == b'_')
}
fn find_kw_top(s: &str, kw: &str) -> Option<usize> {
    let up = s.to_ascii_uppercase(); let kwu = kw.to_ascii_uppercase();
    let b = up.as_bytes(); let mut depth = 0i32; let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'(' => depth += 1, b')' => depth -= 1,
            // run-46: keywords inside quoted identifiers / strings are not keywords
            b'\'' | b'"' | b'`' => {
                let q = b[i]; i += 1;
                while i < b.len() && b[i] != q { i += 1; }
                i += 1; continue;
            }
            b'[' => { while i < b.len() && b[i] != b']' { i += 1; } i += 1; continue; }
            _ => {}
        }
        if depth == 0 && b[i..].starts_with(kwu.as_bytes()) {
            let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
            let before = i == 0 || !word(b[i-1]);
            let after = i + kwu.len() >= b.len() || !word(b[i+kwu.len()]);
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

/// run-46: strip identifier quoting ("x", [x], `x`) so quoted reserved words reach
/// the same store keys their unquoted spellings would.
fn unquote_ident(s: &str) -> String {
    let t = s.trim();
    if t.len() >= 2 {
        let b = t.as_bytes();
        if (b[0] == b'"' && b[t.len()-1] == b'"') || (b[0] == b'`' && b[t.len()-1] == b'`')
            || (b[0] == b'[' && b[t.len()-1] == b']') {
            return t[1..t.len()-1].to_string();
        }
    }
    t.to_string()
}

/// run-50: split a TVF argument list on top-level commas, honouring single quotes,
/// and strip one level of outer quotes from each piece
fn split_tvf_args(raw: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut inq = false;
    let mut depth = 0i32;
    for c in raw.chars() {
        match c {
            '\'' => { inq = !inq; cur.push(c); }
            '(' | '[' | '{' if !inq => { depth += 1; cur.push(c); }
            ')' | ']' | '}' if !inq => { depth -= 1; cur.push(c); }
            ',' if !inq && depth == 0 => { out.push(std::mem::take(&mut cur)); }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() || !out.is_empty() { out.push(cur); }
    out.into_iter().map(|p| {
        let t = p.trim();
        if t.len() >= 2 && t.starts_with('\'') && t.ends_with('\'') {
            t[1..t.len()-1].replace("''", "'")
        } else { t.to_string() }
    }).collect()
}

thread_local! {
    // run-50: >0 while expanding a view body (TRUSTED_SCHEMA gate)
    static IN_VIEW: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

/// run-50: per-column (desc, collation) from a CREATE INDEX statement's column list
fn parse_index_col_attrs(sql: &str) -> Vec<(bool, String)> {
    let up = sql.to_ascii_uppercase();
    let open = match up.find('(') { Some(p) => p, None => return Vec::new() };
    let close = match sql.rfind(')') { Some(p) => p, None => return Vec::new() };
    if close <= open { return Vec::new(); }
    let list = &sql[open + 1..close];
    split_top(list, ',').iter().map(|part| {
        let pu = part.to_ascii_uppercase();
        let desc = pu.split_whitespace().last() == Some("DESC")
            || pu.trim_end().ends_with(" DESC");
        let coll = match pu.find("COLLATE ") {
            Some(cp) => part[cp + 8..].split_whitespace().next().unwrap_or("BINARY").to_string(),
            None => "BINARY".to_string(),
        };
        (desc, coll)
    }).collect()
}

fn source_rows(ctx: &Ctx, from: &str) -> Result<(Vec<String>, Vec<Row>), String> {
    let from_uq = if from.trim().starts_with('(') { from.trim().to_string() } else { unquote_ident(from) };
    let from = from_uq.as_str();
    let f = from.trim();
    // run-52: a materialized CTE shadows tables/views of the same name (C shape)
    if !f.starts_with('(') && !f.contains('(') {
        if let Some((cols, rows)) = cte_lookup(f) {
            let rmaps = rows.into_iter()
                .map(|r| cols.iter().cloned().zip(r).collect::<Row>()).collect();
            return Ok((cols, rmaps));
        }
    }
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
        // run-50: raw argument text for TVFs that take more than one argument
        let rawargs = f[op+1..f.rfind(')').unwrap_or(f.len())].to_string();
        if fname.starts_with("pragma_") {
            crate::pvtab_touch(ctx.db, &fname); // run-49: lazy module_list fill
        }
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
            "json_each" | "json_tree" => {
                // run-50: full vtab columns (key/value/type/atom/id/parent/fullkey/path)
                // with C's JSONB-offset ids; optional second argument roots the walk
                let parts = split_tvf_args(&rawargs);
                let doc = parts.first().cloned().unwrap_or_default();
                let rootp = parts.get(1).cloned();
                let walked = crate::json::walk(&doc, rootp.as_deref(), fname == "json_tree")?;
                let rows = walked.into_iter().map(|w| {
                    let mut m = Row::new();
                    m.insert("key".into(), w.key.unwrap_or(V::Null));
                    m.insert("value".into(), w.value);
                    m.insert("type".into(), V::Text(w.jtype));
                    m.insert("atom".into(), w.atom.unwrap_or(V::Null));
                    m.insert("id".into(), V::Int(w.id));
                    m.insert("parent".into(), w.parent.map(V::Int).unwrap_or(V::Null));
                    m.insert("fullkey".into(), V::Text(w.fullkey));
                    m.insert("path".into(), V::Text(w.path));
                    m
                }).collect();
                return Ok((vec!["key".into(), "value".into(), "type".into(), "atom".into(),
                                "id".into(), "parent".into(), "fullkey".into(), "path".into()], rows));
            }
            "pragma_table_info" => {
                // run-41: a vtab instance reports its declared visible shape (name/type)
                if let Some(shape) = crate::vtab_shape(ctx.db, &arg) {
                    let rows = shape.iter().filter(|(_, _, hidden)| !hidden).enumerate()
                        .map(|(i, (name, ty, _))| { let mut m = Row::new();
                            m.insert("cid".into(), V::Int(i as i64));
                            m.insert("name".into(), V::Text(name.clone()));
                            m.insert("type".into(), V::Text(ty.clone())); m }).collect();
                    return Ok((vec!["cid".into(), "name".into(), "type".into()], rows));
                }
                let n = ctx.tables.get(&arg).map(|(c,_)| c.len()).unwrap_or(0);
                return Ok((vec!["x".into()], (0..n).map(|_| Row::new()).collect())); }
            "pragma_foreign_key_list" => {
                // run-53: real projection (id/seq/table/from/to/on_update/on_delete/match)
                let cn: Vec<String> = vec!["id".into(), "seq".into(), "table".into(), "from".into(),
                              "to".into(), "on_update".into(), "on_delete".into(), "match".into()];
                let rows = ctx.fk_list_proj.get(&arg).cloned().unwrap_or_default().into_iter().map(|r| {
                    let mut m = Row::new();
                    for (c, v) in cn.iter().zip(r) {
                        m.insert(c.clone(), v.map(V::Text).unwrap_or(V::Null));
                    }
                    m
                }).collect();
                return Ok((cn, rows));
            }
            "pragma_index_list" => {
                // run-53: real projection (seq/name/unique/origin/partial)
                let cn: Vec<String> = vec!["seq".into(), "name".into(), "unique".into(), "origin".into(), "partial".into()];
                let rows = ctx.index_list_proj.get(&arg).cloned().unwrap_or_default().into_iter().map(|r| {
                    let mut m = Row::new();
                    for (c, v) in cn.iter().zip(r) {
                        m.insert(c.clone(), v.map(V::Text).unwrap_or(V::Null));
                    }
                    m
                }).collect();
                return Ok((cn, rows));
            }
            "pragma_database_list" => {
                let mut rows = Vec::new();
                let mut seq = 0i64;
                { let mut m=Row::new(); m.insert("seq".into(), V::Int(seq)); m.insert("name".into(), V::Text("main".into())); rows.push(m); }
                for a in &ctx.conn.attached { seq += 1; let mut m=Row::new(); m.insert("seq".into(), V::Int(seq)); m.insert("name".into(), V::Text(a.clone())); rows.push(m); }
                return Ok((vec!["seq".into(), "name".into()], rows)); }
            // run-44: registry/result TVFs (live data, not canned)
            "pragma_collation_list" => {
                let rows = crate::collation_list_names(ctx.db).into_iter().enumerate()
                    .map(|(i, n)| { let mut m = Row::new();
                        m.insert("seq".into(), V::Int(i as i64)); m.insert("name".into(), V::Text(n)); m })
                    .collect();
                return Ok((vec!["seq".into(), "name".into()], rows));
            }
            "pragma_compile_options" => {
                let rows = crate::compileoption_all().into_iter()
                    .map(|o| { let mut m = Row::new();
                        m.insert("compile_options".into(), V::Text(o.to_string())); m })
                    .collect();
                return Ok((vec!["compile_options".into()], rows));
            }
            "pragma_table_xinfo" => {
                // shape from the live snapshot (vtabs report their declared shape)
                let mut rows: Vec<Row> = Vec::new();
                let mk = |i: i64, n: String, ty: String, hidden: i64| -> Row {
                    let mut m = Row::new();
                    m.insert("cid".into(), V::Int(i)); m.insert("name".into(), V::Text(n));
                    m.insert("type".into(), V::Text(ty)); m.insert("notnull".into(), V::Int(0));
                    m.insert("dflt_value".into(), V::Null); m.insert("pk".into(), V::Int(0));
                    m.insert("hidden".into(), V::Int(hidden)); m
                };
                if let Some(shape) = crate::vtab_shape(ctx.db, &arg) {
                    for (i, (n, ty, hid)) in shape.into_iter().enumerate() {
                        rows.push(mk(i as i64, n, ty, hid as i64));
                    }
                } else if let Some((cols, _)) = ctx.tables.get(&arg) {
                    for (i, c) in cols.iter().enumerate() {
                        rows.push(mk(i as i64, c.clone(), String::new(), 0));
                    }
                }
                return Ok((vec!["cid".into(), "name".into(), "type".into(), "notnull".into(),
                                "dflt_value".into(), "pk".into(), "hidden".into()], rows));
            }
            "pragma_index_info" => {
                let mut rows: Vec<Row> = Vec::new();
                if let Some((_, tbl, cols, _)) = ctx.index_defs.iter().find(|(n, _, _, _)| *n == arg) {
                    let tcols: Vec<String> = ctx.tables.get(tbl).map(|(c, _)| c.clone()).unwrap_or_default();
                    for (seq, col) in cols.iter().enumerate() {
                        let cid = tcols.iter().position(|c| c == col).map(|p| p as i64).unwrap_or(-1);
                        let mut m = Row::new();
                        m.insert("seqno".into(), V::Int(seq as i64));
                        m.insert("cid".into(), V::Int(cid));
                        m.insert("name".into(), V::Text(col.clone()));
                        rows.push(m);
                    }
                }
                return Ok((vec!["seqno".into(), "cid".into(), "name".into()], rows));
            }
            "pragma_index_xinfo" => {
                // run-50: index_xinfo adds desc / coll / key and the trailing
                // rowid entry (cid -1, key 0) C appends for a rowid-table index
                let mut rows: Vec<Row> = Vec::new();
                if let Some((_, tbl, cols, sql)) = ctx.index_defs.iter().find(|(n, _, _, _)| *n == arg) {
                    let tcols: Vec<String> = ctx.tables.get(tbl).map(|(c, _)| c.clone()).unwrap_or_default();
                    let attrs = parse_index_col_attrs(sql);
                    for (seq, colraw) in cols.iter().enumerate() {
                        // the stored column expr may carry ASC/DESC/COLLATE; the bare
                        // name is the leading identifier
                        let col = colraw.split_whitespace().next().unwrap_or(colraw).to_string();
                        let cid = tcols.iter().position(|c| *c == col).map(|p| p as i64).unwrap_or(-1);
                        let (desc, coll) = attrs.get(seq).cloned().unwrap_or((false, "BINARY".into()));
                        let mut m = Row::new();
                        m.insert("seqno".into(), V::Int(seq as i64));
                        m.insert("cid".into(), V::Int(cid));
                        m.insert("name".into(), V::Text(col));
                        m.insert("desc".into(), V::Int(desc as i64));
                        m.insert("coll".into(), V::Text(coll));
                        m.insert("key".into(), V::Int(1));
                        rows.push(m);
                    }
                    // trailing rowid entry (unless the index already covers the rowid)
                    let mut m = Row::new();
                    m.insert("seqno".into(), V::Int(cols.len() as i64));
                    m.insert("cid".into(), V::Int(-1));
                    m.insert("name".into(), V::Null);
                    m.insert("desc".into(), V::Int(0));
                    m.insert("coll".into(), V::Text("BINARY".into()));
                    m.insert("key".into(), V::Int(0));
                    rows.push(m);
                }
                return Ok((vec!["seqno".into(), "cid".into(), "name".into(),
                                "desc".into(), "coll".into(), "key".into()], rows));
            }
            "completion" => {
                // run-50: LIVE C's phase contract — keywords (1), databases (7),
                // tables+views (8), table columns (9). The pragma/function/collation
                // phases are dead code in live C (pinned; ADR 0038). `phase` is a
                // HIDDEN column: queryable by name, absent from star-selects.
                let mut cands: Vec<(String, i64)> = SQL_KEYWORDS.iter()
                    .map(|k| (k.to_string(), 1)).collect();
                cands.push(("main".into(), 7));
                let mut objs: Vec<String> = ctx.tables.keys().cloned()
                    .chain(ctx.views.keys().cloned())
                    .filter(|n| !n.contains('.')).collect();
                objs.sort();
                for o in &objs { cands.push((o.clone(), 8)); }
                for (tn, (cols, _)) in ctx.tables.iter() {
                    if tn.contains('.') { continue; }
                    for c in cols { cands.push((c.clone(), 9)); } }
                let pfx = arg.to_ascii_lowercase();
                let rows = cands.into_iter()
                    .filter(|(c, _)| c.to_ascii_lowercase().starts_with(&pfx))
                    .map(|(c, ph)| { let mut m = Row::new();
                        m.insert("candidate".into(), V::Text(c));
                        m.insert("phase".into(), V::Int(ph)); m })
                    .collect();
                return Ok((vec!["candidate".into()], rows));
            }
            "pragma_function_list" => {
                let rows = FUNCTION_LIST.iter().map(|n| { let mut m = Row::new();
                    m.insert("name".into(), V::Text(n.to_string())); m }).collect();
                return Ok((vec!["name".into()], rows));
            }
            "pragma_pragma_list" => {
                let rows = PRAGMA_LIST.iter().map(|n| { let mut m = Row::new();
                    m.insert("name".into(), V::Text(n.to_string())); m }).collect();
                return Ok((vec!["name".into()], rows));
            }
            _ => return Err(format!("unsupported table source: {fname}")),
        }
    }
    // run-41: registered-module vtab instance — real cursor scan through
    // xOpen/xBestIndex/xFilter/xEof/xColumn/xNext/xClose. Rows carry HIDDEN columns
    // (selectable by name / usable in WHERE) but the visible column list drives SELECT *.
    {
        // run-48: reconnect-on-demand — a reopened file schema row or an eponymous-only
        // module materializes an instance (xConnect) before the scan; an unregistered
        // module surfaces C's exact error.
        let mut res = crate::vtab_scan(ctx.db, f);
        if res.is_none() {
            if let Some((_, module, args, sql)) = ctx.conn.vtab_schema.iter()
                .find(|(n, _, _, _)| n == f).cloned() {
                crate::vtab_connect_instance(ctx.db, f, &module, &args, &sql)?;
                res = crate::vtab_scan(ctx.db, f);
            } else if let Some(r) = crate::vtab_eponymous_connect(ctx.db, f) {
                r?;
                res = crate::vtab_scan(ctx.db, f);
            }
        }
        if let Some(res) = res {
            // the col list carries HIDDEN columns AND rowid (addressable by name / in
            // WHERE / ORDER BY); `SELECT *` expansion filters to the visible shape via
            // vtab_shape upstream, so neither leaks into star projections.
            let (_visible, mut all, rows, rowids) = res?;
            all.push("rowid".into());
            let rmaps = rows.into_iter().zip(rowids)
                .map(|(r, rid)| {
                    let mut m: Row = all.iter().cloned().zip(r.into_iter().chain(std::iter::once(V::Int(rid)))).collect();
                    m.insert("rowid".into(), V::Int(rid));
                    m
                })
                .collect();
            return Ok((all, rmaps));
        }
    }
    // run-38: wholenumber eponymous vtab — a bounded generator (needs a WHERE bound;
    // vtab-core general module system is a documented residual)
    if ctx.conn.vtabs.get(f).map(|m| m == "wholenumber").unwrap_or(false) {
        let bound = WN_BOUND.with(|c| c.get()).max(0);
        let rows = (1..=bound).map(|v| { let mut m = Row::new(); m.insert("value".into(), V::Int(v)); m }).collect();
        return Ok((vec!["value".into()], rows));
    }
    // run-49: pragma_module_list follows C's LAZY population — user modules from the
    // live registry plus the pragma vtabs this connection has actually touched
    if f.eq_ignore_ascii_case("pragma_module_list") {
        crate::pvtab_touch(ctx.db, "pragma_module_list"); // its own query instantiates it
        let rows = crate::module_list_names(ctx.db).into_iter()
            .map(|n| { let mut m = Row::new(); m.insert("name".into(), V::Text(n)); m })
            .collect();
        return Ok((vec!["name".into()], rows));
    }
    // run-44: bare (unparenthesized) registry TVF forms
    if f.eq_ignore_ascii_case("pragma_collation_list") {
        crate::pvtab_touch(ctx.db, "pragma_collation_list"); // run-49: lazy module_list fill
        let rows = crate::collation_list_names(ctx.db).into_iter().enumerate()
            .map(|(i, n)| { let mut m = Row::new();
                m.insert("seq".into(), V::Int(i as i64)); m.insert("name".into(), V::Text(n)); m })
            .collect();
        return Ok((vec!["seq".into(), "name".into()], rows));
    }
    if f.eq_ignore_ascii_case("pragma_compile_options") {
        crate::pvtab_touch(ctx.db, "pragma_compile_options"); // run-49: lazy module_list fill
        let rows = crate::compileoption_all().into_iter()
            .map(|o| { let mut m = Row::new();
                m.insert("compile_options".into(), V::Text(o.to_string())); m })
            .collect();
        return Ok((vec!["compile_options".into()], rows));
    }
    if f.eq_ignore_ascii_case("pragma_function_list") {
        crate::pvtab_touch(ctx.db, "pragma_function_list"); // run-49: lazy module_list fill
        let rows = FUNCTION_LIST.iter().map(|n| { let mut m = Row::new(); m.insert("name".into(), V::Text(n.to_string())); m }).collect();
        return Ok((vec!["name".into()], rows));
    }
    if f.eq_ignore_ascii_case("pragma_pragma_list") {
        crate::pvtab_touch(ctx.db, "pragma_pragma_list"); // run-49: lazy module_list fill
        let rows = PRAGMA_LIST.iter().map(|n| { let mut m = Row::new(); m.insert("name".into(), V::Text(n.to_string())); m }).collect();
        return Ok((vec!["name".into()], rows));
    }
    if f.eq_ignore_ascii_case("pragma_database_list") {
        crate::pvtab_touch(ctx.db, "pragma_database_list"); // run-49: lazy module_list fill
        let mut rows = Vec::new();
        let mut seq = 0i64;
        { let mut m=Row::new(); m.insert("seq".into(), V::Int(seq)); m.insert("name".into(), V::Text("main".into())); rows.push(m); }
        for a in &ctx.conn.attached { seq += 1; let mut m=Row::new(); m.insert("seq".into(), V::Int(seq)); m.insert("name".into(), V::Text(a.clone())); rows.push(m); }
        return Ok((vec!["seq".into(), "name".into()], rows));
    }
    // view: expand its stored SELECT (real re-execution, not a cache)
    if ctx.views.contains_key(f) && ctx.conn.pragmas.get("!view_off").copied().unwrap_or(0) != 0 {
        // run-47: DBCONFIG_ENABLE_VIEW off refuses view access with C's message
        return Err(format!("access to view \"{f}\" prohibited"));
    }
    if let Some(vsql) = ctx.views.get(f) {
        // run-50: TRUSTED_SCHEMA gates non-innocuous app functions inside views
        IN_VIEW.with(|d| d.set(d.get() + 1));
        let r = select_rows_o(ctx, vsql, &Row::new());
        IN_VIEW.with(|d| d.set(d.get() - 1));
        let (cols, rows) = r?;
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

fn is_agg(name: &str) -> bool { matches!(name.to_ascii_lowercase().trim_end_matches("#distinct"),
    "count"|"sum"|"total"|"avg"|"min"|"max"|"group_concat") }
fn expr_has_agg(e: &Ex) -> bool {
    match e { Ex::Func(n, a) => (is_agg(n)
            && !crate::auth_fn_ignored(n) // run-51: an IGNOREd aggregate stops aggregating
            && !(matches!(n.to_ascii_lowercase().as_str(), "min"|"max") && a.len() > 1)) || a.iter().any(expr_has_agg),
        Ex::Bin(_, x, y) | Ex::Is(x, y, _) => expr_has_agg(x) || expr_has_agg(y),
        Ex::Unary(_, x) | Ex::IsNull(x, _) | Ex::Cast(x, _) | Ex::Collate(x, _) => expr_has_agg(x),
        Ex::InList(x, xs) => expr_has_agg(x) || xs.iter().any(expr_has_agg),
        Ex::Like(x, y, _, _) => expr_has_agg(x) || expr_has_agg(y),
        Ex::Case(w, el) => w.iter().any(|(a,b)| expr_has_agg(a)||expr_has_agg(b)) || el.as_ref().map_or(false, |e| expr_has_agg(e)),
        Ex::Filtered(f, _) => expr_has_agg(f),
        _ => false }
}
fn expr_has_udf_agg(e: &Ex, ctx: &Ctx) -> bool {
    if ctx.db == 0 { return false; }
    match e {
        Ex::Func(n, a) => crate::udf_is_aggregate(ctx.db, &n.to_ascii_lowercase(), a.len())
            || a.iter().any(|x| expr_has_udf_agg(x, ctx)),
        Ex::Bin(_, x, y) | Ex::Is(x, y, _) => expr_has_udf_agg(x, ctx) || expr_has_udf_agg(y, ctx),
        Ex::Unary(_, x) | Ex::IsNull(x, _) | Ex::Cast(x, _) | Ex::Collate(x, _) => expr_has_udf_agg(x, ctx),
        Ex::InList(x, xs) => expr_has_udf_agg(x, ctx) || xs.iter().any(|y| expr_has_udf_agg(y, ctx)),
        Ex::Like(x, y, _, _) => expr_has_udf_agg(x, ctx) || expr_has_udf_agg(y, ctx),
        Ex::Case(w, el) => w.iter().any(|(a,b)| expr_has_udf_agg(a, ctx)||expr_has_udf_agg(b, ctx))
            || el.as_ref().map_or(false, |e| expr_has_udf_agg(e, ctx)),
        Ex::Filtered(f, _) => expr_has_udf_agg(f, ctx),
        _ => false,
    }
}

fn eval_agg(e: &Ex, rows: &[Row], ctx: &Ctx) -> Result<V, String> {
    // registered aggregate UDF: run xStep over the group then xFinal (run-28)
    if let Ex::Func(name, args) = e {
        let ln = name.to_ascii_lowercase();
        if ctx.db != 0 && crate::udf_is_aggregate(ctx.db, &ln, args.len()) {
            let mut arg_rows: Vec<Vec<V>> = Vec::with_capacity(rows.len());
            for r in rows {
                let mut vals = Vec::with_capacity(args.len());
                for a in args { vals.push(eval_expr(a, r, ctx)?); }
                arg_rows.push(vals);
            }
            if let Some(res) = crate::udf_invoke_aggregate(ctx.db, &ln, &arg_rows) {
                return res;
            }
        }
    }
    if let Ex::Filtered(f, w) = e {
        // aggregate FILTER (WHERE ...): restrict the input rows for real
        let mut keep = Vec::new();
        for r in rows { if eval_expr(w, r, ctx)?.truthy() == Some(true) { keep.push(r.clone()); } }
        return eval_agg(f, &keep, ctx);
    }
    if let Ex::Func(name, args) = e {
        let full = name.to_ascii_lowercase();
        let distinct = full.ends_with("#distinct");
        let ln = full.trim_end_matches("#distinct").to_string();
        if is_agg(&ln) && !(matches!(ln.as_str(), "min"|"max") && args.len() > 1) {
            let is_star = matches!(args.get(0), Some(Ex::Col(c)) if c == "*");
            let mut vals = Vec::new();
            if !is_star { for r in rows { let v = eval_expr(&args[0], r, ctx)?; if !matches!(v, V::Null) { vals.push(v); } } }
            if distinct {
                let mut seen = std::collections::HashSet::new();
                vals.retain(|v| seen.insert(format!("{:?}", v.render())));
            }
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
    // composite expressions over aggregates (e.g. HAVING sum(v) > 4): recurse
    match e {
        Ex::Bin(op, a, b) if expr_has_agg(e) => {
            let (x, y) = (eval_agg(a, rows, ctx)?, eval_agg(b, rows, ctx)?);
            let xe = Ex::Lit(match x { V::Null => LitV::Null, V::Int(i) => LitV::Int(i), V::Real(r) => LitV::Real(r), V::Text(t) => LitV::Str(t), V::Blob(b) => LitV::Blob(b) });
            let ye = Ex::Lit(match y { V::Null => LitV::Null, V::Int(i) => LitV::Int(i), V::Real(r) => LitV::Real(r), V::Text(t) => LitV::Str(t), V::Blob(b) => LitV::Blob(b) });
            eval_expr(&Ex::Bin(op.clone(), Box::new(xe), Box::new(ye)), &Row::new(), ctx)
        }
        // run-51: scalar functions OVER aggregates (coalesce(min(a),999)): compute
        // each argument in aggregate context, then apply the outer function
        Ex::Func(name, args) if expr_has_agg(e) => {
            let mut lit_args = Vec::with_capacity(args.len());
            for a in args {
                let v = eval_agg(a, rows, ctx)?;
                lit_args.push(Ex::Lit(match v {
                    V::Null => LitV::Null, V::Int(i) => LitV::Int(i), V::Real(r) => LitV::Real(r),
                    V::Text(t) => LitV::Str(t), V::Blob(b) => LitV::Blob(b),
                }));
            }
            eval_expr(&Ex::Func(name.clone(), lit_args), &Row::new(), ctx)
        }
        _ => eval_expr(e, rows.first().cloned().as_ref().unwrap_or(&Row::new()), ctx),
    }
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
    let qual = if !alias.is_empty() { alias } else { unquote_ident(&base) };
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
            // run-50: HIDDEN vtab/TVF columns ride along (queryable by name,
            // absent from the visible column list and star-selects)
            for (kk, vv) in r {
                if !cols.contains(kk) { m.entry(kk.clone()).or_insert_with(|| vv.clone()); }
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

/// parse a simple probe predicate "col = lit" / "col <op> lit" / "col BETWEEN a AND b"
/// (single top-level term, no AND/OR) -> (col, low_bound, high_bound), inclusive
fn simple_range_pred(w: &str) -> Option<(String, Option<V>, Option<V>)> {
    if find_kw_top(w, "AND").is_some() && find_kw_top(w, "BETWEEN").is_none() { return None; }
    if find_kw_top(w, "OR").is_some() { return None; }
    if let Some(p) = find_kw_top(w, "BETWEEN") {
        let col = w[..p].trim().to_string();
        let rest = &w[p + 7..];
        let ap = find_kw_top(rest, "AND")?;
        let lo = lit_val(rest[..ap].trim())?;
        let hi = lit_val(rest[ap + 3..].trim())?;
        if !is_ident(&col) { return None; }
        return Some((col, Some(lo), Some(hi)));
    }
    for (op, lob, hib) in [("<=", false, true), (">=", true, false), ("<", false, true), (">", true, false), ("=", true, true)] {
        if let Some(p) = w.find(op) {
            // avoid matching '<=' as '<'
            if op == "<" && w[p..].starts_with("<=") { continue; }
            if op == ">" && w[p..].starts_with(">=") { continue; }
            let col = w[..p].trim().to_string();
            let val = lit_val(w[p + op.len()..].trim())?;
            if !is_ident(&col) { return None; }
            let lo = if lob { Some(val.clone()) } else { None };
            let hi = if hib { Some(val) } else { None };
            return Some((col, lo, hi));
        }
    }
    None
}
fn is_ident(s: &str) -> bool { !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') }
fn lit_val(s: &str) -> Option<V> {
    let s = s.trim();
    if let Ok(i) = s.parse::<i64>() { return Some(V::Int(i)); }
    if let Ok(f) = s.parse::<f64>() { return Some(V::Real(f)); }
    if s.len() >= 2 && s.starts_with('\'') && s.ends_with('\'') { return Some(V::Text(s[1..s.len()-1].replace("''", "'"))); }
    None
}
fn key_num_or_text(k: &str) -> V {
    if let Ok(i) = k.parse::<i64>() { V::Int(i) } else if let Ok(f) = k.parse::<f64>() { V::Real(f) } else { V::Text(k.to_string()) }
}
fn in_range(v: &V, lo: &Option<V>, hi: &Option<V>) -> bool {
    if let Some(l) = lo { if vcmp(v, l) == std::cmp::Ordering::Less { return false; } }
    if let Some(h) = hi { if vcmp(v, h) == std::cmp::Ordering::Greater { return false; } }
    true
}

fn select_core(ctx: &Ctx, sql: &str, outer: &Row) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    let s = sql.trim();
    let up = s.to_ascii_uppercase();
    if !kw_bound(&up, "SELECT") { return Err("not a SELECT".into()); }
    let mut rest = s[6..].to_string();
    // GROUP BY (top level; ORDER BY/LIMIT already stripped by select_rows_o)
    let mut group_str: Option<String> = None;
    let mut having_str: Option<String> = None;
    if let Some(g) = find_kw_top(&rest, "GROUP BY") {
        let mut gtxt = rest[g+8..].trim().to_string();
        if let Some(h) = find_kw_top(&gtxt, "HAVING") {
            having_str = Some(gtxt[h+6..].trim().to_string());
            gtxt = gtxt[..h].trim().to_string();
        }
        group_str = Some(gtxt);
        rest = rest[..g].trim().to_string();
    }
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
    // run-50: SELECT DISTINCT dedupes the projected rows
    let (items_str, sel_distinct) = {
        let t = items_str.trim();
        let b = t.as_bytes();
        if b.len() > 8 && b[..8].eq_ignore_ascii_case(b"DISTINCT") && b[8].is_ascii_whitespace() {
            (t[8..].trim_start().to_string(), true)
        } else { (items_str.clone(), false) }
    };
    let mut items: Vec<(String, String)> = split_top(&items_str, ',').iter().map(|i| item_alias(i)).collect();
    // run-38: expand `SELECT *` over a single bare store table to its columns
    if items.iter().any(|(e, _)| e.trim() == "*") {
        if let Some(f) = &from_str {
            if let Some((cols, _)) = ctx.tables.get(f.trim()) {
                items = cols.iter().map(|c| (c.clone(), c.clone())).collect();
            } else if let Some(shape) = crate::vtab_shape(ctx.db, f.trim()) {
                // run-41: vtab star expands to the declared VISIBLE columns only
                items = shape.iter().filter(|(_, _, hidden)| !hidden)
                    .map(|(n, _, _)| (n.clone(), n.clone())).collect();
            } else if let Ok((cols, _)) = source_rows(ctx, f.trim()) {
                // run-50: TVF/view sources expand to their reported (visible) columns
                items = cols.iter().map(|c| (c.clone(), c.clone())).collect();
            }
        }
    }

    // window handling (two pinned shapes)
    if items.iter().any(|(e, _)| find_kw_top(e, "OVER").is_some()) {
        return window_select(ctx, &items, from_str.as_deref().unwrap_or(""), outer);
    }

    // real index probe: single bare store table + a simple indexed predicate ->
    // fetch candidate rows through the index b-tree map instead of a full scan.
    let mut src: Option<Vec<Row>> = None;
    if let (Some(f), Some(w)) = (&from_str, &where_str) {
        let tname = f.trim();
        if let (Some((cols, rows)), Some(idxs)) = (ctx.tables.get(tname), ctx.indexes.get(tname)) {
            if let Some((col, lo, hi)) = simple_range_pred(w) {
                if let Some((_c, map)) = idxs.iter().find(|(c, _)| *c == col) {
                    let mut hits: Vec<usize> = Vec::new();
                    for (k, positions) in map.iter() {
                        let kv = key_num_or_text(k);
                        if in_range(&kv, &lo, &hi) { hits.extend(positions.iter().copied()); }
                    }
                    ctx.probes.set(ctx.probes.get() + 1);
                    let mut rmaps = Vec::new();
                    for p in hits { if let Some(r) = rows.get(p) {
                        rmaps.push(cols.iter().cloned().zip(r.iter().cloned()).collect()); } }
                    src = Some(rmaps);
                }
            }
        }
    }
    // run-46: offer a simple `col = literal` predicate on a single-vtab FROM to the
    // module's xBestIndex (EQ pushdown; the engine still applies WHERE afterwards)
    let mut vtab_hint_set = false;
    if src.is_none() {
        if let (Some(f), Some(w)) = (&from_str, &where_str) {
            let fname = unquote_ident(f);
            if !fname.contains(char::is_whitespace) && !fname.starts_with('(') {
                if let Some(shape) = crate::vtab_shape(ctx.db, &fname) {
                    if let Some(eq) = w.find('=') {
                        let (l, r) = (w[..eq].trim(), w[eq + 1..].trim());
                        let lname = unquote_ident(l);
                        let rv: Option<V> = if let Ok(i) = r.parse::<i64>() { Some(V::Int(i)) }
                            else if r.len() >= 2 && r.starts_with('\'') && r.ends_with('\'') {
                                Some(V::Text(r[1..r.len()-1].replace("''", "'")))
                            } else { None };
                        if let (Some(v), Some(ci)) = (rv, shape.iter().position(|(n, _, _)| n.eq_ignore_ascii_case(&lname))) {
                            if !l.contains(|c: char| c.is_whitespace() || c == '(') {
                                crate::vtab_set_hint(&fname, ci, v);
                                vtab_hint_set = true;
                            }
                        }
                    }
                }
            }
        }
    }
    let src: Vec<Row> = match src {
        Some(s) => s,
        None => {
            let r = match &from_str { Some(f) => parse_from(ctx, f, outer), None => Ok(vec![Row::new()]) };
            if vtab_hint_set { crate::vtab_clear_hint(); }
            r?
        }
    };
    if vtab_hint_set { crate::vtab_clear_hint(); }
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
    let has_agg = exprs.iter().any(expr_has_agg) || exprs.iter().any(|e| expr_has_udf_agg(e, ctx));
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
        let having_ex = match &having_str { Some(h) => Some(parse_expr_full(h)?), None => None };
        for key in order {
            let rows = &groups[&key];
            if let Some(h) = &having_ex {
                if eval_agg(h, rows, ctx)?.truthy() != Some(true) { continue; }
            }
            let mut orow = Vec::new();
            for e in &exprs { orow.push(eval_agg(e, rows, ctx)?); }
            out.push(orow);
        }
    } else if has_agg {
        let env_rows: Vec<Row> = src.iter().map(|r| with_outer(r)).collect();
        let mut row = Vec::new();
        for e in &exprs { row.push(eval_agg(e, &env_rows, ctx)?); }
        out.push(row);
    } else {
        for r in &src { let env = with_outer(r); let mut orow = Vec::new();
            for e in &exprs { orow.push(eval_expr(e, &env, ctx)?); } out.push(orow); }
    }
    if sel_distinct {
        let mut seen = std::collections::HashSet::new();
        out.retain(|r| seen.insert(r.iter().map(|v| format!("{:?}", v.render())).collect::<Vec<_>>().join("\u{1}")));
    }
    Ok((colnames, out))
}

// ---------------- window functions (pack v14: general engine) ----------------
// row_number/rank/dense_rank/lag/lead + sum/min/max/avg/count OVER
// ([PARTITION BY ...] [ORDER BY ...] [ROWS|GROUPS|RANGE frame]).
// Values are computed per source row from real partitions/frames; output stays in
// source order (the outer ORDER BY sorts afterwards; stable sorts preserve ties).

struct WinSpec {
    part: Vec<Ex>,
    order: Vec<(Ex, bool)>, // (expr, desc)
    frame: WinFrame,
    exclude: WinExclude,    // run-47: EXCLUDE clause
}
enum WinFrame {
    RangePeers,             // default: RANGE UNBOUNDED PRECEDING .. CURRENT ROW (peer-inclusive)
    Rows(Option<i64>),      // ROWS BETWEEN <n|unbounded> PRECEDING AND CURRENT ROW
    RowsFull,               // ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING (run-33)
    Groups(i64),            // GROUPS BETWEEN n PRECEDING AND CURRENT ROW
    RowsBetween(i64, i64),  // run-47: ROWS BETWEEN n PRECEDING AND m FOLLOWING
    RangeOffset(i64, i64),  // run-47: RANGE BETWEEN n PRECEDING AND m FOLLOWING (numeric key)
}
#[derive(PartialEq, Clone, Copy)]
enum WinExclude { NoOthers, CurrentRow, Group, Ties }

fn parse_winspec(spec: &str) -> Result<WinSpec, String> {
    let mut rest = spec.trim().to_string();
    let mut part = Vec::new();
    let mut order = Vec::new();
    let mut frame = WinFrame::RangePeers;
    if let Some(p) = find_kw_top(&rest, "PARTITION BY") {
        let after = rest[p + 12..].trim().to_string();
        let stop = find_kw_top(&after, "ORDER BY")
            .or_else(|| find_kw_top(&after, "ROWS"))
            .or_else(|| find_kw_top(&after, "GROUPS"))
            .or_else(|| find_kw_top(&after, "RANGE"))
            .unwrap_or(after.len());
        for e in split_top(&after[..stop], ',') { part.push(parse_expr_full(&e)?); }
        rest = format!("{}{}", &rest[..p], &after[stop..]);
    }
    if let Some(p) = find_kw_top(&rest, "ORDER BY") {
        let after = rest[p + 8..].trim().to_string();
        let stop = find_kw_top(&after, "ROWS")
            .or_else(|| find_kw_top(&after, "GROUPS"))
            .or_else(|| find_kw_top(&after, "RANGE"))
            .unwrap_or(after.len());
        for term in split_top(&after[..stop], ',') {
            let mut words = term.split_whitespace();
            let name = words.next().unwrap_or("").to_string();
            let desc = words.next().map_or(false, |w| w.eq_ignore_ascii_case("DESC"));
            order.push((parse_expr_full(&name)?, desc));
        }
        rest = format!("{}{}", &rest[..p], &after[stop..]);
    }
    // run-47: EXCLUDE clause (stripped before frame parsing)
    let mut exclude = WinExclude::NoOthers;
    let mut up = rest.to_ascii_uppercase();
    if let Some(p) = up.find("EXCLUDE ") {
        let tail = up[p + 8..].trim().to_string();
        exclude = if tail.starts_with("CURRENT ROW") { WinExclude::CurrentRow }
            else if tail.starts_with("GROUP") { WinExclude::Group }
            else if tail.starts_with("TIES") { WinExclude::Ties }
            else { WinExclude::NoOthers };
        rest = rest[..p].trim_end().to_string();
        up = rest.to_ascii_uppercase();
    }
    if let Some(p) = up.find("ROWS BETWEEN ") {
        let body = rest[p + 13..].trim();
        let bu = body.to_ascii_uppercase();
        if bu.starts_with("UNBOUNDED PRECEDING") {
            frame = if bu.contains("AND UNBOUNDED FOLLOWING") { WinFrame::RowsFull } else { WinFrame::Rows(None) };
        }
        else if let Some(n) = body.split_whitespace().next().and_then(|w| w.parse::<i64>().ok()) {
            // run-47: n PRECEDING AND (CURRENT ROW | m FOLLOWING)
            if let Some(fp) = bu.find(" AND ") {
                let endb = bu[fp + 5..].trim().to_string();
                if endb.ends_with("FOLLOWING") || endb.contains("FOLLOWING") {
                    if let Some(m) = body[fp + 5..].trim().split_whitespace().next().and_then(|w| w.parse::<i64>().ok()) {
                        frame = WinFrame::RowsBetween(n, m);
                    } else { frame = WinFrame::Rows(Some(n)); }
                } else { frame = WinFrame::Rows(Some(n)); }
            } else { frame = WinFrame::Rows(Some(n)); }
        }
    } else if let Some(p) = up.find("GROUPS BETWEEN ") {
        let body = rest[p + 15..].trim();
        if let Some(n) = body.split_whitespace().next().and_then(|w| w.parse::<i64>().ok()) {
            frame = WinFrame::Groups(n);
        }
    } else if let Some(p) = up.find("RANGE BETWEEN ") {
        // run-47: offset RANGE — n PRECEDING AND m FOLLOWING over a numeric ORDER key
        let body = rest[p + 14..].trim();
        let bu = body.to_ascii_uppercase();
        if !bu.starts_with("UNBOUNDED") {
            if let (Some(n), Some(fp)) = (body.split_whitespace().next().and_then(|w| w.parse::<i64>().ok()),
                                          bu.find(" AND ")) {
                if let Some(m) = body[fp + 5..].trim().split_whitespace().next().and_then(|w| w.parse::<i64>().ok()) {
                    frame = WinFrame::RangeOffset(n, m);
                }
            }
        }
    }
    Ok(WinSpec { part, order, frame, exclude })
}

/// split "fn(args) OVER (spec)" -> (fname, args_text, spec_text)
fn split_over(item: &str) -> Option<(String, String, String)> {
    let p = find_kw_top(item, "OVER")?;
    let call = item[..p].trim();
    let spec = item[p + 4..].trim();
    let spec = spec.strip_prefix('(')?.strip_suffix(')')?.to_string();
    let op = call.find('(')?;
    let fname = call[..op].trim().to_ascii_lowercase();
    let args = call[op + 1..call.rfind(')')?].trim().to_string();
    Some((fname, args, spec))
}

fn window_select(ctx: &Ctx, items: &[(String, String)], from: &str, outer: &Row)
    -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    // run-33: WINDOW <name> AS (<spec>) [, ...] clause after FROM; OVER <name> resolves
    let mut from = from.to_string();
    let mut named: std::collections::HashMap<String, String> = Default::default();
    if let Some(wp) = find_kw_top(&from, "WINDOW") {
        let clause = from[wp + 6..].trim().to_string();
        from = from[..wp].trim().to_string();
        for def in split_top(&clause, ',') {
            if let Some(ap) = find_kw_top(&def, "AS") {
                let nm = def[..ap].trim().to_ascii_lowercase();
                let spec = def[ap + 2..].trim().trim_start_matches('(').trim_end_matches(')').to_string();
                named.insert(nm, spec);
            }
        }
    }
    let from = from.as_str();
    let src = parse_from(ctx, from, outer)?;
    let n = src.len();
    let mut out: Vec<Vec<V>> = vec![Vec::new(); n];
    for (expr, _alias) in items {
        if find_kw_top(expr, "OVER").is_none() {
            let ex = parse_expr_full(expr)?;
            for (i, r) in src.iter().enumerate() { out[i].push(eval_expr(&ex, r, ctx)?); }
            continue;
        }
        let (fname, args_txt, spec_txt) = match split_over(expr) {
            Some(x) => x,
            None => {
                // OVER <name> — look up the named window
                let p = find_kw_top(expr, "OVER").ok_or("bad window expression")?;
                let call = expr[..p].trim();
                let nm = expr[p + 4..].trim().trim_end_matches(')').trim().to_ascii_lowercase();
                let spec = named.get(&nm).cloned().ok_or("bad window expression")?;
                let op = call.find('(').ok_or("bad window expression")?;
                let fname = call[..op].trim().to_ascii_lowercase();
                let args = call[op + 1..call.rfind(')').ok_or("bad window expression")?].trim().to_string();
                (fname, args, spec)
            }
        };
        let spec = parse_winspec(&spec_txt)?;
        let arg_exs: Vec<Ex> = if args_txt.is_empty() || args_txt == "*" { Vec::new() }
            else { split_top(&args_txt, ',').iter().map(|a| parse_expr_full(a)).collect::<Result<_,_>>()? };
        // partition rows
        let mut partkeys: Vec<String> = Vec::with_capacity(n);
        for r in &src {
            let mut kv = Vec::new();
            for pe in &spec.part { kv.push(format!("{:?}", eval_expr(pe, r, ctx)?.render())); }
            partkeys.push(kv.join("\u{1}"));
        }
        let mut parts: std::collections::HashMap<String, Vec<usize>> = std::collections::HashMap::new();
        for i in 0..n { parts.entry(partkeys[i].clone()).or_default().push(i); }
        let mut vals: Vec<V> = vec![V::Null; n];
        for (_k, idxs) in parts {
            // stable sort by window ORDER BY keys
            let mut keys: Vec<Vec<V>> = Vec::new();
            for &i in &idxs {
                let mut kv = Vec::new();
                for (oe, _) in &spec.order { kv.push(eval_expr(oe, &src[i], ctx)?); }
                keys.push(kv);
            }
            let mut order: Vec<usize> = (0..idxs.len()).collect(); // positions into idxs
            order.sort_by(|&a, &b| {
                for (ki, (_, desc)) in spec.order.iter().enumerate() {
                    let o = vcmp(&keys[a][ki], &keys[b][ki]);
                    if o != std::cmp::Ordering::Equal { return if *desc { o.reverse() } else { o }; }
                }
                std::cmp::Ordering::Equal
            });
            let sorted: Vec<usize> = order.iter().map(|&p| idxs[p]).collect(); // source indices, window order
            let m = sorted.len();
            // peer groups (equal ORDER BY keys)
            let mut group_of = vec![0usize; m];
            let mut peer_end = vec![0usize; m];
            {
                let key_at = |p: usize| &keys[order[p]];
                let mut g = 0;
                let mut s = 0;
                while s < m {
                    let mut e = s;
                    while e + 1 < m && vcmp_vec(key_at(e + 1), key_at(s)) == std::cmp::Ordering::Equal { e += 1; }
                    for p in s..=e { group_of[p] = g; peer_end[p] = e; }
                    g += 1;
                    s = e + 1;
                }
            }
            // frame aggregate helper
            let arg_val = |p: usize| -> Result<V, String> {
                match arg_exs.first() { Some(e) => eval_expr(e, &src[sorted[p]], ctx), None => Ok(V::Int(1)) }
            };
            for p in 0..m {
                let i = sorted[p];
                let v = match fname.as_str() {
                    "row_number" => V::Int(p as i64 + 1),
                    "rank" => { let first = (0..m).find(|&q| group_of[q] == group_of[p]).unwrap_or(p); V::Int(first as i64 + 1) }
                    "dense_rank" => V::Int(group_of[p] as i64 + 1),
                    "lag" | "lead" => {
                        let off: i64 = arg_exs.get(1).map(|e| eval_expr(e, &src[i], ctx).map(|v| v.as_i64()))
                            .transpose()?.unwrap_or(1);
                        let q = if fname == "lag" { p as i64 - off } else { p as i64 + off };
                        if q < 0 || q >= m as i64 {
                            match arg_exs.get(2) { Some(d) => eval_expr(d, &src[i], ctx)?, None => V::Null }
                        } else {
                            match arg_exs.first() { Some(e) => eval_expr(e, &src[sorted[q as usize]], ctx)?, None => V::Null }
                        }
                    }
                    "sum" | "min" | "max" | "avg" | "count" => {
                        let (lo, hi) = match &spec.frame {
                            WinFrame::RangePeers => (0usize, peer_end[p]),
                            WinFrame::Rows(None) => (0usize, p),
                            WinFrame::RowsFull => (0usize, m - 1),
                            WinFrame::Rows(Some(k)) => ((p as i64 - k).max(0) as usize, p),
                            WinFrame::Groups(k) => {
                                let g0 = (group_of[p] as i64 - k).max(0);
                                let lo = (0..m).find(|&q| group_of[q] as i64 >= g0).unwrap_or(0);
                                (lo, peer_end[p])
                            }
                            // run-47: ROWS BETWEEN n PRECEDING AND m FOLLOWING
                            WinFrame::RowsBetween(k, j) =>
                                ((p as i64 - k).max(0) as usize, ((p as i64 + j).min(m as i64 - 1)) as usize),
                            // run-47: offset RANGE over the (numeric) first ORDER key —
                            // frame membership is by peer VALUE distance, not position
                            WinFrame::RangeOffset(k, j) => {
                                let kv = keys[order[p]].first().map(|v| v.as_f64()).unwrap_or(0.0);
                                let (lo_v, hi_v) = (kv - *k as f64, kv + *j as f64);
                                let mut lo = p; let mut hi = p;
                                while lo > 0 {
                                    let qv = keys[order[lo - 1]].first().map(|v| v.as_f64()).unwrap_or(0.0);
                                    if qv >= lo_v { lo -= 1; } else { break; }
                                }
                                while hi + 1 < m {
                                    let qv = keys[order[hi + 1]].first().map(|v| v.as_f64()).unwrap_or(0.0);
                                    if qv <= hi_v { hi += 1; } else { break; }
                                }
                                (lo, hi)
                            }
                        };
                        let mut acc: Vec<V> = Vec::new();
                        let mut rows_n = 0i64;
                        for q in lo..=hi {
                            // run-47: EXCLUDE clause removes rows from the frame
                            let excluded = match spec.exclude {
                                WinExclude::NoOthers => false,
                                WinExclude::CurrentRow => q == p,
                                WinExclude::Group => group_of[q] == group_of[p],
                                WinExclude::Ties => group_of[q] == group_of[p] && q != p,
                            };
                            if excluded { continue; }
                            rows_n += 1;
                            let av = arg_val(q)?;
                            if !matches!(av, V::Null) { acc.push(av); }
                        }
                        match fname.as_str() {
                            "count" => { if arg_exs.is_empty() { V::Int(rows_n) } else { V::Int(acc.len() as i64) } }
                            "sum" => { if acc.is_empty() { V::Null }
                                else if acc.iter().all(|v| matches!(v, V::Int(_))) { V::Int(acc.iter().map(|v| v.as_i64()).sum()) }
                                else { V::Real(acc.iter().map(|v| v.as_f64()).sum()) } }
                            "avg" => { if acc.is_empty() { V::Null } else { V::Real(acc.iter().map(|v| v.as_f64()).sum::<f64>() / acc.len() as f64) } }
                            "min" => acc.into_iter().min_by(vcmp).unwrap_or(V::Null),
                            _ => acc.into_iter().max_by(vcmp).unwrap_or(V::Null),
                        }
                    }
                    // ---- run-33 window leftovers (COVERAGE-named residuals) ----
                    "first_value" | "last_value" | "nth_value" => {
                        // frame end mirrors the aggregate frame rules
                        let hi = match &spec.frame {
                            WinFrame::RangePeers => peer_end[p],
                            WinFrame::Rows(None) => p,
                            WinFrame::RowsFull => m - 1,
                            WinFrame::Rows(Some(k)) => { let _ = k; p }
                            WinFrame::Groups(_) => peer_end[p],
                            WinFrame::RowsBetween(_, j) => ((p as i64 + j).min(m as i64 - 1)) as usize,
                            WinFrame::RangeOffset(_, _) => peer_end[p],
                        };
                        let lo = match &spec.frame { WinFrame::Rows(Some(k)) => (p as i64 - k).max(0) as usize, _ => 0 };
                        let pos: Option<usize> = match fname.as_str() {
                            "first_value" => Some(lo),
                            "last_value" => Some(hi),
                            _ => {
                                let k = arg_exs.get(1).map(|e| eval_expr(e, &src[i], ctx).map(|v| v.as_i64()))
                                    .transpose()?.unwrap_or(1);
                                if k >= 1 && lo as i64 + k - 1 <= hi as i64 { Some(lo + k as usize - 1) } else { None }
                            }
                        };
                        match pos {
                            Some(q) => match arg_exs.first() { Some(e) => eval_expr(e, &src[sorted[q]], ctx)?, None => V::Null },
                            None => V::Null,
                        }
                    }
                    "ntile" => {
                        // SQLite: earlier buckets get the extra rows (size ceil then floor)
                        let nb = arg_exs.first().map(|e| eval_expr(e, &src[i], ctx).map(|v| v.as_i64()))
                            .transpose()?.unwrap_or(1).max(1);
                        let (mi, nbi) = (m as i64, nb);
                        let big = mi % nbi;              // buckets with (m/nb + 1) rows
                        let sz_big = mi / nbi + 1;
                        let sz_small = mi / nbi;
                        let p64 = p as i64;
                        let v = if p64 < big * sz_big { p64 / sz_big + 1 }
                                else if sz_small > 0 { big + (p64 - big * sz_big) / sz_small + 1 }
                                else { p64 + 1 };
                        V::Int(v)
                    }
                    "percent_rank" => {
                        if m <= 1 { V::Real(0.0) } else {
                            let first = (0..m).find(|&q| group_of[q] == group_of[p]).unwrap_or(p);
                            V::Real(first as f64 / (m - 1) as f64)
                        }
                    }
                    "cume_dist" => V::Real((peer_end[p] + 1) as f64 / m as f64),
                    other => return Err(format!("unsupported window function: {other}")),
                };
                vals[i] = v;
            }
        }
        for i in 0..n { out[i].push(vals[i].clone()); }
    }
    Ok((items.iter().map(|(_, a)| a.clone()).collect(), out))
}

fn vcmp_vec(a: &Vec<V>, b: &Vec<V>) -> std::cmp::Ordering {
    for i in 0..a.len().min(b.len()) {
        let o = vcmp(&a[i], &b[i]);
        if o != std::cmp::Ordering::Equal { return o; }
    }
    std::cmp::Ordering::Equal
}

thread_local! {
    // run-38: bound for the wholenumber vtab generator (max integer literal in the SQL)
    static WN_BOUND: std::cell::Cell<i64> = const { std::cell::Cell::new(0) };
}
// ---------------- run-52: WITH / WITH RECURSIVE (C's Queue/Current machine) ----------------

thread_local! {
    // materialized CTE tables visible to source_rows, as a stack of WITH scopes
    static CTE_SCOPES: std::cell::RefCell<Vec<std::collections::HashMap<String, (Vec<String>, Vec<Vec<V>>)>>> =
        const { std::cell::RefCell::new(Vec::new()) };
}
/// CTE lookup consulted FIRST by source_rows (a CTE shadows a real table, like C)
fn cte_lookup(name: &str) -> Option<(Vec<String>, Vec<Vec<V>>)> {
    CTE_SCOPES.with(|s| {
        for scope in s.borrow().iter().rev() {
            if let Some(hit) = scope.get(&name.to_ascii_lowercase()) { return Some(hit.clone()); }
        }
        None
    })
}
fn cte_scope_set(name: &str, cols: Vec<String>, rows: Vec<Vec<V>>) {
    CTE_SCOPES.with(|s| {
        if let Some(top) = s.borrow_mut().last_mut() {
            top.insert(name.to_ascii_lowercase(), (cols, rows));
        }
    });
}
struct CteScopeGuard;
impl Drop for CteScopeGuard {
    fn drop(&mut self) { CTE_SCOPES.with(|s| { s.borrow_mut().pop(); }); }
}

struct CteDef { name: String, cols: Option<Vec<String>>, body: String }

/// parse `WITH [RECURSIVE] name[(cols)] AS [NOT] [MATERIALIZED] (body), ... <main>`
fn parse_with(sql: &str) -> Result<(bool, Vec<CteDef>, String), String> {
    let s = sql.trim();
    let mut rest = s["WITH".len()..].trim_start().to_string();
    let mut recursive = false;
    if rest.len() >= 9 && rest.as_bytes()[..9].eq_ignore_ascii_case(b"RECURSIVE")
        && rest.as_bytes().get(9).is_none_or(|b| b.is_ascii_whitespace()) {
        recursive = true;
        rest = rest[9..].trim_start().to_string();
    }
    let mut defs: Vec<CteDef> = Vec::new();
    loop {
        // name
        let name_end = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(rest.len());
        let name = rest[..name_end].to_string();
        if name.is_empty() { return Err("near \"WITH\": syntax error".into()); }
        rest = rest[name_end..].trim_start().to_string();
        // optional (cols)
        let mut cols: Option<Vec<String>> = None;
        if rest.starts_with('(') {
            let close = matching_paren(&rest, 0).ok_or("unbalanced parentheses")?;
            cols = Some(rest[1..close].split(',').map(|c| unquote_ident(c.trim())).collect());
            rest = rest[close + 1..].trim_start().to_string();
        }
        // AS [NOT] [MATERIALIZED]
        let up = rest.to_ascii_uppercase();
        if !up.starts_with("AS") { return Err("near \"AS\": syntax error".into()); }
        rest = rest[2..].trim_start().to_string();
        let up = rest.to_ascii_uppercase();
        if up.starts_with("NOT MATERIALIZED") { rest = rest["NOT MATERIALIZED".len()..].trim_start().to_string(); }
        else if up.starts_with("MATERIALIZED") { rest = rest["MATERIALIZED".len()..].trim_start().to_string(); }
        if !rest.starts_with('(') { return Err("near \"(\": syntax error".into()); }
        let close = matching_paren(&rest, 0).ok_or("unbalanced parentheses")?;
        let body = rest[1..close].trim().to_string();
        rest = rest[close + 1..].trim_start().to_string();
        defs.push(CteDef { name, cols, body });
        if rest.starts_with(',') { rest = rest[1..].trim_start().to_string(); continue; }
        break;
    }
    Ok((recursive, defs, rest))
}
fn matching_paren(s: &str, open: usize) -> Option<usize> {
    let b = s.as_bytes();
    let mut depth = 0i32;
    let mut inq = false;
    for (i, &c) in b.iter().enumerate().skip(open) {
        match c {
            b'\'' => inq = !inq,
            b'(' if !inq => depth += 1,
            b')' if !inq => { depth -= 1; if depth == 0 { return Some(i); } }
            _ => {}
        }
    }
    None
}
/// run-52: lightweight WITH parse for the authorizer walk: (recursive, [(name, body)], main)
pub fn parse_with_pub(sql: &str) -> Result<(bool, Vec<(String, String)>, String), String> {
    let (r, defs, main) = parse_with(sql)?;
    Ok((r, defs.into_iter().map(|d| (d.name, d.body)).collect(), main))
}
pub fn refs_name_pub(sql: &str, name: &str) -> bool { refs_name(sql, name) }
pub fn split_union_pub(body: &str) -> (Vec<String>, Vec<bool>) { split_union(body) }

/// does `sql` reference `name` as a word (case-insensitive)?
pub(crate) fn refs_name(sql: &str, name: &str) -> bool {
    sql.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .any(|w| w.eq_ignore_ascii_case(name))
}
/// split a compound body on top-level UNION [ALL]; returns (terms, all_flags between)
fn split_union(body: &str) -> (Vec<String>, Vec<bool>) {
    let mut terms = Vec::new();
    let mut alls = Vec::new();
    let mut rest = body.trim().to_string();
    loop {
        match find_kw_top(&rest, "UNION") {
            Some(p) => {
                terms.push(rest[..p].trim().to_string());
                let after = rest[p + 5..].trim_start();
                if after.len() >= 3 && after.as_bytes()[..3].eq_ignore_ascii_case(b"ALL")
                    && after.as_bytes().get(3).is_none_or(|b| b.is_ascii_whitespace()) {
                    alls.push(true);
                    rest = after[3..].trim_start().to_string();
                } else {
                    alls.push(false);
                    rest = after.to_string();
                }
            }
            None => { terms.push(rest.trim().to_string()); break; }
        }
    }
    (terms, alls)
}
fn row_key(r: &[V]) -> String {
    r.iter().map(|v| format!("{:?}", v.render())).collect::<Vec<_>>().join("\u{1}")
}

/// evaluate a WITH statement: materialize used CTEs (recursive ones through C's
/// Queue/Current FIFO), then run the main statement against the scope.
fn eval_with(ctx: &Ctx, sql: &str, outer: &Row) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    let (recursive, defs, main) = parse_with(sql)?;
    if main.is_empty() { return Err("near \";\": syntax error".into()); }
    CTE_SCOPES.with(|s| s.borrow_mut().push(Default::default()));
    let _guard = CteScopeGuard;
    // LIMIT cap for an unbounded recursion: only the simple `SELECT ... FROM cte LIMIT n`
    // outer form (no WHERE / GROUP / ORDER) can soundly stop the machine early
    let mut cap: Option<usize> = None;
    {
        let mup = main.to_ascii_uppercase();
        if let Some(lp) = find_kw_top(&main, "LIMIT") {
            if !mup.contains(" WHERE ") && !mup.contains("GROUP BY") && !mup.contains("ORDER BY") {
                if let Ok(n) = main[lp + 5..].trim().trim_end_matches(';').parse::<usize>() {
                    cap = Some(n);
                }
            }
        }
    }
    // materialize the CTEs the main statement (transitively) uses, dependency-first
    fn materialize(ctx: &Ctx, defs: &[CteDef], name: &str, recursive_kw: bool,
                   in_progress: &mut Vec<String>, outer: &Row, cap: Option<usize>) -> Result<(), String> {
        if cte_lookup(name).is_some() { return Ok(()); }
        if in_progress.iter().any(|n| n.eq_ignore_ascii_case(name)) {
            return Err(format!("circular reference: {name}"));
        }
        let def = match defs.iter().find(|d| d.name.eq_ignore_ascii_case(name)) {
            Some(d) => d, None => return Ok(()), // a real table, not a CTE
        };
        in_progress.push(name.to_string());
        let (terms, alls) = split_union(&def.body);
        let self_recursive = recursive_kw && terms.len() > 1
            && refs_name(terms.last().unwrap(), &def.name);
        // dependencies of every term (other CTEs) first
        for t in &terms {
            for d in defs {
                if !d.name.eq_ignore_ascii_case(&def.name) && refs_name(t, &d.name) {
                    materialize(ctx, defs, &d.name, recursive_kw, in_progress, outer, None)?;
                }
            }
        }
        let result: (Vec<String>, Vec<Vec<V>>) = if self_recursive {
            let rec_term = terms.last().unwrap().clone();
            let distinct = !alls.last().copied().unwrap_or(true);
            // C's compile-time refusals (probed messages)
            let rup = rec_term.to_ascii_uppercase();
            let from_part = match rup.find(" FROM ") {
                Some(p) => {
                    let tail = &rec_term[p..];
                    let end = tail.to_ascii_uppercase().find(" WHERE ").unwrap_or(tail.len());
                    tail[..end].to_string()
                }
                None => String::new(),
            };
            // count self-references used as TABLE SOURCES (a `name.` column
            // qualifier is not a second reference)
            let self_refs = {
                let fb = from_part.as_bytes();
                let mut n = 0usize;
                let mut i = 0;
                while i < fb.len() {
                    if (fb[i] as char).is_ascii_alphanumeric() || fb[i] == b'_' {
                        let st_i = i;
                        while i < fb.len() && ((fb[i] as char).is_ascii_alphanumeric() || fb[i] == b'_') { i += 1; }
                        let prev_dot = st_i > 0 && fb[st_i - 1] == b'.';
                        let next_dot = i < fb.len() && fb[i] == b'.';
                        if !prev_dot && !next_dot && from_part[st_i..i].eq_ignore_ascii_case(&def.name) {
                            n += 1;
                        }
                    } else { i += 1; }
                }
                n
            };
            if self_refs > 1 {
                return Err(format!("multiple references to recursive table: {}", def.name));
            }
            {
                let sel_end = rup.find(" FROM ").unwrap_or(rec_term.len());
                let items = &rec_term[6..sel_end];
                for item in split_top(items, ',') {
                    if let Ok(ex) = parse_expr_full(&item) {
                        if expr_has_agg(&ex) {
                            return Err("recursive aggregate queries not supported".into());
                        }
                    }
                }
            }
            // seed = the terms before the recursive member (UNION ALL concatenation)
            let mut seed_cols: Vec<String> = Vec::new();
            let mut queue: std::collections::VecDeque<Vec<V>> = Default::default();
            let mut seen: std::collections::HashSet<String> = Default::default();
            for (i, t) in terms[..terms.len() - 1].iter().enumerate() {
                let (c, rs) = select_rows_o(ctx, t, outer)?;
                if i == 0 {
                    if let Some(decl) = &def.cols {
                        if decl.len() != c.len() {
                            return Err(format!("table {} has {} values for {} columns",
                                def.name, c.len(), decl.len()));
                        }
                    }
                    seed_cols = c;
                }
                for r in rs {
                    if distinct && !seen.insert(row_key(&r)) { continue; }
                    queue.push_back(r);
                }
            }
            let colnames: Vec<String> = def.cols.clone().unwrap_or(seed_cols);
            // the Queue/Current machine: extract ONE row as the recursive table,
            // run the recursive member against it, enqueue its outputs FIFO
            let mut out: Vec<Vec<V>> = Vec::new();
            while let Some(cur) = queue.pop_front() {
                out.push(cur.clone());
                if let Some(c) = cap { if out.len() >= c { break; } }
                cte_scope_set(&def.name, colnames.clone(), vec![cur]);
                let (_c, rs) = select_rows_o(ctx, &rec_term, outer)?;
                for r in rs {
                    if distinct && !seen.insert(row_key(&r)) { continue; }
                    queue.push_back(r);
                }
            }
            (colnames, out)
        } else {
            let (c, rows) = select_rows_o(ctx, &def.body, outer)?;
            if let Some(decl) = &def.cols {
                if decl.len() != c.len() {
                    return Err(format!("table {} has {} values for {} columns",
                        def.name, c.len(), decl.len()));
                }
            }
            (def.cols.clone().unwrap_or(c), rows)
        };
        in_progress.pop();
        cte_scope_set(&def.name, result.0, result.1);
        Ok(())
    }
    let mut in_progress: Vec<String> = Vec::new();
    for d in &defs {
        if refs_name(&main, &d.name) {
            materialize(ctx, &defs, &d.name, recursive, &mut in_progress, outer, cap)?;
        }
    }
    select_rows_o(ctx, &main, outer)
}

fn select_rows_o(ctx: &Ctx, sql: &str, outer: &Row) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    // run-52: WITH / WITH RECURSIVE route through the CTE machine
    {
        let t = sql.trim();
        if t.len() > 4 && t.as_bytes()[..4].eq_ignore_ascii_case(b"WITH")
            && t.as_bytes()[4].is_ascii_whitespace() {
            return eval_with(ctx, t, outer);
        }
    }
    // pick up the largest integer literal so a wholenumber source can bound itself
    { let mut mx = 0i64; let b = sql.as_bytes(); let mut i = 0;
      while i < b.len() {
          if b[i].is_ascii_digit() && (i == 0 || !(b[i-1] as char).is_alphanumeric() && b[i-1] != b'_') {
              let st = i; while i < b.len() && b[i].is_ascii_digit() { i += 1; }
              if let Ok(v) = sql[st..i].parse::<i64>() { if v > mx { mx = v; } }
          } else { i += 1; }
      }
      WN_BOUND.with(|c| c.set(mx)); }
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
        // ORDER BY term: <name|ordinal> [COLLATE c] [ASC|DESC] [NULLS FIRST|LAST]
        struct OKey { ci: usize, desc: bool, coll: Option<String>, nulls_first: Option<bool> }
        let mut keys: Vec<OKey> = Vec::new();
        for term in split_top(&ob, ',') {
            let words: Vec<String> = term.split_whitespace().map(|w| w.to_string()).collect();
            let name = words.first().cloned().unwrap_or_else(|| "1".into());
            let mut desc = false; let mut coll: Option<String> = None; let mut nulls_first = None;
            let mut wi = 1;
            while wi < words.len() {
                let w = words[wi].to_ascii_uppercase();
                match w.as_str() {
                    "COLLATE" => { if wi + 1 < words.len() { coll = Some(words[wi+1].trim_matches('"').to_ascii_lowercase()); wi += 1; } }
                    "ASC" => desc = false,
                    "DESC" => desc = true,
                    "NULLS" => { if wi + 1 < words.len() { nulls_first = Some(words[wi+1].eq_ignore_ascii_case("FIRST")); wi += 1; } }
                    _ => {}
                }
                wi += 1;
            }
            // run-40: an ORDER BY name that isn't a projected column or ordinal is
            // skipped (stable) instead of defaulting to column 0 (which mis-sorted
            // e.g. `SELECT name FROM pragma_database_list ORDER BY seq`)
            let ci = match name.parse::<usize>() {
                Ok(n) => Some(n - 1),
                Err(_) => colnames.iter().position(|c| *c == name)
                    .or_else(|| colnames.iter().position(|c| c.rsplit('.').next() == name.rsplit('.').next())),
            };
            let ci = match ci { Some(ci) => ci, None => continue };
            // declared column collation applies when no explicit COLLATE is given
            if coll.is_none() {
                if let Some(cn) = colnames.get(ci) {
                    coll = ctx.col_colls.get(&cn.rsplit('.').next().unwrap_or(cn).to_ascii_lowercase()).cloned();
                }
            }
            // resolve custom collations up front so unknown names error like C prepare
            if let Some(name) = coll.as_deref() {
                if !matches!(name, "uint" | "rot13" | "nocase" | "decimal" | "rtrim" | "binary") && !crate::coll_user_exists(ctx.db, name) {
                    return Err(format!("no such collation sequence: {name}"));
                }
            }
            keys.push(OKey { ci, desc, coll, nulls_first });
        }
        rows.sort_by(|a, b| {
            use std::cmp::Ordering::*;
            for k in &keys {
                let (x, y) = (a.get(k.ci).unwrap_or(&V::Null), b.get(k.ci).unwrap_or(&V::Null));
                let (nx, ny) = (matches!(x, V::Null), matches!(y, V::Null));
                if nx || ny {
                    if nx && ny { continue; }
                    // SQLite default: NULLs first ASC, last DESC; explicit NULLS overrides
                    let first = k.nulls_first.unwrap_or(!k.desc);
                    return if nx { if first { Less } else { Greater } }
                           else { if first { Greater } else { Less } };
                }
                let o = match k.coll.as_deref() {
                    Some(name) => coll_order(ctx, name, x, y).unwrap_or_else(|_| vcmp(x, y)),
                    None => vcmp(x, y),
                };
                if o != Equal { return if k.desc { o.reverse() } else { o }; }
            }
            Equal
        });
    }
    if let Some((n, off)) = limit {
        rows = rows.into_iter().skip(off).take(n).collect();
    }
    Ok((colnames, rows))
}

/// typed SELECT for the statement API: same executor, rows stay typed
pub fn stmt_select_typed(ctx: &mut Ctx, sql: &str) -> Result<(Vec<String>, Vec<Vec<V>>), String> {
    select_rows_o(ctx, sql, &Row::new())
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
    if kw_bound(&up, "SELECT") || kw_bound(&up, "WITH") {
        let (_c, rows) = select_rows_o(ctx, s, &Row::new())?;
        return Ok(Some(rows.into_iter().map(|r| r.into_iter().map(|v| v.render()).collect()).collect()));
    }
    Ok(None)
}

fn run_pragma(ctx: &mut Ctx, body: &str) -> Result<Vec<Vec<Option<String>>>, String> {
    let b = body.trim();
    let (name, val) = match b.split_once('=') { Some((n, v)) => (n.trim().to_ascii_lowercase(), Some(v.trim().to_string())), None => (b.trim().to_ascii_lowercase(), None) };
    let boolval = |v: &str| -> i64 { match v.to_ascii_uppercase().as_str() { "ON"|"TRUE"|"YES" => 1, "OFF"|"FALSE"|"NO" => 0, _ => v.parse().unwrap_or(0) } };
    // run-53: PRAGMA index_list(t) / foreign_key_list(t) share the TVF projection
    if name.starts_with("index_list(") || name.starts_with("foreign_key_list(") {
        let arg = name[name.find('(').unwrap() + 1..].trim_end_matches(')').trim().trim_matches('\'').to_string();
        let rows = if name.starts_with("index_list") {
            ctx.index_list_proj.get(&arg).cloned().unwrap_or_default()
        } else {
            ctx.fk_list_proj.get(&arg).cloned().unwrap_or_default()
        };
        return Ok(rows);
    }
    match name.as_str() {
        n if n.starts_with("wal_checkpoint") => {
            // counts row (busy, log, checkpointed); backfill runs in the post-exec sync
            if !ctx.conn.is_file || ctx.conn.journal != "wal" {
                return Ok(vec![vec![Some("0".into()), Some("-1".into()), Some("-1".into())]]);
            }
            let arg = n["wal_checkpoint".len()..].trim().trim_start_matches('(').trim_end_matches(')').trim().to_ascii_uppercase();
            let mode = if arg.is_empty() { "PASSIVE".to_string() } else { arg };
            let nf = crate::store::wal_frame_count(ctx.db);
            ctx.conn.pending_ckpt = Some(mode);
            Ok(vec![vec![Some("0".into()), Some(nf.to_string()), Some(nf.to_string())]])
        }
        "page_count" => {
            // freelist model: deleted pages stay counted until VACUUM rebuilds (run-34)
            let v = ctx.conn.page_hwm.max(ctx.conn.page_cur).max(1);
            Ok(vec![vec![Some(v.to_string())]])
        }
        "integrity_check" | "quick_check" => Ok(vec![vec![Some("ok".into())]]),
        "encoding" => Ok(if val.is_none() { vec![vec![Some("UTF-8".into())]] } else { vec![] }),
        "journal_mode" => {
            // v22: real WAL slice for file connections; :memory: refuses WAL like C
            if !ctx.conn.is_file { return Ok(vec![vec![Some("memory".into())]]); }
            if let Some(v) = &val {
                match v.trim().to_ascii_lowercase().as_str() {
                    "wal" => ctx.conn.journal = "wal".into(),
                    "delete" => ctx.conn.journal = "delete".into(),
                    _ => {} // other modes keep current (frozen scope: wal + delete)
                }
            }
            let mode = if ctx.conn.journal == "wal" { "wal" } else { "delete" };
            Ok(vec![vec![Some(mode.into())]])
        }
        "locking_mode" => Ok(vec![vec![Some("normal".into())]]),
        // run-46: page_size / auto_vacuum sets are PENDING until VACUUM applies them (C)
        "page_size" => { match val { Some(v) => { ctx.conn.pragmas.insert(name.clone(), boolval(&v)); Ok(vec![]) }
            None => { let cur = *ctx.conn.pragmas.get("page_size#active").unwrap_or(&4096); Ok(vec![vec![Some(cur.to_string())]]) } } }
        "auto_vacuum" => { match val { Some(v) => { ctx.conn.pragmas.insert(name.clone(), boolval(&v)); Ok(vec![]) }
            None => { let cur = *ctx.conn.pragmas.get("auto_vacuum#active").unwrap_or(&0); Ok(vec![vec![Some(cur.to_string())]]) } } }
        "busy_timeout" => { match val {
            Some(v) => { let n = boolval(&v); ctx.conn.pragmas.insert(name.clone(), n); Ok(vec![vec![Some(n.to_string())]]) } // set RETURNS the value
            None => { let cur = *ctx.conn.pragmas.get(&name).unwrap_or(&0); Ok(vec![vec![Some(cur.to_string())]]) } } }
        "writable_schema" => { match val {
            Some(v) => {
                // run-50: DEFENSIVE makes this a silent no-op (pinned)
                if !crate::defensive_on(ctx.db) {
                    ctx.conn.pragmas.insert("!writable_schema".into(), boolval(&v));
                }
                Ok(vec![])
            }
            None => { let cur = *ctx.conn.pragmas.get("!writable_schema").unwrap_or(&0); Ok(vec![vec![Some(cur.to_string())]]) } } }
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
        // run-44: own writes do not bump; picking up a sibling's commit does (store hook)
        "data_version" => Ok(vec![vec![Some((1 + ctx.conn.data_version).to_string())]]),
        "freelist_count" => Ok(vec![vec![Some((ctx.conn.page_hwm - ctx.conn.page_cur).max(0).to_string())]]),
        "collation_list" => {
            // live registry: user registrations newest-first, then RTRIM/NOCASE/BINARY (C order)
            Ok(crate::collation_list_names(ctx.db).into_iter().enumerate()
                .map(|(i, n)| vec![Some(i.to_string()), Some(n)]).collect())
        }
        "case_sensitive_like" => { if let Some(v) = val { ctx.conn.case_sensitive_like = boolval(&v) != 0; } Ok(vec![]) }
        "user_version" | "application_id" | "cache_size" | "recursive_triggers" | "defer_foreign_keys"
        | "query_only" | "temp_store" | "automatic_index" | "ignore_check_constraints" => {
            match val {
                Some(v) => { ctx.conn.pragmas.insert(name.clone(), boolval(&v)); Ok(vec![]) }
                None => { let cur = *ctx.conn.pragmas.get(&name).unwrap_or(&Conn::pragma_default(&name)); Ok(vec![vec![Some(cur.to_string())]]) }
            }
        }
        // run-44: C silently ignores unknown pragma names (get and set forms)
        _ => Ok(vec![]),
    }
}
