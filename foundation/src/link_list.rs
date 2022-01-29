#[derive(Debug)]
struct ListNode {
    data: i64,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn insert(mut self, new_data: i64) {
        match self.next {
            None => self.next = Some(ListNode {data: new_data, next: None}),
            Some(_) => self.next.insert(new_data),
        }
    }
}

fn create_link_list (data: i64) -> ListNode {
    return ListNode{
        data,
        next: None
    }
}

pub fn run() {
    let mut ls = create_link_list(1);
    println!("{:?}", ls);
    ls.insert(2);
    println!("{:?}", ls);
}
