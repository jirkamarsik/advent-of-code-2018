pub fn iter_dep_product<Outer, Inner, F>(
    mut outer: Outer,
    inner_generator: F,
) -> impl Iterator<Item = (Outer::Item, Inner::Item)>
where
    Outer: Iterator,
    Inner: Iterator,
    F: Fn(Outer::Item) -> Inner,
    Outer::Item: Copy,
{
    let state = outer.next().map(|o| (o, inner_generator(o)));
    IterDepProduct {
        outer,
        inner_generator,
        state,
    }
}

pub fn iter_product<Outer, Inner>(
    outer: Outer,
    inner: Inner,
) -> impl Iterator<Item = (Outer::Item, Inner::Item)>
where
    Outer: Iterator,
    Inner: Iterator + Clone,
    Outer::Item: Copy,
{
    iter_dep_product(outer, move |_| inner.clone())
}

pub struct IterDepProduct<Outer, Inner, F>
where
    Outer: Iterator,
    Inner: Iterator,
    F: Fn(Outer::Item) -> Inner,
    Outer::Item: Copy,
{
    outer: Outer,
    inner_generator: F,
    state: Option<(Outer::Item, Inner)>,
}

impl<Outer, Inner, F> Iterator for IterDepProduct<Outer, Inner, F>
where
    Outer: Iterator,
    Inner: Iterator,
    F: Fn(Outer::Item) -> Inner,
    Outer::Item: Copy,
{
    type Item = (Outer::Item, Inner::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match self.state.as_mut() {
            Some((o, inner)) => match inner.next() {
                Some(i) => Some((*o, i)),
                None => {
                    self.state = self.outer.next().map(|o| (o, (self.inner_generator)(o)));
                    self.next()
                }
            },
            None => None,
        }
    }
}
