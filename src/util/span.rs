use std::cmp::{min, max};
use std::ops::Range;

/// A range from `start` to `end` that supports set operations with other `Span`s
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end:   usize,
}
impl Span {

    /// Returns a `Span` based on two values in ascending order
    pub fn new(a:usize, b:usize) -> Span {
        if a < b {
            Span {start: a, end: b}
        } else {
            Span {start: b, end: a}
        }
    }

    /// Returns the length of the `Span`
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the `Span` as a `Range`
    pub fn to_range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Checks if another `Span` intersects with this `Span`
    pub fn intersects(&self, other: &Span) -> bool {
        !(self.end <= other.start || self.start >= other.end)
    }

    /// Checks if another `Span` is fully within this `Span`'s bounds
    pub fn contains(&self, other: &Span) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Checks if a `usize` value is within this `Span`'s bounds
    pub fn contains_value(&self, value: usize) -> bool {
        self.start <= value && value < self.end
    }

    /// Returns a new`Span` covering the intersecting values
    pub fn intersection(&self, other: &Span) -> Option<Span> {
        // a ......|--------|....
        // b ...|-----|..........
        // = ......|==|..........
        if self.intersects(other) {
            Some(Span {
                start: max(self.start, other.start),
                end: min(self.end, other.end),
            })
        } else { None }
    }

    /// Returns a new`Span` covering the combined values
    pub fn union(&self, other:&Span) -> Option<Span> {
        // a ......|--------|....
        // b ...|-----|..........
        // = ...|===========|....
        if self.intersects(other) {
            Some(Span {
                start: min(self.start, other.start),
                end: max(self.end, other.end),
            })
        } else { None }
    }

    /// Returns a list of new`Span`s containing the difference
    /// between this and another `Span`
    pub fn subtraction(&self, other: &Span) -> Option<Vec<Span>> {
        // a ...|-----------|....
        // b ......|-----|.......
        // = ...|==|.....|==|....
        if other.contains(self) {
            None
        } else if !self.intersects(other) {
            Some(vec![self.clone()])
        } else {
            let intersection = self.intersection(other).unwrap();
            let mut values:Vec<usize> = vec![self.start, self.end, intersection.start, intersection.end];
            values.sort();
            let output = values
                .chunks(2)
                .map(|x| Span {start: x[0], end: x[1]})
                .filter(|i| i.len() > 0)
                .collect();
            Some(output)
        }
    }

