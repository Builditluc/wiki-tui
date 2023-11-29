#[macro_export]
macro_rules! has_modifier {
    ($key: expr, Modifier::$modifier: ident) => {
        $key.modifiers == KeyModifiers::$modifier
    };
}
