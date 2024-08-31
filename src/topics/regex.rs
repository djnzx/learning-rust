use regex::Regex;

#[test]
fn code1() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let matches = re.is_match("2014-01-01");
    println!("Did our date match? {}", matches);
}
