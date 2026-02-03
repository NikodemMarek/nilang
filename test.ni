fn main(): int {
    vr is_true: bool = true;
    vr text: string = "Hello World!";

    wl (is_true) {
        print(text);

        if (is_true) {
            is_true = false;
        }
    }

    rt 0;
}
