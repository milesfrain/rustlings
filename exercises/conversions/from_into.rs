// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation
// in order for the line `let p = Person::from("Mark,20")` to compile
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of Person
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return the default of Person
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return the default of Person
// Otherwise, then return an instantiated Person object with the results

// Could be pretty clean solution with collect_tuple()
// but need to install this crate
// use itertools::Itertools;
// https://stackoverflow.com/a/61555349

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        // let res = s.split(',').collect_tuple();
        // let res = s.split(',').collect();
        // let v = vec![1, 2, 3];
        // match &v[..] {

        let mut i = s.split(',');
        // question - how to chain Option and Result (downgrade / hush to Option)
        // Is closure required?
        /*
        match (i.next(), (|| {i.next()?.parse::<usize>()})(), i.next()) {
            (Some(first), Ok(age), None) => Person {name: String::from(first), age:5},
            // &[first, second, ..] => Person::default(),
            // &[first, second, ..] => Person {name: String::from(first), age:5},
            // Some((name, age)) => Person::default(),
            _ => Person::default(),
        };
        */
        // Would be nice to be able to chain all of these together
        // https://github.com/rust-lang/rfcs/pull/2497
        // if let Some(name) = i.next() && name.len() != 0 {
        if let Some(name) = i.next() {
            if name.len() != 0 {
                if let Some(age_str) = i.next() {
                    if let Ok(age) = age_str.parse::<usize>() {
                        if i.next() == None {
                            return Person {name: String::from(name), age};
                        }
                    }
                }
            }
        }
        Person::default()
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
