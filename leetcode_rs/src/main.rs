static sum:i32 = 0;
fn main() {
    let mut rule = Vec::new();
    rule.push(vec![2,3,4]);
    rule.push(vec![2,3,4]);
    rule.push(vec![3]);
    rule.push(vec![0,1,2,4]);
    rule.push(vec![0,1,3]);

    let mut stack = Vec::new();
    stack.push(0);
    let ans = recursion_sol(&mut stack, &rule);
    
    println!("{}",ans);

}

fn recursion_sol(stack:&mut Vec<usize>,rule:&Vec<Vec<usize>>)->i32{
    let top = stack.last().unwrap().clone();
    if stack.len() == 6 {
        if top == 0{
            return 1;
        }else{
            return 0;
        }
    }
    let mut ans = 0;
    for &i in rule[top].iter(){
        stack.push(i);
        ans += recursion_sol(stack, rule);
        stack.pop();
    }

    return ans;
}

