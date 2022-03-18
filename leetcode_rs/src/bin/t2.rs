fn main(){
    let mut s = String::new();
    let cin = std::io::stdin();
    cin.read_line(&mut s);
    s = s.replace("\n", "").replace("\r", "");
    let n_d = s.split(" ").map(|x|x.parse().unwrap()).collect::<Vec<usize>>();
    let N = n_d[0];
    let D = n_d[1];
    s.clear();
    cin.read_line(&mut s);
    s = s.replace("\n", "").replace("\r", "");
    let mut build:Vec<usize>= s.split(" ").map(|x|x.parse().unwrap()).collect();
    build.sort();
    let mut l = 0;
    let mut r = 2;
    let mut ans = 0;
    while r<build.len(){
        if r-l<2{
            r+=1;
            continue;
        }
        if build[r]-build[l] > D{
            l+=1;
            continue;
        }
        ans += (r-l)*(r-l-1)/2;
        ans %= 99997867;
        r+=1;
    }
    println!("{}",ans);

}