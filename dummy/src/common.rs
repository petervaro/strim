// HACK: This is temporary, until the test cases enabled by the `compile-errors`
//       feature are not automatically checked!
#[allow(unused_imports)]
use strim::trim;

#[cfg(feature = "compile-errors")]
#[test]
fn no_arguments() {
    trim!();
}

#[cfg(feature = "compile-errors")]
#[test]
fn too_many_arguments() {
    trim!("hello" "world");
}
