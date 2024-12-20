use _3macro::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[deref(field = "inner")]
pub struct RespBulking {
    inner: String,
    nothing: (),
}

fn main() {
    let s = RespBulking {
        inner: "hello".to_string(),
        nothing: (),
    };

    println!("{:?}", &s);
}