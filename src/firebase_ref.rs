use std::iter::Iterator;

pub struct Ref {
    ref_location: Vec<String>
}

impl Ref {
    pub fn db_ref(&self, path: &str) -> Ref {
        let mut new_path = self.ref_location.clone();
        new_path.append(&mut Ref::sanitize(path));
        Ref { ref_location: new_path }
    }

    pub fn new(path: &str) -> Ref {
        Ref { ref_location: Ref::sanitize(path) }
    }

    fn sanitize(path: &str) -> Vec<String> {
        let splits = path.split("/");
        splits
            .filter(|s| s.len() > 0)
            .map(|s| String::from(s))
            .collect()
    }

    pub fn path_string(&self) -> String {
        self.ref_location.join("/").to_string()
    }
}

#[cfg(test)]
mod ref_test {
    use super::*;

    #[test]
    fn new_ref() {
        let original_ref = Ref::new("/a/b/c");
        let new_ref = original_ref.db_ref("d/e/f");
        assert_eq!(vec!["a", "b", "c", "d", "e", "f"], new_ref.ref_location);
    }

    #[test]
    fn new_ref_no_leading() {
        let original_ref = Ref::new("a");
        let new_ref = original_ref.db_ref("b/c");
        assert_eq!(vec!["a", "b", "c"], new_ref.ref_location);
    }

    #[test]
    fn new_ref_original_trailing() {
        let original_ref = Ref::new("a/");
        let new_ref = original_ref.db_ref("b/c");
        assert_eq!(vec!["a", "b", "c"], new_ref.ref_location);
    }

    #[test]
    fn new_ref_original_trailing_new_leading() {
        let original_ref = Ref::new("a/");
        let new_ref = original_ref.db_ref("/b/c");
        assert_eq!(vec!["a", "b", "c"], new_ref.ref_location);
    }

}
