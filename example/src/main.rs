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
}
