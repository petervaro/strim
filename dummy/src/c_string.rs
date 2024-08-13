use strim::trim;

#[test]
fn valid() {
    assert_eq!(
        trim!(c"
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
ute irure dolor in reprehenderit in voluptate velit es\x0C
se cillum dolore eu fugiat nulla pariatur.

        "),
        c"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
          eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim \
          ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
          aliquip ex ea commodo consequat. Duis aute irure dolor in \
          reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
          pariatur.",
    );
}

#[test]
fn valid_and_trailing_comma() {
    assert_eq!(
        trim!(
            c"Hello,
              World!",
        ),
        c"Hello,World!"
    );
}

#[cfg(feature = "compile-errors")]
#[test]
fn invalid_trailing_commas() {
    trim!(c"hello",,);
}

#[cfg(feature = "compile-errors")]
#[test]
fn invalid_named_parameter() {
    trim!(c"hello", hello);
}

#[cfg(feature = "compile-errors")]
#[test]
fn incomplete_join_with() {
    trim!(c"hello", join_with);
    trim!(c"hello", join_with =);
}

#[cfg(feature = "compile-errors")]
#[test]
fn invalid_assignment_for_join_with() {
    trim!(c"hello", join_with: '\n');
}

#[cfg(feature = "compile-errors")]
#[test]
fn invalid_delimiter_for_join_with() {
    trim!(c"hello", join_with = ());
    trim!(c"hello", join_with = 1 + 1);
}

#[cfg(feature = "compile-errors")]
#[test]
fn invalid_nul_terminator_delimiter_for_join_with() {
    trim!(c"hello", join_with = b'\0');
}

#[test]
fn valid_joined_with_byte() {
    // Without trailing comma
    assert_eq!(
        trim!(
            c"What
              When
              Where
              Who
              Whom
              Which
              Whose
              Why
              How
              I don't know!",
            join_with = b'?'
        ),
        c"What?When?Where?Who?Whom?Which?Whose?Why?How?I don't know!",
    );

    // With trailing comma
    assert_eq!(
        trim!(c"
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
ute irure dolor in reprehenderit in voluptate velit es\x0C
se cillum dolore eu fugiat nulla pariatur.

            ",
            join_with = b'|',
        ),
        c"Lorem ipsum dolor s|it amet|, consectetur adipisc|ing elit, sed do \
          eiusmod tem|por incididunt ut labore et do|lore magna al|iqua. Ut \
          en|im ad minim ven|iam, quis nostrud exercitation ullamco la|boris \
          nisi ut aliquip ex ea commodo consequat|. Duis a|ute irure dolor in \
          reprehenderit in voluptate velit es|se cillum dolore eu fugiat nulla \
          pariatur."
    );
}

#[test]
fn valid_joined_with_byte_string() {
    // Without trailing comma
    assert_eq!(
        trim!(
            c"What
              When
              Where
              Who
              Whom
              Which
              Whose
              Why
              How
              I don't know!",
            join_with = c"? "
        ),
        c"What? When? Where? Who? Whom? Which? Whose? Why? How? I don't know!",
    );

    // With trailing comma
    assert_eq!(
        trim!(c"
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
ute irure dolor in reprehenderit in voluptate velit es\x0C
se cillum dolore eu fugiat nulla pariatur.

            ",
            join_with = c"<|>",
        ),
        c"Lorem ipsum dolor s<|>it amet<|>, consectetur adipisc<|>ing elit, \
          sed do eiusmod tem<|>por incididunt ut labore et do<|>lore magna \
          al<|>iqua. Ut en<|>im ad minim ven<|>iam, quis nostrud exercitation \
          ullamco la<|>boris nisi ut aliquip ex ea commodo consequat<|>. Duis \
          a<|>ute irure dolor in reprehenderit in voluptate velit es<|>se \
          cillum dolore eu fugiat nulla pariatur."
    );
}

#[cfg(feature = "compile-errors")]
#[test]
fn invalid_trailing_commas_after_join_with() {
    trim!(c"hello", join_with = '\n',,);
    trim!(c"hello", join_with = c"\r\n",,);
}
