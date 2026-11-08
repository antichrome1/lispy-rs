macros! {
    /// Run the body exactly N times.
    macro ntimes($n:expr => $body:block) {
        for _ in 0..$n {
            $body
        }
    } @export @wrap

    macro dotimes(($var:ident, $count:expr $(, $result:expr)?) $($body:tt)*) {
       let __count: usize = $count;
        #[allow(unused_variables)]
        for $var in 0..__count {
            $($body)*
        }
        #[allow(unused_variables)]
        let $var = __count;
        $($result)?
    } @export @wrap

    /// Run the body when the condition is true.
    macro when($condition:expr, $body:block) {
        if $condition {
            Some($body)
        } else {
            None
        }
    } @export @wrap

    /// Run the body when the condition is false.
    macro unless($condition:expr, $body:block) {
        if !$condition {
            Some($body)
        } else {
            None
        }
    } @export @wrap
}

#[macro_export]
macro_rules! incf {
    ($place:expr) => {
        incf!($place, 1)
    };
    ($place:expr, $amount:expr) => {{
        let target = &mut $place;
        *target += $amount;
        *target
    }};
}

#[macro_export]
macro_rules! decf {
    ($place:expr) => {
        decf!($place, 1)
    };
    ($place:expr, $amount:expr) => {{
        let target = &mut $place;
        *target -= $amount;
        *target
    }};
}


#[cfg(test)]
mod when_unless_tests {
    #[test]
    fn when_returns_body_value_only_if_true() {
        assert_eq!(when!(2 > 1, { "yes" }), Some("yes"));
        assert_eq!(when!(1 > 2, { "yes" }), None);
    }

    #[test]
    fn unless_returns_body_value_only_if_false() {
        assert_eq!(unless!(1 > 2, { 42 }), Some(42));
        assert_eq!(unless!(2 > 1, { 42 }), None);
    }
}

#[cfg(test)]
mod incf_decf_tests {
    #[test]
    fn incf_increments_and_returns_new_value() {
        let mut count = 4;

        let result = incf!(count);

        assert_eq!(result, 5);
        assert_eq!(count, 5);
    }

    #[test]
    fn incf_adds_an_amount() {
        let mut count = 4;

        let result = incf!(count, 3);

        assert_eq!(result, 7);
        assert_eq!(count, 7);
    }

    #[test]
    fn decf_decrements_and_returns_new_value() {
        let mut count = 4;

        let result = decf!(count);

        assert_eq!(result, 3);
        assert_eq!(count, 3);
    }

    #[test]
    fn decf_subtracts_an_amount() {
        let mut count = 10;

        let result = decf!(count, 4);

        assert_eq!(result, 6);
        assert_eq!(count, 6);
    }

    #[test]
    fn macros_work_on_indexed_places() {
        let mut values = [10, 20];

        assert_eq!(incf!(values[0], 2), 12);
        assert_eq!(decf!(values[1], 5), 15);
        assert_eq!(values, [12, 15]);
    }
}

#[cfg(test)]
mod ntimes_dotimes_tests {
    use std::cell::Cell;

    #[test]
    fn runs_exactly_n_times() {
        let mut runs = 0;
        dotimes!((_i, 5) {
            runs += 1;
        });
        assert_eq!(runs, 5);
    }

    #[test]
    fn zero_count_runs_nothing() {
        let mut runs = 0;
        dotimes!((_i, 0) {
            runs += 1;
        });
        assert_eq!(runs, 0);
    }

    #[test]
    fn index_goes_from_zero_to_n_minus_one() {
        let mut seen = Vec::new();
        dotimes!((i, 4) {
            seen.push(i);
        });
        assert_eq!(seen, vec![0, 1, 2, 3]);
    }

    #[test]
    fn count_expression_is_evaluated_once() {
        let calls = Cell::new(0);
        let count = || {
            calls.set(calls.get() + 1);
            3
        };
        let mut runs = 0;
        dotimes!((_i, count()) {
            runs += 1;
        });
        assert_eq!(runs, 3);
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn count_can_be_an_arbitrary_expression() {
        let mut runs = 0;
        dotimes!((_i, 2 + 1) {
            runs += 1;
        });
        assert_eq!(runs, 3);
    }

    #[test]
    fn break_stops_the_loop() {
        let mut runs = 0;
        dotimes!((i, 10) {
            if i == 3 {
                break;
            }
            runs += 1;
        });
        assert_eq!(runs, 3);
    }

    #[test]
    fn continue_skips_an_iteration() {
        let mut seen = Vec::new();
        dotimes!((i, 5) {
            if i % 2 == 0 {
                continue;
            }
            seen.push(i);
        });
        assert_eq!(seen, vec![1, 3]);
    }

    #[test]
    fn multiple_body_statements_run_in_order() {
        let mut log = Vec::new();
        dotimes!((i, 2) {
            log.push(i * 10);
            log.push(i * 10 + 1);
        });
        assert_eq!(log, vec![0, 1, 10, 11]);
    }

    #[test]
    fn result_form_sees_var_bound_to_count() {
        let v = dotimes!((i, 4, i * 10) {});
        assert_eq!(v, 40);
    }

    #[test]
    fn no_result_form_returns_unit() {
        let v: () = dotimes!((i, 2) {
            let _ = i;
        });
        assert_eq!(v, ());
    }
}
