
#[cfg(test)]
mod code_with_declarative {
    use declarative::foreach;

    #[test]
    pub fn should_perform_lists_foreach_with_filter() {
        let r: Vec<_> = procedural::foreach! {
            a <- (1..=3)
            b <- (1..=a) if b != 2
            yield (a, b)
        }
        .collect();

        assert_eq!(r, vec![(1, 1), (2, 1), (3, 1), (3, 3)]);
    }

    #[test]
    #[allow(unused_variables)]
    pub fn should_perform_option_foreach() {
        let r: Option<i32> = foreach! {
            a <- (Some(1))
            b <- (None::<i32>) if (a != 2)
            c <- (Some(3))
            yield a + b + c
        };

        assert_eq!(r, None)
    }
}

#[cfg(test)]
mod code_with_procedural {
    use procedural::foreach;

    #[test]
    pub fn should_perform_lists_foreach_with_filter() {
        let r: Vec<_> = foreach! {
            a <- (1..=3)
            b <- (1..=a) if b != 2
            yield (a, b)
        }
        .collect();

        assert_eq!(r, vec![(1, 1), (2, 1), (3, 1), (3, 3)]);
    }

    #[test]
    #[allow(unused_variables)]
    pub fn should_perform_option_foreach_with_filter() {
        let r = foreach! {
            a <- Some(2)
            b <- Some(a) if a != 2
            c <- Some(3)
            yield a + b + c
        };

        assert_eq!(r, None)
    }
}
