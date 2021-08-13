/*
 * Copyright (c) 2021. Davi Pereira dos Santos
 * This file is part of the hosh project.
 * Please respect the license - more about this in the section (*) below.
 *
 * hosh is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * hosh is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with hosh.  If not, see <http://www.gnu.org/licenses/>.
 *
 * (*) Removing authorship by any means, e.g. by distribution of derived
 * works or verbatim, obfuscated, compiled or rewritten versions of any
 * part of this work is a crime and is unethical regarding the effort and
 * time spent here.
 * Relevant employers or funding agencies will be notified accordingly.
 */

// #![feature(iterator_fold_self)]
use std::collections::VecDeque;
use std::str;
use std::time::Instant;
use std::convert::TryInto;

pub mod math;

use math::{digest, from_b62, int_to_perm, to_b62};
use crate::math::{PERM, DIGITS, NBYTES};

fn main() {
    for _i in 0..6 {
        let mut lst: VecDeque<(PERM, DIGITS)> = VecDeque::new();
        let now = Instant::now();
        for i in 1..10_000 {
            let byts: [u8; NBYTES / 2] = u128::to_be_bytes(83612 + i);
            let content: [u8; NBYTES] = [byts, byts].concat().try_into().unwrap();
            let digest = digest(&content);
            let n = u128::from_be_bytes(digest[..NBYTES / 2].try_into().unwrap());
            let perm = int_to_perm(&n);  // 350ns
            // let perm2 = &mut perm.clone();
            // let perm2 = mul(&perm, perm2);
            lst.push_front((perm, to_b62(&n)));
        }

        let t = now.elapsed().as_nanos() as f64 / 10_000.0;
        let res = to_b62(&295232799039604140847618609643519999999);
        let resd = from_b62(&res);
        let x: u128 = 340282366920938463463374607431768211455;
        let y: u128 = 2;
        println!(
            "{} {}us  <{}>  {} {}", math::add(&x, &math::ainv(&math::ainv(&y))),
            t.round() / 1000.0,
            str::from_utf8(&res).unwrap(),
            lst.len(),
            resd
        );
    }
}


// /// Take an array representing a sequence of 3-tuples and fold it through an arbitrary sandwich logic.
// fn keep_largest(lst: &[u8]) -> [u8; 3] {
//     lst.chunks(3).map(|x| [x[0], x[1], x[2]]).reduce(|x, y| [x[0], y[1], x[0]]).unwrap()
// }
// /*
// 3  |     lst.chunks(3).reduce(|x, y| [x[0], y[1], x[0]]).unwrap()
//    |                                 ^^^^^^^^^^^^^^^^^^
//    |                                 |
//    |                                 expected `&[u8]`, found array `[u8; 3]`
//    |                                 help: consider borrowing here: `&[x[0], y[1], x[0]]`
// */
//
// fn keep_largest2(lst: &[u8]) -> [u8; 3] {
//     let mut r: [u8; 3] = lst[..].try_into().unwrap();
//     for i in (3..lst.len()).step_by(3) {
//         let y = &lst[i..i + 3];
//         r = [r[0], y[1], r[0]];
//     }
//     r
// }


use std::*;
use std::collections::HashMap;
//
// fn m4m<T0, T1, T2, RT>(a: T0, b: T1, mo: T2) -> RT {
//     "Multiply two unitriangular matrices 4x4 modulo 'mo'.
//
//     'a' and 'b' given as lists in the format: [a1,4 a1,3 a2,4 a2,3 a3,4 a1,2]
//
//     1 a0 a4 a5
//     0  1 a2 a3
//     0  0  1 a1
//     0  0  0  1
//
//     >>> a, b = [51,18340,56,756,456,344], [781,2340,9870,1234,9134,3134]
//     >>> m4m(b, m4inv(b, 4294967291), 4294967291) == [0,0,0,0,0,0]
//     True
//     >>> c = m4m(a, b, 4294967291)
//     >>> m4m(c, m4inv(b, 4294967291), 4294967291) == a
//     True
//     ";
//     return vec![((((a[0] + b[0]) + (a[5] * b[2])) + (a[1] * b[4])) % mo), (((a[1] + b[1]) + (a[5] * b[3])) % mo), (((a[2] + b[2]) + (a[3] * b[4])) % mo), ((a[3] + b[3]) % mo), ((a[4] + b[4]) % mo), ((a[5] + b[5]) % mo)];
// }
//
// fn m4inv<T0, T1, RT>(m: T0, mo: T1) -> RT {
//     "Inverse of unitriangular matrix modulo 'mo'
//
//     'm' given as a list in the format: [a1,4 a1,3 a2,4 a2,3 a3,4 a1,2]
//
//     1 a0 a4 a5
//     0  1 a2 a3
//     0  0  1 a1
//     0  0  0  1
//
//     Based on https://groupprops.subwiki.org/wiki/Unitriangular_matrix_group:UT(4,p)
//
//     >>> e = [42821,772431,428543,443530,42121,7213]
//     >>> m4inv(m4inv(e, 4294967291), 4294967291)==e
//     True
//     ";
//     return vec![(((((m[5] * m[2]) + (m[1] * m[4])) - ((m[5] * m[3]) * m[4])) - m[0]) % mo), (((m[5] * m[3]) - m[1]) % mo), (((m[3] * m[4]) - m[2]) % mo), (-(m[3]) % mo), (-(m[4]) % mo), (-(m[5]) % mo)];
// }
//
// fn int2m4<T0, T1, RT>(num: T0, mo: T1) -> RT {
//     "
//     >>> e = [42821,772431,428543,443530,42121,7213]
//     >>> e == int2m4(m42int(e,4294967291), 4294967291)
//     True
//     ";
//     let m = vec![0, 0, 0, 0, 0, 0];
//     let (num, m[5]) = divmod(num, mo);
//     let (num, m[4]) = divmod(num, mo);
//     let (num, m[3]) = divmod(num, mo);
//     let (num, m[2]) = divmod(num, mo);
//     let (num, m[1]) = divmod(num, mo);
//     let (num, m[0]) = divmod(num, mo);
//     return m;
// }
//
// fn m42int<T0, T1, RT>(m: T0, mo: T1) -> RT {
//     "
//     >>> n = 986723489762345987253897254295863
//     >>> m42int(int2m4(n, 4294967291), 4294967291) == n
//     True
//     ";
//     return (((((m[5] + (m[4] * mo)) + (m[3] * mo.pow(2))) + (m[2] * mo.pow(3))) + (m[1] * mo.pow(4))) + (m[0] * mo.pow(5)));
// }
