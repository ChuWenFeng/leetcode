--
-- @lc app=leetcode.cn id=196 lang=mysql
--
-- [196] 删除重复的电子邮箱
--

-- @lc code=start
# Please write a DELETE statement and DO NOT write a SELECT statement.
# Write your MySQL query statement below
delete p1 from Person p1 ,Person p2 
where p1.Email = p2.Email and p1.Id > p2.Id;
-- @lc code=end

