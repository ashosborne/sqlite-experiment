//! pack v8: the behavioural cheat sheet is GONE. sqlite3_exec computes answers via
//! the store (kitchen) + eval (expressions/pragmas/functions). This table is empty.
pub struct ScriptPin { pub rc: i32, pub rows: &'static [&'static [Option<&'static str>]], pub errmsg: bool }
pub static SCRIPT_TABLE: &[(&str, ScriptPin)] = &[];
