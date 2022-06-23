//! # Unless
//! Provides [`Within`] to any string iterator, which allows grouping of
//! characters. See the documentation for [`WithinSearch::within`] for more.

pub struct Within<'a, Iter, P> {
    inner: Iter,
    matcher: P,
    curr: Option<&'a str>,
    id: usize,
}

pub trait WithinSearch<'a, Iter, P> {
    /// Groups elements between specified characters.
    ///
    /// For example:
    /// ```no_run
    /// fn main() {
    /// 	let test = "This is a \"string with multiple elements\""
    ///         .within(|c| match c { '\"' => true, _ => false })
    ///         .split(" ");
    /// 	assert_eq!(test.next(), Some("This"));
    /// 	assert_eq!(test.next(), Some("is"));
    /// 	assert_eq!(test.next(), Some("a"));
    /// 	assert_eq!(test.text(), Some("\"string with multiple elements\""));
    /// 	assert_eq!(test.next(), None);
    /// }
    /// ```
    fn within(self, matcher: P) -> Within<'a, Iter, P>;
}

impl<'a, Iter, P> WithinSearch<'a, Iter, P> for Iter
where
    Iter: Iterator<Item = &'a str>,
    P: FnMut(char) -> bool,
{
    fn within(mut self, matcher: P) -> Within<'a, Iter, P> {
        let curr = self.next();
        Within {
            inner: self,
            matcher,
            curr,
            id: 0,
        }
    }
}

impl<'a, Iter, P> Iterator for Within<'a, Iter, P>
where
    Iter: Iterator<Item = &'a str>,
    P: FnMut(char) -> bool,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
