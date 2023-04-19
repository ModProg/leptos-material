// macro_rules! dbg {
//     ($expr:expr) => {{
//         let expr = $expr;
//         log!(concat!(stringify!($expr), " = {:?}"), expr);
//         expr
//     }};
// }

macro_rules! classes {
    (@insert $classes:ident,) => {};
    (@insert $classes:ident, $lit:literal $(, $($tts:tt)*)?) => {
        $classes.push_str(concat!($lit, " "));
        classes!(@insert $classes, $($($tts)*)?);
    };
    (@insert $classes:ident, $string:ident $(, $($tts:tt)*)?) => {
        $classes.push_str(&format!("{} ", $string));
        classes!(@insert $classes, $($($tts)*)?);
    };
    (@insert $classes:ident, $option:ident? $(, $($tts:tt)*)?) => {
        if let Some(option) = $option {
            $classes.push_str(&format!("{} ", option));
        }
        classes!(@insert $classes, $($($tts)*)?);
    };
    [$($tts:tt)*] => {
        {
            let mut classes = String::new();
            classes!(@insert classes, $($tts)*);
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
