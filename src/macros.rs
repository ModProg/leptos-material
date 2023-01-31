macro_rules! dbg {
    ($expr:expr) => {{
        let expr = $expr;
        log!(concat!(stringify!($expr), " = {:?}"), expr);
        expr
    }};
}

macro_rules! classes {
    (#insert $classes:ident, $lit:literal) => {
            $classes.push_str(concat!($lit, " "))
    };
    (#insert $classes:ident, $ident:ident) => {
        if $ident {
            $classes.push_str(concat!(stringify!($ident), " "))
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

macro_rules! styles {
    (#insert $styles:ident, $lit:literal) => {
            $styles.push_str(concat!($lit, ";"))
    };
    (#insert $styles:ident, ($ident:ident?)) => {
        if let Some($ident) = $ident {
            $styles.push_str(&format!("{}: {};", const_str::replace!(stringify!($ident), '_', "-"), $ident));
        }
    };
    [$($style:tt),*$(,)?] => {
        {
            let mut styles = String::new();
            $(styles!(#insert styles, $style);)*
            styles
        }
    };
}
