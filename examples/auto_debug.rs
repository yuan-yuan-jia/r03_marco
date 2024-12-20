use _3macro::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    #[debug(skip=true)]
    inner: String,
    #[debug(skip)]
    nothing: (),
    hello: u32,
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
        hello: 2,
    };

    println!("{:?}", s);
}