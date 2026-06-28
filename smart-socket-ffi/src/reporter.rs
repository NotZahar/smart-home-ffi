use std::marker::PhantomData;

use crate::report::Report;

pub struct Reporter<'a, Items = ()> {
    items: Items,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Reporter<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: (),
            _lifetime: PhantomData,
        }
    }
}

impl<'a> Default for Reporter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Items> Reporter<'a, Items> {
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn add<T>(self, item: &'a T) -> Reporter<'a, (Items, &'a T)>
    where
        T: Report,
    {
        Reporter {
            items: (self.items, item),
            _lifetime: PhantomData,
        }
    }
}

impl<'a, Items> Reporter<'a, Items>
where
    Items: ReportItems,
{
    pub fn report(&self) -> String {
        let mut output = String::from("Report:\n");
        let mut index = 0;
        self.items.append_report(&mut output, &mut index);
        print!("{output}");
        output
    }
}

pub trait ReportItems {
    fn append_report(&self, output: &mut String, index: &mut usize);
}

impl ReportItems for () {
    fn append_report(&self, _output: &mut String, _index: &mut usize) {}
}

impl<PreviousItems, Item> ReportItems for (PreviousItems, &Item)
where
    PreviousItems: ReportItems,
    Item: Report,
{
    fn append_report(&self, output: &mut String, index: &mut usize) {
        let (previous, item) = self;
        previous.append_report(output, index);

        *index += 1;
        output.push_str(&format!("Item #{index}:\n"));
        for line in item.report().lines() {
            output.push_str("  ");
            output.push_str(line);
            output.push('\n');
        }
    }
}
