use crate::point::Point;

pub fn shift_tail_if_needed(head: &Point, tail: &mut Point) {
    if head.x == tail.x && (head.y - tail.y).abs() > 1 {
        shift_tail_vertically(head, tail);
    } else if head.y == tail.y && (head.x - tail.x).abs() > 1 {
        shift_tail_horizontally(head, tail);
    } else if (head.x - tail.x).abs() == 1 && (head.y - tail.y).abs() > 1 {
        tail.x = head.x;
        shift_tail_vertically(head, tail);
    } else if (head.y - tail.y).abs() == 1 && (head.x - tail.x).abs() > 1 {
        tail.y = head.y;
        shift_tail_horizontally(head, tail);
    } else if (head.y - tail.y).abs() == 2 && (head.x - tail.x).abs() == 2 {
        shift_tail_vertically(head, tail);
        shift_tail_horizontally(head, tail);
    }
}

fn shift_tail_horizontally(head: &Point, tail: &mut Point) {
    if head.x > tail.x {
        tail.x = tail.x + (head.x - tail.x - 1);
    } else {
        tail.x = tail.x + (head.x - tail.x + 1);
    }
}

fn shift_tail_vertically(head: &Point, tail: &mut Point) {
    if head.y > tail.y {
        tail.y = tail.y + (head.y - tail.y - 1);
    } else {
        tail.y = tail.y + (head.y - tail.y + 1);
    }
}
