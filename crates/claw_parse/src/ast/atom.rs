use super::Span;

macro_rules! spanned {
    {
        $(#[$outer:meta])*
        $scope:vis struct $name:ident($inner:ty);
    } => {
        $(#[$outer])*
        $scope struct $name($inner, Span);

        impl $name {
            pub fn new(inner: $inner, span: Span) -> Self {
                $name(inner, span)
            }

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
        $scope:vis struct $name:ident(mut $inner:ty);
    } => {
        spanned! {
            $(#[$outer])*
            $scope struct $name($inner);
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
}

spanned! {
    /// An identifier, very similar to [`TokenKind::Ident`](crate::lex::TokenKind::Ident).
    #[derive(Clone, Debug)]
    pub struct Ident(mut String);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_macro() {
        spanned! {
            pub struct MySpanned(u8);
        }

        let my_spanned = MySpanned::new(12, 0..2);

        assert_eq!(*my_spanned.inner(), 12);
        assert_eq!(*my_spanned.span(), 0..2);

        // Test Deref
        assert_eq!(*my_spanned, 12);
    }

    #[test]
    fn span_mut_macro() {
        spanned! {
            pub struct MyMutSpanned(mut i8);
        }

        let mut my_mut_spanned = MyMutSpanned::new(-33, 0..3);

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
    fn ident() {
        let original_text = "hah_funny_pun".to_string();
        let mydent = Ident::new(original_text.clone(), 0..13);

        assert_eq!(*mydent, original_text);
    }
}
