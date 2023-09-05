/// Creates a new HashMap using a literal-like syntax. It automatically
/// performs `Into` conversions for convenience.
/// 
/// Source: https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
#[macro_export]
macro_rules! hashmap {
    [ $($key:expr => $value:expr),* ] => {{
        let mut m = ::std::collections::HashMap::new();
        $(
            m.insert($key.into(), $value.into());
        )+
        m
    }}
}

/// Creates a new HashSet using a literal-like syntax.
#[macro_export]
macro_rules! hashset {
    [ $($value:expr),* ] => {{
        let mut m = ::std::collections::HashSet::new();
        $(
            m.insert($value.into());
        )+
        m
    }}
}
