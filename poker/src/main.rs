#![allow(unused)]
/// 判断牌型
/// 为简单起见代码中相关变量名使用了中文缩写

#[derive(Debug, PartialEq)]
enum Poker {
    Ths, // 同花顺
    Bz,  // 豹子
    Dz,  // 单张
    Th,  // 同花
    Sz,  // 顺子
}

fn check_sz(mut pokers: Vec<(i32, i32)>) -> bool {
    pokers.sort_unstable();
    let mut f = pokers[0].0;
    pokers.iter().skip(1).all(|x| {
        f += 1;
        f == x.0
    })
}

fn check_bz(pokers: &[(i32, i32)]) -> bool {
    let f = pokers[0];
    pokers.iter().skip(1).all(|x| x.0 == f.0)
}

fn check_th(pokers: &[(i32, i32)]) -> bool {
    let f = pokers[0];
    pokers.iter().skip(1).all(|x| x.1 == f.1)
}

fn bz_compare(mut a: Vec<(i32, i32)>, mut b: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    a.sort_unstable();
    b.sort_unstable();
    if a[0].0 > b[0].0 {
        a
    } else {
        b
    }
}

fn check(pokers: Vec<(i32, i32)>) -> Poker {
    // 豹子
    if check_bz(&pokers) {
        return Poker::Bz;
    }
    // 同花
    if check_th(&pokers) {
        // 顺子
        if check_sz(pokers) {
            return Poker::Ths;
        }
        return Poker::Th;
    }
    // 顺子
    if check_sz(pokers) {
        return Poker::Sz;
    }
    Poker::Dz
}

fn main() {
    check(vec![(14, 1), (14, 2), (14, 3)]);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_check() {
        assert_eq!(check(vec![(14, 1), (14, 2), (14, 3)]), Poker::Bz);
        assert_eq!(check(vec![(12, 1), (14, 2), (13, 3)]), Poker::Sz);
        assert_eq!(check(vec![(10, 1), (14, 2), (12, 3)]), Poker::Dz);

        assert_eq!(
            bz_compare(
                vec![(14, 1), (14, 2), (14, 3)],
                vec![(13, 1), (13, 2), (13, 3)]
            ),
            vec![(14, 1), (14, 2), (14, 3)]
        );
    }
}
