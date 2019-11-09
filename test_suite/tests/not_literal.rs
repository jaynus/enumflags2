const FOO_BAR: u8 = 1;
const FOO_BAZ: u8 = 2;

#[derive(Clone, Copy, enumflags2::BitFlags)]
#[repr(u8)]
enum Foo {
    Bar = FOO_BAR,
    Baz = FOO_BAZ,
}
