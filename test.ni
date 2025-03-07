st Point {
    x: int,
    y: int
}

st Rect {
    start: Point,
    end: Point
}

fn main(): int {
    vr p: Point = Point { x: 5, y: 13 };
    rt p.y;
}
