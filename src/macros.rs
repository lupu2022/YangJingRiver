//! Macros

#[macro_export]
macro_rules! vector {
    () => {
        {
            // Handle the case when called with no arguments, i.e. vector![]
            use $crate::vector::Vector;
            Vector::new(vec![])
        }
    };
    ($($x:expr),*) => {
        {
            use $crate::vector::Vector;
            Vector::new(vec![$($x),*])
        }
    };
    ($x:expr; $n:expr) => {
        {
            use $crate::vector::Vector;
            Vector::new(vec![$x; $n])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;

    #[test]
    fn vector_macro() {
        {
            // An arbitrary vector
            let vec = vector![1, 2, 3, 4, 5, 6];
            assert_eq!(6, vec.size());
            assert_eq!(&vec![1, 2, 3, 4, 5, 6], vec.data());
        }

        {
            // A floating point vector
            let vec = vector![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
            let ref expected_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
            assert_eq!(6, vec.size());
            assert_eq!(expected_data, vec.data());
        }
    }

    #[test]
    fn vector_macro_constant_size() {
        // A constant size vector
        let vec = vector![1.0; 5];
        let ref expected_data = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        assert_eq!(5, vec.size());
        assert_eq!(expected_data, vec.data());
    }

    #[test]
    fn vector_macro_empty_vec() {
        let vec: Vector<f64> = vector![];

        assert_eq!(0, vec.size());
    }

}
