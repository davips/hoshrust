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

// pub mod ldictpy;

// pub mod math;
use ::halg;
use halg::{PERM, PERM_SIZE};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList};
use pyo3::wrap_pyfunction;
use reduce::Reduce;
use std::convert::TryInto;
use std::ops::Deref;
use std::error::Error;

/// A Python module implemented in Rust.
#[pymodule]
fn halg(py: Python, m: &PyModule) -> PyResult<()> {
    let _ = py;
    m.add_function(wrap_pyfunction!(b62, m)?)?;
    m.add_function(wrap_pyfunction!(mul, m)?)?;
    m.add_function(wrap_pyfunction!(mulmany_, m)?)?;
    Ok(())
}

/// doc
#[pyfunction]
fn b62(blob: &[u8]) -> PyResult<String> {
    let ret = halg::b62_to_str(&halg::to_b62(&halg::digest_to_int(&halg::digest(blob))));
    Ok((ret).to_string())
}

/// doc
#[pyfunction]
fn mul(a: &[u8], b: &[u8]) -> PyResult<String> {
    Ok(unsafe { String::from_utf8_unchecked(halg::mul(a, b).to_vec()) })
} // .as_bytes()[..NBYTES].try_into().unwrap()

/// doc
#[pyfunction]
fn mulmany_(perms: &[u8]) -> PyResult<String> {
    // let res = perms[..].iter();
    let mut r: PERM = [0; PERM_SIZE];
    // let mut r: PERM = match res {
    //     Ok(x) => x,
    //     Err(e) => panic!("{:?} >>>>>>> {:?}", perms, e),
    // };
    // ret = r.iter().reduce(halg::mul);
    for i in (PERM_SIZE..perms.len()).step_by(PERM_SIZE) {
        let a = &perms[i..i + PERM_SIZE];
        r = halg::mul(&r, &a);
    }
    unsafe { Ok(String::from_utf8_unchecked(r.to_vec())) }
}

// /// doc
// #[pyfunction]
// fn mulmany_(perms: Vec<u8>) -> PyResult<String> {
//     let cl = perms; //.clone();
//     let mut r:PERM = cl[..PERM_SIZE].try_into().unwrap();
//     for i in (PERM_SIZE..cl.len()).step_by(PERM_SIZE) {
//         let a:PERM = cl[i..i + PERM_SIZE].try_into().unwrap();
//         r = halg::mul(&r, &a);
//     }
//     unsafe { Ok(String::from_utf8_unchecked(r.to_vec())) }
// }
