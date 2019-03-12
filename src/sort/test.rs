use crate::sort::bubble::{sort, sort_by};
use crate::sort::insertion::{sort as i_sort, sort_by as i_sort_by};
use std::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Debug, Clone)]
struct Employee {
    id: i32,
    name: String,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Employee) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Employee {
    fn eq(&self, other: &Employee) -> bool {
        self.id == other.id && self.name == other.name
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Employee) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for Employee {}

#[test]
fn bubble_sort_int() {
    let mut arr = [2, 1, 3, 5, 4];
    sort(&mut arr);
    assert_eq!(arr, [1, 2, 3, 4, 5]);
}

#[test]
fn bubble_sort_struct() {
    let mut arr = [
        Employee {
            id: 2,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 1,
            name: "Khan".to_string(),
        },
    ];
    sort(&mut arr);
    assert_eq!(
        arr,
        [
            Employee {
                id: 1,
                name: "Khan".to_string(),
            },
            Employee {
                id: 2,
                name: "Abrar".to_string(),
            }
        ]
    );
}

#[test]
fn bubble_sort_by_fn() {
    let mut arr = [
        Employee {
            id: 2,
            name: "Khan".to_string(),
        },
        Employee {
            id: 1,
            name: "Abrar".to_string(),
        },
    ];
    sort_by(&mut arr, |a: &Employee, b: &Employee| a.name.cmp(&b.name));
    assert_eq!(
        arr,
        [
            Employee {
                id: 1,
                name: "Abrar".to_string(),
            },
            Employee {
                id: 2,
                name: "Khan".to_string(),
            }
        ]
    );
}

#[test]
pub fn insertion_sort_int() {
    let mut arr = vec![1, 2, 3, 3, 3, 3, 4, 3, 1, 2, 4, 3, -1];
    i_sort(&mut arr);
    assert_eq!(arr, vec![-1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3, 4, 4]);
}

#[test]
pub fn insertion_sort_str() {
    let mut arr = vec!["zasda", "dasd", "dasd"];
    i_sort(&mut arr);
    assert_eq!(arr, vec!["dasd", "dasd", "zasda"]);
}

#[test]
pub fn insertion_sort_struct() {
    let mut arr = [
        Employee {
            id: 2,
            name: "Abrar".to_string(),
        },
        Employee {
            id: 1,
            name: "Khan".to_string(),
        },
    ];
    i_sort(&mut arr);
    assert_eq!(
        arr,
        [
            Employee {
                id: 1,
                name: "Khan".to_string(),
            },
            Employee {
                id: 2,
                name: "Abrar".to_string(),
            }
        ]
    );
}

#[test]
pub fn insertion_sort_struct_by() {
    let mut arr = [
        Employee {
            id: 2,
            name: "Khan".to_string(),
        },
        Employee {
            id: 1,
            name: "Abrar".to_string(),
        },
    ];
    i_sort_by(&mut arr, |a: &Employee, b: &Employee| a.name.cmp(&b.name));
    assert_eq!(
        arr,
        [
            Employee {
                id: 1,
                name: "Abrar".to_string(),
            },
            Employee {
                id: 2,
                name: "Khan".to_string(),
            }
        ]
    );
}
