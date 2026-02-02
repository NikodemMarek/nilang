fn main(): int {
    vr is_true: bool = false;

    if (is_true) {
        vr bye: string = "Bye World!";
        print(bye);
    } ef (true) {
        vr wait: string = "I'm also here!";
        print(wait);
    } el {
        vr text: string = "Hello World!";
        print(text);
    }

    rt 0;
}
