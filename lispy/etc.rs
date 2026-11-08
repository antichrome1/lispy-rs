macros! {
    macro prog1($first:expr; $($rest:stmt;)*) {
        let value = $first;
        $($rest)*
            value
    } @export @wrap

    macro prog2($first:expr; $second:expr; $($rest:stmt;)*) {
        let _ = $first;
        let value = $second;
        $($rest)*
        value
    } @export @wrap
}

#[cfg(test)]
mod prog_tests {

    #[test]
    fn prog1_returns_first_value_after_running_rest() {
        let mut events = Vec::new();

        let result = prog1! {
            {
                events.push("first");
                10
            };
            events.push("second");
            events.push("third");
        };

        assert_eq!(result, 10);
        assert_eq!(events, vec!["first", "second", "third"]);
    }

    #[test]
    fn prog2_returns_second_value_after_running_rest() {
        let mut events = Vec::new();

        let result = prog2! {
            {
                events.push("first");
                10
            };
            {
                events.push("second");
                20
            };
            events.push("third");
        };

        assert_eq!(result, 20);
        assert_eq!(events, vec!["first", "second", "third"]);
    }

    #[test]
    fn prog1_runs_without_rest_expressions() {
        let result = prog1! {
            42;
        };

        assert_eq!(result, 42);
    }

    #[test]
    fn prog2_runs_without_rest_expressions() {
        let result = prog2! {
            "ignored";
            "returned";
        };

        assert_eq!(result, "returned");
    }
}
