use proc_macro2::{LineColumn, Span};

pub struct Spaced<U, V>(pub U, pub LineColumn, pub V);

impl<U, V> syn::parse::Parse for Spaced<U, V>
where
    U: syn::parse::Parse,
    V: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let before_span = input.span();
        let first = input.parse::<U>()?;
        let middle_span = input.span();
        let second = input.parse::<V>()?;
        let spacing = LineColumn {
            line: middle_span.start().line - before_span.end().line,
            column: middle_span.start().column - before_span.end().column,
        };
        Ok(Spaced(first, spacing, second))
    }
}

pub struct Spacing<LC: LineColumnPredicate> {
    pub line: usize,
    pub column: usize,
    _marker: std::marker::PhantomData<LC>,
}

impl<LC: LineColumnPredicate> Spacing<LC> {
    fn from_spans(span_before: Span, span_after: Span) -> Self {
        let line = span_after.start().line.abs_diff(span_before.end().line);
        let column = if line == 0 {
            span_after.start().column.abs_diff(span_before.end().column)
        } else {
            span_after.start().column
        };
        Spacing {
            line,
            column,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<LC: LineColumnPredicate> std::fmt::Debug for Spacing<LC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Spacing")
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}

#[derive(Debug)]
pub struct SpacedBy<U, LC: LineColumnPredicate, V>(pub U, pub Spacing<LC>, pub V);

impl<U, LC: LineColumnPredicate, V> syn::parse::Parse for SpacedBy<U, LC, V>
where
    U: syn::parse::Parse,
    V: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let span0 = input.span();
        println!("After first: {:?}", span0.end());
        let first = input.parse::<U>()?;

        let span1 = input.span();
        println!("Before second: {:?}", span1.start());
        let second = input.parse::<V>()?;

        let line_diff = span1.start().line.abs_diff(span0.end().line);
        let spacing = Spacing::<LC> {
            line: line_diff,
            column: if line_diff == 0 {
                span1.start().column.abs_diff(span0.end().column)
            } else {
                span1.start().column
            },
            _marker: std::marker::PhantomData,
        };
        LC::assert_line_column(&spacing, span1)?;
        Ok(SpacedBy(first, spacing, second))
    }
}

