pub(crate) trait CurriedItem<'a> {
    type Item;
}

pub(crate) trait LendingIterator {
    type Item: ?Sized + for<'a> CurriedItem<'a>;

    fn next<'a>(&'a mut self) -> Option<<Self::Item as CurriedItem<'a>>::Item>;
}
