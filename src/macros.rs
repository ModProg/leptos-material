macro_rules! dbg {
    ($expr:expr) => {{
        let expr = $expr;
        log!(concat!(stringify!($expr), " = {:?}"), expr);
        expr
    }};
}

macro_rules! classes {
    (#insert $classes:ident, $lit:literal) => {
            $classes.push_str(concat!(" ", $lit))
    };
    (#insert $classes:ident, $ident:ident) => {
        if $ident {
            $classes.push_str(concat!(" ", stringify!($ident)))
        }
    };
    [$($class:tt),*$(,)?] => {
        {
            let mut classes = String::new();
            $(classes!(#insert classes, $class);)*
            classes
        }
    };
}
