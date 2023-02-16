mod entry {

    use std::borrow::Cow;

    struct Entry {
        title: Cow<'static, str>,
        pos: Cow<'static, str>,
        definitions: Vec<Cow<'static, str>>,
    }



    fn tost(){
    let e = Entry{title: "yangle", pos: "N", vec![]};
    }
}
