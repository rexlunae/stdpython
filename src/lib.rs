#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(extend_one)]
#![feature(let_chains)]
// https://pyo3.rs/main/function/signature

//#![feature(c_variadic)]
//use std::collections::HashMap;
use std::fmt::Display;
use pyo3::{
    pyfunction, 
    types::{PyDict, PyTuple},
};

//pub mod pyfn;

/// Implements Python print() function.
#[pyfunction]
#[pyo3(signature = (*objects, **kwds))]
pub fn print(objects: &PyTuple, kwds: Option<&PyDict>) {
    let sep = if let Some(k) = kwds && let Some(v) = k.get_item("sep") { v.to_string() } else { " ".to_string() };
    let end = if let Some(k) = kwds && let Some(v) = k.get_item("end") { v.to_string() } else { "\n".to_string() };

    for o in objects {
        print!("{}{}", o, sep);

    }
    println!("{}", end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
