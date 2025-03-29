st Point {
    x: int,
    y: int
}

st Rect {
    start: Point,
    end: Point
}

fn test(a: int): int {
    rt a + 1;
}

fn main(): int {
    vr test: int = test(5);
    printi(test + 8);

    rt 0;
}
