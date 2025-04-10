#![allow(unused_imports)]

mod crd;
use error::*;
use lib::*;

#[cfg(test)]
mod tests {
    use lib::PrimaryResource;

    use super::crd::*;

    #[test]
    fn test_initialize_status() {
        let mut db = Database::default();
        assert!(db.status.is_none());

        db.initialize_status();
        assert!(db.status.is_some());
    }
}
