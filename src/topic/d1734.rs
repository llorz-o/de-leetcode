/*
给你一个整数数组perm，它是前n个正整数的排列，且n是个 奇数。

它被加密成另一个长度为 n - 1的整数数组encoded，满足encoded[i] = perm[i] XOR perm[i + 1]。比方说，
如果perm = [1,3,2]，那么encoded = [2,1]。

给你encoded数组，请你返回原始数组perm。题目保证答案存在且唯一。

示例 1：

输入：encoded = [3,1]
输出：[1,2,3]
解释：如果 perm = [1,2,3] ，那么 encoded = [1 XOR 2,2 XOR 3] = [3,1]
示例 2：

输入：encoded = [6,5,4,6]
输出：[2,4,1,5,3]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/decode-xored-permutation
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

[0001,0010,0011] -> [1,2,3]
[0011,0001] -> [3,1]
[0010]  -> [2]

给出[3,1] 反推 [1,2,3]

[a, b, c, d, e]
 [ab,bc,cd,de]
  [ac,bd,ce]
  [acbd,bdce]

bdce = bcde

abcde ^ bcde(bdce) = a

*/

struct Solution {}

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let mut r = vec![];
        let len = encoded.len();
        let mut all = 0;
        for i in 1..len + 2 {
            all ^= i as i32;
        }
        let mut all_but_frist = 0;
        for i in (1..len).step_by(2) {
            all_but_frist ^= &encoded[i];
        }
        r.push(all ^ all_but_frist);
        for i in 1..len+1 {
            r.push(r[i - 1] ^ encoded[i - 1]);
        }
        r
    }
}

#[test]
fn t1() {
    let r = Solution::decode(vec![6,5,4,6]);
    print!("{:?}", r);
}