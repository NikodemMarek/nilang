st Point {
    x: int,
    y: int
}

st Rect {
    start: Point,
    end: Point
}

fn test(a: int, b: Point): int {
    rt a * b.x + 5;
}

fn main(): int {
    vr p: Point = Point { x: 6, y: 5 };
    rt test(5, p);
}
