#[derive(Debug, Default)]
pub struct List<T>(Option<(T, Box<List<T>>)>);

impl<T> List<T> {
    pub fn new() -> Self {
        List(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(List(t))))
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.0.take() {
            None => None,
            Some((val, next)) => {
                self.0 = next.0;
                Some(val)
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut next)) => next.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        let mut ans = 0;
        let mut p = self;
        while let Some((_, next)) = &p.0 {
            ans += 1;
            unsafe {
                p = std::mem::transmute_copy(next);
            }
        }
        ans
        // match &self.0 {
        //     Some((_, next)) => 1 + next.len(),
        //     None => 0
        // }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List;

    #[test]
    pub fn list_test_01() {
        let mut list = List::default();
        assert_eq!(list.len(), 0);
        list.push_front(3);
        list.push_back(4);
        list.push_front(9);
        assert_eq!(list.len(), 3);
    }

    #[test]
    pub fn list_test_02() {
        let mut list: List<i32> = List::default();
        let p = list.pop_front();
        assert_eq!(p.is_none(), true);
        assert_eq!(list.len(), 0);
        list.push_front(3);
        list.push_back(4);
        list.push_front(9);
        println!("{:?}", list);
        let p = list.pop_front();
        println!("{:?}", p);
        assert_eq!(p, Some(9));
        println!("{:?}", list);
        let p = list.pop_front();
        println!("{:?}", p);
        println!("{:?}", list);
        assert_eq!(p, Some(3));
        let p = list.pop_front();
        println!("{:?}", p);
        println!("{:?}", list);
        assert_eq!(p, Some(4));
        let p = list.pop_front();
        println!("{:?}", p);
        assert_eq!(p, None);
    }
}
