// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');// 按逗号分割字符串
        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let team_1_name = split_iterator.next().unwrap(); // 获取第一个队伍名称
        let team_2_name = split_iterator.next().unwrap(); // next() 获取下一个值 unwrap() 解包
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap(); // parse() 将字符串转换为数字类型
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap(); // 最后unwrap解包得到数字值

        // 为什么需要解包两次？
        // 因为 split_iterator.next() 返回的是 Option<&str> 类型，
        // 需要先用 unwrap() 解包得到 &str 类型的字符串，
        // 然后再用 parse() 方法将 &str 转换为 u8 类型，
        // parse() 方法返回的是 Result<u8, ParseIntError> 类型，
        // 所以还需要再用 unwrap() 解包得到最终的 u8 数值


        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.



        // or_default 方法用于获取与指定键相关联的条目（Entry）。
        // 如果该键不存在，则会插入一个默认值（通过调用 Default trait 的 default 方法生成）并返回对新插入值的可变引用。
        let team1_entry = scores.entry(team_1_name).or_default();
        // or_default 会返回对 TeamScores 结构体的可变引用 &mut TeamScores
        team1_entry.goals_scored += team_1_score;
        team1_entry.goals_conceded += team_2_score;

        // 处理第二个队伍的数据 业务逻辑解释：
        // 加上 team_2_score 作为进球数
        // 加上 team_1_score 作为失球数
        let team2_entry = scores.entry(team_2_name).or_default();
        team2_entry.goals_scored += team_2_score;
        team2_entry.goals_conceded += team_1_score;
        
    }

    scores
}

fn main() {
    // You can optionally experiment here.
    // 举例测试
    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";
    let scores = build_scores_table(RESULTS);
    for (team, score) in scores.iter() {
        println!(
            "{} scored {} goals and conceded {} goals",
            team, score.goals_scored, score.goals_conceded
        );
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
