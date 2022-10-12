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