    /// Returns a list of new `Span`s containing the bounds
    /// of this `Span` split at the boundary intersection
    /// values of another `span`
    pub fn division(&self, other: &Span) -> Option<Vec<Span>> {
        // a ...|-----------|....
        // b ......|-----|.......
        // = ...|==|=====|==|....
        if other.contains(self) {
            return Some(vec![self.clone()])
        }
        if let Some(mut output) = self.subtraction(other) {
            if let Some(intersection) = self.intersection(other) {
                output.push(intersection);
            }
            output.sort_by(|a, b| a.start.cmp(&b.start));
            Some(output)
        } else { None }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_span_new() {
        assert_eq!(Span::new(3, 5), Span {start: 3, end: 5});
        assert_eq!(Span::new(5, 3), Span {start: 3, end: 5});
    }

    #[test]
    fn test_span_properties() {
        // a ......|--------|........................
        let a: Span = Span { start:  7, end: 16 };
        assert_eq!(a.start, 7);
        assert_eq!(a.len(), 9);
        assert_eq!(a.to_range(), 7..16);
        assert!(a.contains_value(7));
        assert!(a.contains_value(10));
        assert!(!a.contains_value(16));
    }

    #[test]
    fn test_span_intersection() {
        // a ......|--------|.........
        // b ...|--|..................
        // c ......|--|...............
        // d .........|--|............
        // e ...............|--|......
        // f .................|--|....
        // g ....|------------|.......
        let a: Span = Span { start:  7, end: 16 };
        let b: Span = Span { start:  4, end:  7 };
        let c: Span = Span { start:  7, end: 10 };
        let d: Span = Span { start: 10, end: 13 };
        let e: Span = Span { start: 16, end: 19 };
        let f: Span = Span { start: 18, end: 21 };
        let g: Span = Span { start:  5, end: 18 };
        assert!(a.contains(&c) && a.contains(&d));
        assert!(!a.contains(&b) && !a.contains(&e));
        assert!(g.contains(&a));
        assert_eq!(a.intersection(&b), None);
        assert_eq!(a.intersection(&c), Some(Span {start:  7, end: 10}));
        assert_eq!(a.intersection(&d), Some(Span {start: 10, end: 13}));
        assert_eq!(a.intersection(&e), None);
        assert_eq!(a.intersection(&f), None);
        assert_eq!(a.intersection(&g), Some(Span {start:  7, end: 16}));
    }

    #[test]
    fn test_span_union() {
        // a ......|--------|.........
        // b ...|--|..................
        // c ......|--|...............
        // d .........|--|............
        // e ...............|--|......
        // f .................|--|....
        // g ....|------------|.......
        let a: Span = Span { start:  7, end: 16 };
        let b: Span = Span { start:  4, end:  7 };
        let c: Span = Span { start:  7, end: 10 };
        let d: Span = Span { start: 10, end: 13 };
        let e: Span = Span { start: 16, end: 19 };
        let f: Span = Span { start: 18, end: 21 };
        let g: Span = Span { start:  5, end: 18 };
        assert_eq!(a.union(&b), None);
        assert_eq!(a.union(&f), None);
        assert_eq!(a.union(&c), Some(Span {start: 7, end: 16}));
        assert_eq!(a.union(&d), Some(Span {start: 7, end: 16}));
        assert_eq!(a.union(&e), None);
        assert_eq!(a.union(&g), Some(Span {start: 5, end: 18}));
    }

    #[test]
    fn test_span_subtraction() {
        // a ......|--------|.........
        // b ...|--|..................
        // c ......|--|...............
        // d .........|--|............
        // e ...............|--|......
        // f .................|--|....
        // g ....|------------|.......
        // h ............|-------|....
        let a: Span = Span { start:  7, end: 16 };
        let b: Span = Span { start:  4, end:  7 };
        let c: Span = Span { start:  7, end: 10 };
        let d: Span = Span { start: 10, end: 13 };
        let e: Span = Span { start: 16, end: 19 };
        let f: Span = Span { start: 18, end: 21 };
        let g: Span = Span { start:  5, end: 18 };
        let h: Span = Span { start: 10, end: 21 };
        assert_eq!(a.subtraction(&b), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.subtraction(&f), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.subtraction(&c), Some(vec![Span { start: 10, end: 16 }]));
        assert_eq!(a.subtraction(&d), Some(vec![Span { start:  7, end: 10 }, Span { start:  13, end: 16 }]));
        assert_eq!(a.subtraction(&e), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.subtraction(&g), None);
        assert_eq!(a.subtraction(&h), Some(vec![Span { start: 7, end: 10 }]));
    }

    #[test]
    fn test_span_division() {
        // a ......|--------|.........
        // b ...|--|..................
        // c ......|--|...............
        // d .........|--|............
        // e ...............|--|......
        // f .................|--|....
        // g ....|------------|.......
        // h ............|-------|....
        let a: Span = Span { start:  7, end: 16 };
        let b: Span = Span { start:  4, end:  7 };
        let c: Span = Span { start:  7, end: 10 };
        let d: Span = Span { start: 10, end: 13 };
        let e: Span = Span { start: 16, end: 19 };
        let f: Span = Span { start: 18, end: 21 };
        let g: Span = Span { start:  5, end: 18 };
        let h: Span = Span { start: 10, end: 21 };
        assert_eq!(a.division(&b), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.division(&f), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.division(&c), Some(vec![Span { start:  7, end: 10 }, Span { start: 10, end: 16 }]));
        assert_eq!(a.division(&d), Some(vec![Span { start:  7, end: 10 }, Span { start:  10, end: 13 }, Span { start:  13, end: 16 }]));
        assert_eq!(a.division(&e), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.division(&g), Some(vec![Span { start:  7, end: 16 }]));
        assert_eq!(a.division(&h), Some(vec![Span { start:  7, end: 10 }, Span { start: 10, end: 16 }]));
    }
}

