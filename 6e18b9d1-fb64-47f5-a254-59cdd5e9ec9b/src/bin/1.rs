#![allow(unused_imports)]
#![allow(non_snake_case)]
use ac_library::ModInt998244353 as Mint;
use ac_library::{Dsu, FenwickTree};
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use num::integer::*;
use num::{BigInt, BigRational, Integer};
use proconio::{fastout, input, marker::*};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use std::vec;
use std::{iter, usize};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut c:[usize;n],
    };
    const MOD: usize = 1_000_000_007;

    c.sort();
    let mut ans = 1;
    for (i, &x) in c.iter().enumerate() {
        ans *= x.saturating_sub(i);
        ans %= MOD;
    }
    println!("{}", ans);
}
