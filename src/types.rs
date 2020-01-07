type Layout = Vec<(Box<DataType>, i32)>;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum DataType {
    Union(Layout),
    Struct(Layout),
    Func,
    Pointer(Box<DataType>),
}

pub fn card(ty: DataType) -> i32 {
    use DataType::*;
    match ty {
        Union(s) => s.iter().fold(0, |res, sty| {
            let t = sty.0.clone();
            res + card(*t)
        }),
        Struct(s) => s.iter().fold(1, |res, sty| {
            let t = sty.0.clone();
            res * card(*t)
        }),
        Pointer(t) => card(*t),
        Func => panic!("Functions shouldnt be treated as cardinality")
    }
}

fn num_bits(n: usize) -> i32 {
    let mut k = 0;
    let mut p = 1;
    loop {
        if n <= p {
            return k;
        }
        k+=1;
        p*=2;
    }
}

pub fn size(ty: DataType) -> i32 {
    use DataType::*;
    match ty {
        Union(s) => num_bits(s.len())+s.iter().fold(0, |res, sty| {
            let t = sty.0.clone();
            let c = size(*t);
            if res > c { res } else { c }
        }),
        Struct(s) => s.iter().fold(0, |res, sty| {
            let t = sty.0.clone();
            res + size(*t)
        }),
        Pointer(_) => panic!("Pointer sizes unknown"),
        Func => panic!("Functions shouldnt be treated as cardinality")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::DataType::*;

    #[test]
    fn cardinality_void_is_0() {
        let void = Union(vec![]);
        assert_eq!(card(void), 0);
    }
    #[test]
    fn cardinality_unit_is_1() {
        let unit = Struct(vec![]);
        assert_eq!(card(unit), 1);
    }
    #[test]
    fn cardinality_bool_is_2() {
        let unit = Struct(vec![]);
        let boolt = Union(vec![(Box::new(unit.clone()), 0), (Box::new(unit), 1)]);

        assert_eq!(card(boolt), 2);
    }
    #[test]
    fn cardinality_trit_is_3() {
        let unit = Struct(vec![]);
        let trit = Union(vec![
            (Box::new(unit.clone()), 0),
            (Box::new(unit.clone()), 1),
            (Box::new(unit), 2)
        ]);

        assert_eq!(card(trit), 3);
    }
    #[test]
    fn cardinality_nibble_is_4() {
        let unit = Struct(vec![]);
        let boolt = Union(vec![(Box::new(unit.clone()), 0), (Box::new(unit), 1)]);
        let nibble = Struct(vec![(Box::new(boolt.clone()), 0), (Box::new(boolt), 1)]);

        assert_eq!(card(nibble), 4);
    }

    #[test]
    fn size_void_is_0() {
        let void = Union(vec![]);
        assert_eq!(size(void), 0);
    }
    #[test]
    fn size_unit_is_0() {
        let unit = Struct(vec![]);
        assert_eq!(size(unit), 0);
    }
    #[test]
    fn size_bool_is_1() {
        let unit = Struct(vec![]);
        let boolt = Union(vec![(Box::new(unit.clone()), 0), (Box::new(unit), 1)]);

        assert_eq!(size(boolt), 1);
    }
    #[test]
    fn size_trit_is_2() {
        let unit = Struct(vec![]);
        let trit = Union(vec![
            (Box::new(unit.clone()), 0),
            (Box::new(unit.clone()), 1),
            (Box::new(unit), 2)
        ]);

        assert_eq!(size(trit), 2);
    }
    #[test]
    fn size_nibble_is_2() {
        let unit = Struct(vec![]);
        let boolt = Union(vec![(Box::new(unit.clone()), 0), (Box::new(unit), 1)]);
        let nibble = Struct(vec![(Box::new(boolt.clone()), 0), (Box::new(boolt), 1)]);

        assert_eq!(size(nibble), 2);
    }
}
