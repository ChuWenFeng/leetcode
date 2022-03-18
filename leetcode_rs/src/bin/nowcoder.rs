fn main(){
    let mut s = String::new();
    let cin = std::io::stdin();
    cin.read_line(&mut s);
    s = s.replace("\n", "").replace("\r", "");
    
    let mut n:usize = s.parse().unwrap();
    s.clear();
    let mut cout = Vec::new();
    while n >0{
        s.clear();
        cin.read_line(&mut s);
        s = s.replace("\n", "").replace("\r", "");
        if s.len()<3{
            println!("{}",s);
            continue;
        }
        let mut chars = s.chars().collect::<Vec<char>>();
        cout.push(chars[0]);
        cout.push(chars[1]);
        let mut c_p = 1;
        for i in 2..chars.len(){
            if chars[i] == cout[c_p]{
                if c_p >= 2 && cout[c_p-1] == cout[c_p-2]{
                    continue;
                }
                if c_p >=1 && cout[c_p]==cout[c_p-1]{
                    continue;
                }
                cout.push(chars[i]);
                    c_p+=1;
            }else{
                cout.push(chars[i]);
                c_p+=1;
            }
        }
        println!("{}",cout.iter().fold(String::new(), |mut acc,x|{acc.push(*x);acc}));
        s.clear();
        cout.clear();
        n-=1;
    }

}