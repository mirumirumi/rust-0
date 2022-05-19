
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    



    println!("{:#?}, {:#?}", four, six);



}
