pub struct List<T> {
    head: Link<T>,
}

// yay type aliases!
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
    pub fn reverse(&mut self) {
        // 定义之前的节点: 取第一个节点
        let mut link = None;
        // 定义并初始化当前节点
        let mut cur_link = self.head.take();

        while let Link::Some(mut boxed_node) = cur_link {
            // 取出它的下一个节点,以供后续使用
            cur_link = boxed_node.next;
            // 让当前节点改变指向link,link本身的值不可用,但是link作为一个变量名还可以换绑其他变量?
            boxed_node.next = link;
            link = Some(boxed_node)
        }
        self.head = link
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Link::Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here
        }
    }
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn to_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// 这里无需生命周期，因为 List 没有使用生命周期的关联项
impl<T> List<T> {
    // 这里我们为 `iter` 声明一个生命周期 'a , 此时 `&self` 需要至少和 `Iter` 活得一样久
    pub fn iter(&self) -> Iter<'_, T> {
        // Some(<Box<Node<T>>) => Some(&<Box<Node<T>>)
        // &<Box<Node<T>> ** => Node<T> & => &Node<T>
        // Some(<Box<Node<T>>) as_deref => Some(&Node<T>)
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// 这里声明生命周期是因为下面的关联类型 Item 需要
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        // TODO
        let mut list = List::new();
        // Check empty list behaves right
        assert_eq!(list.pop(), None);
        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.to_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let s: &&Option<Box<str>> = &&Some(Box::from("hello"));

        // &&Option => 会自动解引用匹配到 => Option<T> 然后执行 Option<&T>
        // 无论多少层引用调用as_ref 自动解引用直到匹配Option<T> => Option<&T>
        // 对Some(&T) => Some(&Target)
        let borrowed2 = s.as_ref();
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn reverse() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        list.reverse();

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
    }

    #[derive(Debug)]
    struct Node {
        val: i32,
        next: Option<Box<Node>>,
    }
    #[test]
    fn test() {
        let mut head = Node {
            val: 1,
            next: Some(Box::new(Node { val: 2, next: None })),
        };

        let mut cur = &mut head;

        // ❌ 以下代码将触发编译器借用冲突错误
        let next_ref = cur.next.as_mut(); // 可变借用了 cur.next
        cur = next_ref.unwrap(); // 又想修改 cur 本身
        print!("{cur:?}");
    }

    #[test]
    fn test2() {
        let mut node = Node { val: 1, next: None };

        let next_ref = node.next.as_mut(); // 可变借用 node.next
        let x = next_ref.is_none();
        node.next = None; // ❌ 同时修改 node → 编译器报错
        print!("{x}");
    }
    #[test]
    fn test3() {
        let mut head = Node {
            val: 1,
            next: Some(Box::new(Node { val: 1, next: None })),
        };
        let mut cur = &mut head;

        // 从 cur（&mut Node）借用字段 cur.next（&mut Option<Box<Node>>）
        // 并赋值给 cur 变量
        cur = cur.next.as_mut().unwrap();
    }

    #[test]
    fn test4() {
        let mut head = Node {
            val: 1,
            next: Some(Box::new(Node { val: 1, next: None })),
        };
        let mut cur = &mut head;

        // 从 cur（&mut Node）借用字段 cur.next（&mut Option<Box<Node>>）
        // 并赋值给 cur 变量
        cur = cur.next.as_mut().unwrap();
    }
}
