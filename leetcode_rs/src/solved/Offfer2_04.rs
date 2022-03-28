use super::Solution;

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let n = matrix.len();
        if n == 0{
            return false;
        }
        let m = matrix[0].len();
        if m == 0{
            return false;
        }

        let mut row = 0;
        let mut col = m-1;

        while row < n && col >= 0 {
            match matrix[row][col].cmp(&target){
                 std::cmp::Ordering::Less =>{
                    row +=1;
                },
                 std::cmp::Ordering::Greater =>{
                    if col == 0{
                        return false;
                    }
                    col -=1;
                },
                _ => return true,
            }
            
        }
        

        return false;
    }
}

#[test]
fn test(){
    let input1 = vec![vec![5],vec![6]];
    let input2 = 6;
    let ans = true;

    let res = Solution::find_number_in2_d_array(input1, input2);
    assert_eq!(res,ans);
}