st Point {
    x: int,
    y: int
}

st Rect {
    start: Point,
    end: Point
}

fn test(a: Point): int {
    rt a.x;
}

fn main(): int {
    vr p1: Point = Point { x: 5, y: 13 };
    vr r: Rect = Rect { start: p1, end: Point { x: 9, y: 7 } };
    vr t: Point = r.end;
    rt test(t);
}
