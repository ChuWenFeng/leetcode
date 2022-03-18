struct Solution{
}

impl Solution {
    fn new() -> Self {
        Solution{}
    }

    pub fn solve(&self, nums: Vec<i32>) -> String {
        let mut s:Vec<String> = nums.iter().map(|x|format!("{}",x)).collect();
        s.sort_by(|x,y|{
            let s1 = (x.clone()+&y[..]);
            let s2 = (y.clone()+&x[..]);
            s2.cmp(&s1)
        });
        let mut ans = String::new();
        for i in s{
            ans += &i;
        }
        let mut iter =  ans.chars();
        let mut head = None;
        
        while let Some(c) = iter.next(){
            if c != '0'{
                head = Some(c);
                break;
            }
        }
        
        let mut ans = String::new();
        if head.is_none(){
            return String::from("0");
        }
        ans.push(head.unwrap());
        for i in iter{
            ans.push(i);
        }
        
        ans
    }
}