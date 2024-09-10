use crate::Element;
use crate::Element::*;

pub fn filter(e1: &Element, e2: &Element) -> Option<Element> {
    if let BOOL(f) = e2 {
        if *f {
            return Some(e1.to_owned());
        }
    }
    None
}
pub fn add(e1: &Element, e2: &Element) -> Element {
    match e1 {
        STR(s1) => {
            if let STR(s2) = e2 {
                return STR(format!("{}{}", s1, s2));
            }
        }
        UINT(u1) => {
            if let UINT(u2) = e2 {
                return UINT(u1 + u2);
            }
        }
        INT(i1) => {
            if let INT(i2) = e2 {
                return INT(i1 + i2);
            }
        }
        FLOAT(f1) => {
            if let FLOAT(f2) = e2 {
                return FLOAT(f1 + f2);
            }
        }
        BOOL(b1) => {
            if let BOOL(b2) = e2 {
                return BOOL(b1 ^ b2);
            }
        }
    }
    panic!("error");
}
pub fn sub(e1: &Element, e2: &Element) -> Element {
    match e1 {
        STR(_) => (),
        UINT(u1) => {
            if let UINT(u2) = e2 {
                return UINT(u1 - u2);
            }
        }
        INT(i1) => {
            if let INT(i2) = e2 {
                return INT(i1 - i2);
            }
        }
        FLOAT(f1) => {
            if let FLOAT(f2) = e2 {
                return FLOAT(f1 - f2);
            }
        }
        BOOL(b1) => {
            if let BOOL(b2) = e2 {
                return BOOL(b1 ^ b2);
            }
        }
    }
    panic!("error");
}
pub fn eq(e1: &Element, e2: &Element) -> bool {
    match e1 {
        STR(s1) => {
            if let STR(s2) = e2 {
                return s1 == s2;
            }
        }
        UINT(u1) => {
            if let UINT(u2) = e2 {
                return u1 == u2;
            }
        }
        INT(i1) => {
            if let INT(i2) = e2 {
                return i1 == i2;
            }
        }
        FLOAT(f1) => {
            if let FLOAT(f2) = e2 {
                return f1 == f2;
            }
        }
        BOOL(b1) => {
            if let BOOL(b2) = e2 {
                return b1 == b2;
            }
        }
    }
    panic!("error");
}
