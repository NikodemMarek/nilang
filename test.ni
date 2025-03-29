st Point {
    x: int,
    y: int
}

st Rect {
    start: Point,
    end: Point
}

fn test(a: int): int {
    vr p: Point = Point { x: 44, y: a };
    rt p.x;
}

fn main(): int {
    vr test: int = test(5);
    printi(test + 8);

    rt 0;
}
