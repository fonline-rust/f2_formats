use f2_common_format::Perk;
use represent::{MakeWith, VisitWith};

use super::ToDo;

#[derive(Debug, MakeWith, VisitWith)]
pub struct Drug {
    /// Determines what character's characteristic is to be changed
    /// Values:
    /// (see stat.msg, starting with the line 100)
    /// 0 - Strength
    /// 1 - Perception
    /// 2 - Endurance
    /// ...
    /// -1 - no effect
    /// -2 - for stats[0] - Amount[1] (below) will contain a random number between Amount[0], and Amount[1] (inclusive)
    stats: ToDo<[i32; 3]>,
    // amount to be changed immediately after taking the drug
    instant_effect: Amount,
    /// The time delay for the first effect (in game minutes).
    duration_1: ToDo<u32>,
    delayed_effect_1: Amount,
    /// The time delay for the second effect. This should be more than duration_1
    duration_2: ToDo<u32>,
    delayed_effect_2: Amount,
    addiction: Addiction,
}

type Amount = ToDo<[i32; 3]>;

#[derive(Debug, MakeWith, VisitWith)]
struct Addiction {
    /// The probability of getting addicted, in percent.
    rate: ToDo<i32>,
    /// Number of the perk to be given when the player is addicted.
    /// Note: any perk can be used, but not all will work.
    effect: Perk,
    /// Delay before the addiction effect is applied.
    onset: ToDo<i32>,
}
