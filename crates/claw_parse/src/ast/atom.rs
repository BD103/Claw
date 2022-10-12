//! Small pieces of data that help make up the [`AST`](crate::ast).

use super::{Expr, Span};

macro_rules! spanned {
    {
        $(#[$outer:meta])*
        $scope:vis struct $name:ident($inner:ty);
    } => {
        $(#[$outer])*
        $scope struct $name($inner, Span);

        impl $name {
            #[inline]
            pub fn inner(&self) -> &$inner {
                &self.0
            }

            #[inline]
            pub fn span(&self) -> &Span {
                &self.1
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                self.inner()
            }
        }
    };
    {
        $(#[$outer:meta])*
        $scope:vis struct $name:ident(mut $($modifiers:ident)* => $inner:ty);
    } => {
        spanned! {
            $(#[$outer])*
            $scope struct $name($($modifiers =>)* $inner);
        }

        impl $name {
            #[inline]
            pub fn inner_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.inner_mut()
            }
        }
    };
    {
        $(#[$outer:meta])*
        $scope:vis struct $name:ident(new $($modifiers:ident)* => $inner:ty);
    } => {
        spanned! {
            $(#[$outer])*
            $scope struct $name($($modifiers =>)* $inner);
        }

        impl $name {
            #[inline]
            pub fn new(value: $inner, span: Span) -> Self {
                $name(value, span)
            }
        }
    };
}

spanned! {
    /// An identifier, very similar to [`TokenKind::Ident`](crate::lex::TokenKind::Ident).
    #[derive(Clone, Debug)]
    pub struct Ident(new => String);
}

spanned! {
    /// A body of [`Expr`]essions.
    #[derive(Clone, Debug)]
    pub struct Body(new => Vec<Expr>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_macro() {
        spanned! {
            pub struct MySpanned(u8);
        }

        let my_spanned = MySpanned(12, 0..2);

        assert_eq!(*my_spanned.inner(), 12);
        assert_eq!(*my_spanned.span(), 0..2);

        // Test Deref
        assert_eq!(*my_spanned, 12);
    }

    #[test]
    fn span_mut_macro() {
        spanned! {
            pub struct MyMutSpanned(mut => i8);
        }

        let mut my_mut_spanned = MyMutSpanned(-33, 0..3);

        assert_eq!(*my_mut_spanned.inner(), -33);
        assert_eq!(*my_mut_spanned.span(), 0..3);

        // Test mutability
        *my_mut_spanned.inner_mut() = 110;

        // Test Deref and DerefMut
        assert_eq!(*my_mut_spanned, 110);

        *my_mut_spanned = -36;

        assert_eq!(*my_mut_spanned, -36);
    }

    #[test]
    fn span_new_macro() {
        spanned! {
            struct MyNewSpanned(new => u16);
        }

        let my_new_spanned = MyNewSpanned::new(84, 0..2);

        assert_eq!(*my_new_spanned, 84);
        assert_eq!(*my_new_spanned.span(), 0..2);
    }

    #[test]
    fn span_new_mut_macro() {
        spanned! {
            pub(crate) struct MyNewMutSpanned(new mut => i16);
        }

        spanned! {
            pub(crate) struct MyMutNewSpanned(mut new => i16);
        }

        let mut my_new_mut_spanned = MyNewMutSpanned::new(4570, 0..4);
        let mut my_mut_new_spanned = MyMutNewSpanned::new(4570, 0..4);

        *my_new_mut_spanned = 3000;
        *my_mut_new_spanned = 3000;

        let inner_results = (3000, *my_new_mut_spanned, *my_mut_new_spanned);
        let span_results = (
            0..4,
            my_new_mut_spanned.span().clone(),
            my_mut_new_spanned.span().clone(),
        );

        assert_eq!(inner_results.0, inner_results.1);
        assert_eq!(inner_results.0, inner_results.2);
        assert_eq!(inner_results.1, inner_results.2);

        assert_eq!(span_results.0, span_results.1);
        assert_eq!(span_results.0, span_results.2);
        assert_eq!(span_results.1, span_results.2);
    }

    #[test]
    fn ident() {
        let original_text = "hah_funny_pun".to_string();
        let mydent = Ident::new(original_text.clone(), 0..13);

        assert_eq!(*mydent, original_text);
    }
}