pub trait LineColumnPredicate {
    fn assert_line_column(lc: &Spacing<Self>, span: Span) -> syn::Result<()>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Exactly<const L: usize, const C: usize>;

impl<const L: usize, const C: usize> LineColumnPredicate for Exactly<L, C> {
    fn assert_line_column(lc: &Spacing<Self>, span: Span) -> syn::Result<()> {
        if lc.line == L && lc.column == C {
            Ok(())
        } else {
            Err(syn::Error::new(
                span,
                format!("Expected exactly {} lines and {} columns of spacing, found {} lines and {} columns", L, C, lc.line, lc.column),
            ))
        }
    }
}

pub type NoSpacing = Exactly<0, 0>;
pub type Spaces<const N: usize> = Exactly<0, N>;
pub type Newlines<const N: usize> = Exactly<N, 0>;
pub struct AnySpacing;

impl LineColumnPredicate for AnySpacing {
    fn assert_line_column(_lc: &Spacing<Self>, _span: Span) -> syn::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PunctuatedSpacedBy<
    T,
    LC0: LineColumnPredicate,
    P: syn::parse::Parse,
    LC1: LineColumnPredicate = LC0,
> {
    elements: Vec<T>,
    _marker: std::marker::PhantomData<(LC0, P, LC1)>,
}

impl<T, LC0: LineColumnPredicate, P: syn::parse::Parse, LC1: LineColumnPredicate>
    PunctuatedSpacedBy<T, LC0, P, LC1>
where
    T: syn::parse::Parse,
{
    fn parse_internal(input: syn::parse::ParseStream<'_>, terminated: bool) -> syn::Result<Self> {
        let mut elements = Vec::new();

        if input.is_empty() {
            return Ok(PunctuatedSpacedBy {
                elements,
                _marker: std::marker::PhantomData,
            });
        }

        // Capture span BEFORE parsing first element
        let mut span_elem = input.span();
        let element = input.parse::<T>()?;
        elements.push(element);

        while !input.is_empty() {
            // Capture span BEFORE parsing punct
            let span_punct = input.span();
            // LC0: Check spacing BEFORE punct (from element to punct)
            let spacing = Spacing::from_spans(span_elem, span_punct);
            LC0::assert_line_column(&spacing, span_punct)?;

            let _punct: P = input.parse()?;

            if input.is_empty() {
                if terminated {
                    break;
                }
                return Err(syn::Error::new(
                    span_punct,
                    "Expected another element after punctuation, found end of input",
                ));
            }

            // Capture span BEFORE parsing next element
            span_elem = input.span();
            // LC1: Check spacing AFTER punct (from punct to element)
            let spacing = Spacing::from_spans(span_punct, span_elem);
            LC1::assert_line_column(&spacing, span_elem)?;

            let element = input.parse::<T>()?;
            elements.push(element);
        }

        Ok(PunctuatedSpacedBy {
            elements,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Self::parse_internal(input, false)
    }

    pub fn parse_terminated(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Self::parse_internal(input, true)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_str, punctuated::Punctuated, LitBool, Token};

    #[test]
    fn test_parse_spaced() {
        let input: Spaced<LitBool, LitBool> = parse_str("true false").unwrap();
        let Spaced(field, spacing, field2) = input;
        assert!(field.value);
        assert_eq!(spacing, LineColumn { line: 0, column: 1 });
        assert!(!field2.value);
    }

    #[test]
    fn test_parse_spaced_by_no_spaces() {
        let input: SpacedBy<Token![#], NoSpacing, LitBool> = parse_str("#false").unwrap();
        let SpacedBy(_, _, field2) = input;
        assert!(!field2.value);
    }

    #[test]
    fn test_parse_spaced_by_one_space() {
        let input: SpacedBy<Token![#], Spaces<1>, LitBool> = parse_str("# false").unwrap();
        let SpacedBy(_, _, field2) = input;
        assert!(!field2.value);
    }

    #[test]
    fn test_parse_spaced_by_2_newlines() {
        let input: SpacedBy<Token![#], Newlines<2>, LitBool> = parse_str("#\n\nfalse").unwrap();
        let SpacedBy(_, _, field2) = input;
        assert!(!field2.value);
    }

    #[test]
    fn test_parse_spaced_by_no_spaces_error() {
        let r: syn::Result<SpacedBy<Token![#], NoSpacing, LitBool>> = parse_str("# false");
        let e = r.unwrap_err();
        assert_eq!(e.span().start(), LineColumn { line: 1, column: 2 });
        assert_eq!(e.span().end(), LineColumn { line: 1, column: 7 });
    }

    #[test]
    fn test_parse_spaced_by_one_space_error() {
        let r: syn::Result<SpacedBy<Token![#], Spaces<1>, LitBool>> = parse_str("#false");
        let e = r.unwrap_err();
        assert_eq!(e.span().start(), LineColumn { line: 1, column: 1 });
        assert_eq!(e.span().end(), LineColumn { line: 1, column: 6 });
    }

    #[test]
    fn test_parse_spaced_by_2_newlines_error() {
        let r: syn::Result<SpacedBy<Token![#], Newlines<2>, LitBool>> = parse_str("#\nfalse");
        let e = r.unwrap_err();
        assert_eq!(e.span().start(), LineColumn { line: 2, column: 0 });
        assert_eq!(e.span().end(), LineColumn { line: 2, column: 5 });
    }

    #[test]
    fn test_parse_punctuated_spaced_by() {
        struct S {
            punc: PunctuatedSpacedBy<LitBool, Spaces<1>, Token![,], Spaces<1>>,
        }
        impl syn::parse::Parse for S {
            fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
                let punc = PunctuatedSpacedBy::parse(input)?;
                Ok(S { punc })
            }
        }
        let input: S = parse_str("true , false , true").unwrap_or_else(|e| {
            panic!(
                "Failed to parse: {} at {}:{}-{}:{}",
                e,
                e.span().start().line,
                e.span().start().column,
                e.span().end().line,
                e.span().end().column,
            )
        });
        let elements = input.punc.elements;
        assert_eq!(elements.len(), 3);
        assert!(elements[0].value);
        assert!(!elements[1].value);
        assert!(elements[2].value);
    }
}
