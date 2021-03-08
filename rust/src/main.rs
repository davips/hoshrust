/*
 * Copyright (c) 2021. Davi Pereira dos Santos
 * This file is part of the halg project.
 * Please respect the license - more about this in the section (*) below.
 *
 * halg is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * halg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with halg.  If not, see <http://www.gnu.org/licenses/>.
 *
 * (*) Removing authorship by any means, e.g. by distribution of derived
 * works or verbatim, obfuscated, compiled or rewritten versions of any
 * part of this work is a crime and is unethical regarding the effort and
 * time spent here.
 * Relevant employers or funding agencies will be notified accordingly.
 */

use std::collections::VecDeque;
use std::str;
use std::time::Instant;
use reduce::Reduce;

use ::halg::{
    b62_to_str, digest, digest_to_int, from_b62, int_to_digest, int_to_perm, inv, mul, perm_to_int, to_b62, DIGITS,
};
use std::convert::TryInto;

fn main() {
    let mut lst: VecDeque<(u128, DIGITS)> = VecDeque::new();
    let now = Instant::now();
    for i in 1..100_000 {
        let content = int_to_digest(876123876213876123873612 + i);
        let digest = digest(&content);
        // println!("{:08b}", digest[0]);
        let n = digest_to_int(&digest);
        let digits = to_b62(&n);
        // println!("{}", digits);
        let n = from_b62(digits);
        let perm = int_to_perm(&n);
        let m = inv(&mul(&perm, &perm));
        let unperm = perm_to_int(&m);
        // println!("{} {} {:?}", n, unperm, perm);
        let res = to_b62(&n);
        lst.push_front((unperm, res));
    }

    let t = now.elapsed().as_nanos() as f64 / 100_000.0;
    let res = to_b62(&295232799039604140847618609643519999999);
    let resd = from_b62(res);
    println!(
        " {}ns  <{}>  {} {}",
        t.round(),
        str::from_utf8(&res).unwrap(),
        lst.len(),
        resd
    );

    let perm = int_to_perm(&295232799039604140847618609643519999);
    let m = mul(&perm, &perm);
    println!(" {:?} {:?}", m, b62_to_str(&m));
    let mi = inv(&perm);
    println!(" {:?}", perm);
    println!(" {:?}", (&mul(&m, &mi)));
}


// /// Take an array representing a sequence of 3-tuples and fold it through an arbitrary sandwich logic.
// fn keep_largest(lst: &[u8]) -> [u8; 3] {
//     lst.chunks(3).reduce(|x, y| &[x[0], y[1], x[0]]).unwrap()
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
