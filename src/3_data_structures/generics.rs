struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

pub(crate) fn run() {
    let a = Point { x: 0.0, y: 4.0 };
    let b = Point { x: 1.2, y: 3.4 };

    let _line = Line { start: a, end: b };
}
