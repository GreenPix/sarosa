
/// Macro using
///
/// Can be used to emulate with function calls
/// the `using` keyword from the JAI language.
///
/// For more information on the idea behind the macro:
/// See this [reddit thread](http://www.reddit.com/r/rust/comments/2t6xqz/jai_demo_dataoriented_features_soa_crazy_using/)
macro_rules! using {
    ($user:ident use $inner:ident { $( $accessor:ident: $tt:ty ),* }) => {
        impl $user {

            $(
            #[inline]
            fn $accessor(&self) -> $tt {
                self.$inner.$accessor
            })*
        }
    };

    ($user:ident use $a:ident.$b:ident { $( $accessor:ident: $tt:ty ),* }) => {
        impl $user {

            $(
            #[inline]
            fn $accessor(&self) -> $tt {
                self.$a.$b.$accessor
            })*
        }
    };

    ($user:ident use $a:ident.$b:ident.$c:ident { $( $accessor:ident: $tt:ty ),* }) => {
        impl $user {

            $(
            #[inline]
            fn $accessor(&self) -> $tt {
                self.$a.$b.$c.$accessor
            })*
        }
    };
}
