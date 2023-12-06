#[derive(Debug, Clone, PartialEq)]
struct Letter {
    typ: String,
    msg: String,
    from: String,
    to: String,
}

struct Envelop {
    typ: String,
    data: Letter,
}

impl Envelop {
    fn new(typ: String, data: Letter) -> Self {
        Envelop { typ, data }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Letter> for Envelop {
    fn into(self) -> Letter {
        self.data
    }
}

fn main() {
    let letter1 = Letter {
        msg: "Hello there!".into(),
        typ: "Very important letter".into(),
        from: "joe".into(),
        to: "john".into(),
    };
    let letter1_bak = letter1.clone();

    let envelop_type = String::from("Very import letter");
    let envelop = Envelop::new(envelop_type, letter1);
    println!("envelop type: {}, {:?}", envelop.typ, envelop.data);

    // Cannot use into() directly in println! as we cannot do into::<Letter>()
    let letter2: Letter = envelop.into();
    println!("letter: {:?}", letter2);

    assert_eq!(letter1_bak, letter2);
}
