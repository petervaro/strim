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

#[test]
fn valid_cases() {
    assert_eq!(
        trim!("
                                Lorem ipsum dolor s
                            it amet
                        , consectetur adipisc
                    ing elit, sed do eiusmod tem
                por incididunt ut labore et do
            lore magna al
        iqua. Ut en
    im ad minim ven
iam, quis nostrud exercitation ullamco la

boris nisi ut aliquip ex ea commodo consequat\r\n. Duis a\t\t\t
ute irure dolor in reprehenderit in voluptate velit es\u{2000}\u{2009}
se cillum dolore eu fugiat nulla pariatur.

        "),
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
         eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim \
         ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
         aliquip ex ea commodo consequat. Duis aute irure dolor in \
         reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
         pariatur."
    );
}
