/*
 * Given the head of a linked list, rotate the list to the right by k places.
 */
use log::debug;

pub enum RotatingDirection {
    Right,
}

impl RotatingDirection {
    fn from_str(s: &str) -> Self {
        match s {
            "right" => RotatingDirection::Right,
            _ => panic!("Invalid mode: {}", s),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }

    fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut current = head;
        let mut vec = Vec::new();
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next;
        }
        vec
    }

    fn split_at(&mut self, n: usize) -> Option<Self> {
        // Memory utility function
        use std::mem;

        let mut split = Some(self);
        // traverse to the n - 1 node and get the next node (which is the nth node)
        for _ in 0..n - 1 {
            // get the next node if available. if not return none
            if let Some(node) = split.map(|v| v.next.as_deref_mut()) {
                split = node;
            } else {
                return None;
            }
        }

        match split {
            Some(node) => {
                let mut new_head = None;
                // swap None with next node
                mem::swap(&mut new_head, &mut node.next);
                new_head.map(|v| *v)
            }
            None => None,
        }
    }

    fn extend_from_list(&mut self, new_tail: Box<ListNode>) {
        let mut tail = self;
        while tail.next.is_some() {
            tail = tail.next.as_deref_mut().unwrap();
        }

        tail.next = Some(new_tail)
    }
}

fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut pointer = head.clone();
    let mut count: usize = 0;
    while let Some(mut node) = pointer {
        count = count + 1;
        if node.next == None {
            node.next = head;
            pointer = node.next.take();
            break;
        }
        pointer = node.next.take();
    }
    let rotating_index = count - (k as usize % count);
    debug!("rotating_index: {}", rotating_index);
    if rotating_index == count {
        // this mean that no rotation is needed
        return pointer.clone();
    }
    let mut new_tail = pointer.clone();
    let new_head = new_tail.as_mut().unwrap().split_at(rotating_index);
    match new_head {
        Some(mut head_pointer) => {
            head_pointer.extend_from_list(new_tail.clone().unwrap());
            Some(Box::new(head_pointer))
        }
        None => None,
    }
}

fn rotating_list_right(array: Vec<i32>, k: i32) -> Result<Vec<i32>, String> {
    let head = ListNode::from_vec(array.clone());

    let rotated_head = rotate_right(head, k);
    let rotated_array = ListNode::to_vec(rotated_head);

    Ok(rotated_array)
}

pub fn rotating_list(
    array: Vec<i32>,
    k: i32,
    rotating_direction: &String,
) -> Result<Vec<i32>, String> {
    match RotatingDirection::from_str(rotating_direction) {
        RotatingDirection::Right => rotating_list_right(array, k),
    }
}
