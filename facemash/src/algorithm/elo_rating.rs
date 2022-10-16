use crate::algorithm::k_factor::{fide_k, icc_k, uscf_k};

pub type EloScore = i64;

pub type EloCompeteResult = f64;

pub const WIN: EloCompeteResult = 1_f64;
pub const DRAW: EloCompeteResult = 0.5;
pub const LOSS: EloCompeteResult = 0_f64;

/// 计算分数变化
///
/// Arguments:
///
/// * `k`: 一个帮助计算的常数
/// * `score`: 胜负值，胜者为1，败者为0
/// * `exp_score`: 期望胜率
///
/// Returns:
///
/// rank分的变化
fn rating_change(k: u64, score: EloCompeteResult, exp_score: EloCompeteResult) -> EloScore {
    (k as f64 * (score - exp_score)) as i64
}

/// `expected_score` 接受两个玩家的rank分并返回第一个玩家的期望胜率
///
/// Arguments:
///
/// * `r_a`: 玩家A的rank分
/// * `r_b`: 玩家B的rank分
///
/// Returns:
///
/// 玩家 A 的期望胜率。
pub fn expected_score(r_a: EloScore, r_b: EloScore) -> f64 {
    1_f64 / (1_f64 + 10_f64.powf(((r_b - r_a) as f64) / 400_f64))
}

/// fide模式计算两个玩家最新的rank分
///
/// Arguments:
///
/// * `r_a`: 玩家 A 的rank分数
/// * `game_count_a`: 玩家 A 玩过的游戏数。
/// * `r_b`: 玩家B的rank分
/// * `game_count_b`: 玩家 B 玩过的游戏数。
/// * `s_a`: 游戏结果，如果a赢则传1，否则传0。
///
/// Returns:
///
/// 两名玩家的最新的rank分
pub fn compete_fide(
    r_a: EloScore,
    game_count_a: u64,
    r_b: EloScore,
    game_count_b: u64,
    s_a: EloCompeteResult,
) -> (EloScore, EloScore) {
    let k_a = fide_k(r_a, game_count_a);
    let k_b = fide_k(r_b, game_count_b);

    compete(r_a, r_b, s_a, k_a, k_b)
}

pub fn compete_uscf(r_a: EloScore, r_b: EloScore, s_a: EloCompeteResult) -> (EloScore, EloScore) {
    let k_a = uscf_k(r_a);
    let k_b = uscf_k(r_b);

    compete(r_a, r_b, s_a, k_a, k_b)
}

pub fn compete_icc(r_a: EloScore, r_b: EloScore, s_a: EloCompeteResult) -> (EloScore, EloScore) {
    let k_a = icc_k();
    let k_b = icc_k();

    compete(r_a, r_b, s_a, k_a, k_b)
}

/// `compete` 接受两个 rank 分数、比赛结果和两个 k 因子，并返回两个竞争对手的新 Elo 分数
///
/// Arguments:
///
/// * `r_a`: 玩家A的当前rank分
/// * `r_b`: 玩家B的当前rank分
/// * `s_a`: 玩家 A 的得分。这是一个介于 0 和 1 之间的数字，其中 0 表示玩家 A 输了，1 表示玩家 A 赢了。
/// * `k_a`: 玩家 A 的 K 因子。
/// * `k_b`: 玩家 B 的 K 因子。
pub fn compete(
    r_a: EloScore,
    r_b: EloScore,
    s_a: EloCompeteResult,
    k_a: u64,
    k_b: u64,
) -> (EloScore, EloScore) {
    let s_b = 1_f64 - s_a;

    let e_a = expected_score(r_a, r_b);
    let e_b = 1_f64 - e_a;

    let new_a = r_a + rating_change(k_a, s_a, e_a);
    let new_b = r_b + rating_change(k_b, s_b, e_b);

    (new_a, new_b)
}

/// `serial_compete` 计算多次游戏后的最新rank分
///
/// Arguments:
///
/// * `r_a`: 玩家的初始rank分
/// * `games`: 元组列表，其中第一个元素是对手的评分，第二个元素是游戏的结果。
/// * `k_factor`: K 因子是一个常数，它决定了比赛结束后评分的变化幅度。
///
/// Returns:
///
/// 玩家最新的rank分
pub fn serial_compete(
    r_a: EloScore,
    games: &[(EloScore, EloCompeteResult)],
    k_factor: u64,
) -> EloScore {
    let mut score = 0_f64;
    let mut exp_score = 0_f64;

    for game in games {
        score += game.1;
        exp_score = expected_score(r_a, game.0);
    }

    r_a + rating_change(k_factor, score, exp_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_score() {
        let john = 1700;
        let paul = 1800;

        // calculate johns chance to win against paul
        let chance = expected_score(john, paul);
        assert!((0.0..=1.0).contains(&chance));
        println!("johns chance to win against paul: {}", chance)
    }

    #[test]
    fn test_compete() {
        let john = 1700;
        let paul = 1800;

        println!("before compete: john: {}, paul: {}", john, paul);
        let (john, paul) = compete(john, paul, LOSS, 32, 32);
        println!("after compete(paul win): john: {}, paul: {}", john, paul);
    }

    #[test]
    fn test_serial_compete() {
        let john = 1700;
        println!("before serial competes: john: {}", john);

        // An array containing the results of johns games in the tournament
        let games = [(1600, WIN), (1800, DRAW), (2000, LOSS)];

        let john = serial_compete(john, &games, 32);
        println!("after serial competes: john: {}", john);
    }

    #[test]
    fn test_compete_uscf() {
        let john = 1400;
        let paul = 1800;

        println!("before compete uscf: john: {}, paul: {}", john, paul);
        let (john, paul) = compete_uscf(john, paul, WIN);
        println!(
            "after compete uscf(paul win): john: {}, paul: {}",
            john, paul
        );
    }
}
