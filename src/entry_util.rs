mod entry_util {

    #[macro_export]
    macro_rules! make_entry {
        ($title:literal,$pos:literal,$def:literal) => {
            [$title, $pos, $def]
        };

        ($title:literal, $def:literal) => {
            [$title, "", $def]
        };
    }
}
