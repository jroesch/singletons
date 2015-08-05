#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(unboxed_closures)]
#![feature(core)]

extern crate syntax;
extern crate rustc;
#[macro_use] extern crate lazy_static;

mod bijection;

use bijection::Bijection;

use std::sync::Mutex;

use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::{TokenTree, TtToken};
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use rustc::plugin::Registry;

lazy_static! {
    static ref SINGLETON_MAP: Mutex<Bijection<i32, i32>> = Mutex::new(Bijection::new(|x| x + 1));
}

fn expand_singleton_type(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<MacResult + 'static> {
    panic!()
}

fn expand_singleton_value(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<MacResult + 'static> {
    panic!()
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("Singleton", expand_singleton_type);
    reg.register_macro("singleton", expand_singleton_value);
}
