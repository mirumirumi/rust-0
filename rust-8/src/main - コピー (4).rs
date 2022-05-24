fn main() {
    let _v: Vec<i32> = Vec::new();
    let _vv: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];
    
    let mut v = Vec::new();  // このあと値を入れるから型推論できる、型推論はある一時点だけじゃなくて未来にまで関与するのか…（たしかにLinterも何も言わない）
    v.push(3);

    {
        let v = vec![1 ,3 ,4];
    }

    let v = vec![1 ,3 ,4];
    for x in &v {
        println!("{}", x);
    }

    let s = "aiueo";

    println!("{}", s.to_string());

    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);




}
