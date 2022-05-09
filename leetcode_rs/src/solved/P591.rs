use super::Solution;
/*
 * @lc app=leetcode.cn id=591 lang=rust
 *
 * [591] 标签验证器
 */

// @lc code=start
impl Solution {
    pub fn is_valid(code: String) -> bool {
        let mut stack:Vec<String> = vec![];
        let charlist:Vec<char> = code.chars().collect();
        let len = code.len();
        let mut idx = 0;
        
        while idx < len{
            let c = charlist[idx];
            if stack.len() == 0 && c != '<'{
                return false;
            }
            match c{
                '<'=>{
                    if idx+1 >= len{
                        return false;
                    }
                    let cnext = charlist[idx+1];
                    match cnext{
                        '/'=>{
                            if let Some(tag) = code[idx+2..].find('>'){
                                if let Some(poptag) = stack.pop(){
                                    if poptag != code[idx+2..idx+2+tag].to_string(){
                                        return false;
                                    }
                                    idx = idx+2+tag+1;
                                    if stack.len() == 0 && idx<len{
                                        return false
                                    }
                                }else{
                                    return false;   
                                }
                            }else{
                                return false;
                            }
                            
                        },
                        '!'=>{
                            if stack.len() == 0 || idx+9>=len || &code[idx+2..idx+9] != "[CDATA["{
                                return false;
                            }
                            if let Some(cdidx) = code[idx+9..].find("]]>"){
                                idx = idx+9+cdidx+3;
                            }else{
                                return false;
                            }
                        },
                        cn if cn >= 'A' || cn <= 'Z' =>{
                            if let Some(endidx) = code[idx+1..].find('>'){
                                if endidx == 0 || endidx > 9{
                                    return false;
                                }
                                for upper in code[idx+1..idx+1+endidx].chars(){
                                   if !upper.is_uppercase(){
                                       return false;
                                   }
                                }
                                stack.push(code[idx+1..idx+1+endidx].to_string());
                                idx = idx+1+endidx+1;
                            }else{
                                return false;
                            }
                        },
                        _=>{
                            idx+=1;
                        }
                    }
                }
                _=>{
                    if let Some(nc) = code[idx+1..].find('<'){
                        idx = idx+1+nc;
                    }else{
                        break; 
                    }
                }
            }
        }


        return stack.len() == 0;
    }
}
// @lc code=end

#[test]
fn test(){
    let code = "<DIV><YFSYYS><UVBNIQ><XPMXUNT><WNGMV><OJJGQREMT><Z><GEJDP><LIQS><NCVYU><RAS><UYFKCJCDN><NA><POJVYT><Z><TDC><VUIZQC><BNANGX><TOF><MR>MK</MR></TOF></BNANGX></VUIZQC></TDC></Z></POJVYT></NA></UYFKCJCDN></RAS></NCVYU></LIQS></GEJDP></Z></OJJGQREMT></WNGMV></XPMXUNT></UVBNIQ></YFSYYS></DIV>".to_string();
    // println!("{:?}",code[1..].find('>'));
    let ans = true;
    let res  =Solution::is_valid(code);
    assert_eq!(res,ans);
}