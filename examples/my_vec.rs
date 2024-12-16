use _3macro::my_vec;
use anyhow::Result;
fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
    ];

    println!("{:?}", v);
    Ok(())
}