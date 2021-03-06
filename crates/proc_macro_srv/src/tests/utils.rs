//! utils used in proc-macro tests

use crate::dylib;
use crate::ProcMacroSrv;
use expect_test::Expect;
use proc_macro_api::ListMacrosTask;
use std::str::FromStr;

pub mod fixtures {
    pub fn proc_macro_test_dylib_path() -> std::path::PathBuf {
        proc_macro_test::PROC_MACRO_TEST_LOCATION.into()
    }
}

fn parse_string(code: &str) -> Option<crate::rustc_server::TokenStream> {
    Some(crate::rustc_server::TokenStream::from_str(code).unwrap())
}

pub fn assert_expand(macro_name: &str, ra_fixture: &str, expect: Expect) {
    assert_expand_impl(macro_name, ra_fixture, None, expect);
}

pub fn assert_expand_attr(macro_name: &str, ra_fixture: &str, attr_args: &str, expect: Expect) {
    assert_expand_impl(macro_name, ra_fixture, Some(attr_args), expect);
}

fn assert_expand_impl(macro_name: &str, input: &str, attr: Option<&str>, expect: Expect) {
    let path = fixtures::proc_macro_test_dylib_path();
    let expander = dylib::Expander::new(&path).unwrap();
    let fixture = parse_string(input).unwrap();
    let attr = attr.map(|attr| parse_string(attr).unwrap().into_subtree());

    let res = expander.expand(macro_name, &fixture.into_subtree(), attr.as_ref()).unwrap();
    expect.assert_eq(&format!("{:?}", res));
}

pub fn list() -> Vec<String> {
    let path = fixtures::proc_macro_test_dylib_path();
    let task = ListMacrosTask { lib: path };
    let mut srv = ProcMacroSrv::default();
    let res = srv.list_macros(&task).unwrap();
    res.macros.into_iter().map(|(name, kind)| format!("{} [{:?}]", name, kind)).collect()
}
