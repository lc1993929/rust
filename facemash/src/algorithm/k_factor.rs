use crate::algorithm::elo_rating::EloScore;

pub fn fide_k(rating: EloScore, game_counts: u64) -> u64 {
    if game_counts < 30 {
        40
    } else if rating < 2400 {
        20
    } else {
        10
    }
}

pub fn uscf_k(rating: EloScore) -> u64 {
    if rating < 2100 {
        32
    } else if rating < 2400 {
        24
    } else {
        16
    }
}

pub fn icc_k() -> u64 {
    32
}
