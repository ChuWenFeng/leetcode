use super::Solution;
/*
 * @lc app=leetcode.cn id=812 lang=rust
 *
 * [812] 最大三角形面积
 */

// @lc code=start
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        if points.len()<3{
            return 0.0;
        }
        let mut plist = vec![];
        for p in points{
            plist.push((p[0],p[1]));
        }
        plist.sort();
        let mut ans:f64 = 0.0;
        let convexHull = get_convexHull(plist);
        for i in 0..convexHull.len(){
            for j in i+1..convexHull.len(){
                for k in j+1..convexHull.len(){
                    ans = ans.max(area(convexHull[i],convexHull[j],convexHull[k]));
                }
            }
        }
        ans
    }
}

fn get_convexHull(plist:Vec<(i32,i32)>)->Vec<(i32,i32)>{
    let mut hull = vec![];

    for i in 0..plist.len(){
        while hull.len()>1 && cross(hull[hull.len()-2], hull[hull.len()-1], plist[i])<=0{
            hull.pop();
        }
        hull.push(plist[i]);
    }

    let len = plist.len();

    for i in (0..plist.len()-1).rev(){
        while hull.len()>len && cross(hull[hull.len()-2], hull[hull.len()-1], plist[i])<=0{
            hull.pop();
        }
        hull.push(plist[i]);
    }
    hull.pop();

    hull
}

fn cross(p:(i32,i32),q:(i32,i32),r:(i32,i32))->i32{
    return (q.0-p.0)*(r.1-q.1) - (q.1-p.1)*(r.0-q.0)
}

fn area(p1:(i32,i32),p2:(i32,i32),p3:(i32,i32))->f64{
    let x1 = p1.0 as f64;
    let y1 = p1.1 as f64;
    let x2 = p2.0 as f64;
    let y2 = p2.1 as f64;
    let x3 = p3.0 as f64;
    let y3 = p3.1 as f64;
    (x1*y2+x2*y3+x3*y1-x1*y3-x2*y1-x3*y2).abs()/2.0
}
// @lc code=end

