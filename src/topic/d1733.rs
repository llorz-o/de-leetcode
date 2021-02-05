/*
在一个由m个用户组成的社交网络里，我们获取到一些用户之间的好友关系。两个用户之间可以相互沟通的条件是他们都掌握同一门语言。

给你一个整数n，数组languages和数组friendships，它们的含义如下：

总共有n种语言，编号从1 到n。
languages[i]是第 i位用户掌握的语言集合。
friendships[i] = [ui, vi]表示ui 和vi为好友关系。
你可以选择 一门语言并教会一些用户，使得所有好友之间都可以相互沟通。请返回你 最少需要教会多少名用户。

请注意，好友关系没有传递性，也就是说如果x 和y是好友，且y和z是好友，x 和z不一定是好友。


示例 1：

输入：n = 2, languages = [[1],[2],[1,2]], friendships = [[1,2],[1,3],[2,3]]
输出：1
解释：你可以选择教用户 1 第二门语言，也可以选择教用户 2 第一门语言。
示例 2：

输入：n = 3, languages = [[2],[1,3],[1,2],[3]], friendships = [[1,4],[1,2],[3,4],[2,3]]
输出：2
解释：教用户 1 和用户 3 第三门语言，需要教 2 名用户。


来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/minimum-number-of-people-to-teach
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
*/
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn minimum_teachings(_: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        fn has_common(languages: &Vec<Vec<i32>>, i: i32, j: i32) -> bool {
            let ui: &Vec<i32> = &languages[(i - 1) as usize];
            let vi: &Vec<i32> = &languages[(j - 1) as usize];

            for u in ui {
                for v in vi {
                    if u == v {
                        return true;
                    }
                }
            }
            false
        }

        // 无法交流的好友
        let mut not_connected: HashMap<i32, i32> = HashMap::new();

        for friendship in friendships {
            if !has_common(&languages, friendship[0], friendship[1]) {
                for us in friendship {
                    match not_connected.get(&us) {
                        Some(&v) => not_connected.insert(us, v + 1),
                        None => not_connected.insert(us, 1)
                    };
                }
            }
        }

        //
        let mut most_language: HashMap<i32, i32> = HashMap::new();

        for (k, _) in not_connected.iter() {
            for l in languages[(k - 1) as usize].clone() {
                match most_language.get(&l) {
                    Some(&v) => most_language.insert(l, v + 1),
                    None => most_language.insert(l, 1)
                };
            }
        }

        print!("{:?}-{:?}", not_connected, most_language);

        let mut most = 0; // 找到会的最多的语言

        for (_, v) in most_language {
            if v > most {
                most = v;
            }
        }

        let nodes = not_connected.len(); // 所有无法沟通的人总数

        (nodes as i32) - most
    }
}

#[test]
fn t1() {
    let languages = vec![vec![1], vec![2], vec![1, 2]];
    let friendships = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    assert_eq!(Solution::minimum_teachings(2, languages, friendships), 1);
}

#[test]
fn t2() {
    let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
    let friendships = vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]];
    assert_eq!(Solution::minimum_teachings(3, languages, friendships), 2);
}