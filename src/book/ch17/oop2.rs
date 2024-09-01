use crate::book::ch17::oop::AveragedCollection;

#[test]
fn code1() {
    let mut ac = AveragedCollection::new();

    // to access the method from trait we need to import it
    use crate::book::ch17::oop::HasAverage;
    let avg = ac.average();

    dbg!(avg); // None

    // no access outside the module
    // dbg!(ac.average);
}
