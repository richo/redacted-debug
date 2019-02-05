use redacted_debug::RedactedDebug;

#[derive(RedactedDebug)]
struct Demo<'a, T: ?Sized + std::fmt::Debug> {
    a: Box<T>,
    #[redacted]
    b: u8,
    c: &'a str,
    d: String,
}

#[derive(RedactedDebug)]
struct DemoTuple<'a, T: ?Sized + std::fmt::Debug>(
    Box<T>,
    u8,
    &'a str,
    #[redacted]
    String,
);

#[derive(RedactedDebug)]
struct Unit;
#[derive(Debug)]
struct Tuple(u8, String);
#[derive(Debug)]
struct Named {
    a: u8,
    b: String,
}

#[derive(RedactedDebug)]
enum EnumUntagged {
    A,
    B,
}

#[derive(Debug)]
enum EnumTagged {
    A(u8),
    B(String),
}

fn main() {
    let demo = Demo {
        a: b"bytestring".to_vec().into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: "String".to_owned(),
    };

    let demo_tuple = DemoTuple(
        b"bytestring".to_vec().into_boxed_slice(),
        255,
        "&'static str",
        "String".to_owned(),
    );

    println!("{:?}", &demo);
    println!("{:?}", &demo_tuple);

    let unit: Unit = Unit;
    println!("Unit: {:?}", &unit);

    let tuple = Tuple(1, "hi there".into());
    println!("Tuple: {:?}", &tuple);

    let named = Named { a: 12, b: "hi there".into() };
    println!("Named: {:?}", &named);

    let untagged = EnumUntagged::A;
    println!("EnumUntagged: {:?}", &untagged);

    let tagged = EnumTagged::A(12);
    println!("EnumTagged: {:?}", &tagged);
}
