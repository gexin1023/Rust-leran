// 斐波那契数列
fn main() {
    let n:usize =46;
    println!("斐波那契数列中第{}个数是：{}", n,fibbo(n));
}

fn fibbo(n: usize) -> i32{
    let mut arr:Vec<i32> = Vec::new();
    let mut i : usize= 0;
    while i < n {
        if i<=1{
            arr.push(1);
        }else{
            arr.push(arr[0]+arr[1]);
            arr.remove(0);
        }
        i = i+1;
    }
    arr[1]
}
