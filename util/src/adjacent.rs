pub trait Adjacent {
    fn adjacent_to(&self, r: usize, c: usize) -> Vec<(usize, usize)>;
}

impl<T> Adjacent for Vec<Vec<T>> {
    fn adjacent_to(&self, r: usize, c: usize) -> Vec<(usize, usize)> {
        let mut adj = Vec::new();
        if !self.is_empty() && !self[0].is_empty() {
            for rdelta in [-1, 0, 1] {
                if (rdelta == -1 && r <= 0) || (rdelta == 1 && r >= self.len() - 1) {
                    continue;
                }
                for cdelta in [-1, 0, 1] {
                    if (cdelta == -1 && c <= 0)
                        || (cdelta == 1 && c >= self[0].len() - 1)
                        || (rdelta == 0 && cdelta == 0)
                    {
                        continue;
                    }
                    adj.push(((r as i32 + rdelta) as usize, (c as i32 + cdelta) as usize));
                }
            }
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn empty() {
        let v: Vec<Vec<u32>> = Vec::new();
        assert!(v.adjacent_to(0, 0).is_empty());
    }

    #[test]
    fn nonempty() {
        let v = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(
            v.adjacent_to(0, 0).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(0, 1), (1, 0), (1, 1),])
        );
        assert_eq!(
            v.adjacent_to(0, 1).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(0, 0), (0, 2), (1, 0), (1, 1), (1, 2),])
        );
        assert_eq!(
            v.adjacent_to(0, 2).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(0, 1), (1, 1), (1, 2),])
        );
        assert_eq!(
            v.adjacent_to(1, 0).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(0, 0), (0, 1), (1, 1), (2, 0), (2, 1),])
        );
        assert_eq!(
            v.adjacent_to(1, 1).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ])
        );
        assert_eq!(
            v.adjacent_to(1, 2).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)])
        );
        assert_eq!(
            v.adjacent_to(2, 0).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(1, 0), (1, 1), (2, 1)])
        );
        assert_eq!(
            v.adjacent_to(2, 1).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)])
        );
        assert_eq!(
            v.adjacent_to(2, 2).iter().cloned().collect::<HashSet<_>>(),
            HashSet::from([(1, 1), (1, 2), (2, 1)])
        );
    }
}
