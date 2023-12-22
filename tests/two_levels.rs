use flatfish::ff;

trait Trait1 {
    type Item1<T>: Trait2<T>;
}

trait Trait2<T> {
    type Item2;
    fn foo();
    const VALUE: u32;
}

struct Impl1;

impl Trait1 for Impl1 {
    type Item1<T> = Impl2;
}

struct Impl2;

impl<T> Trait2<T> for Impl2 {
    type Item2 = u32;
    fn foo() {}
    const VALUE: u32 = 42;
}

fn foo1<T: Trait1>(_: <<T as Trait1>::Item1<u32> as Trait2<u32>>::Item2) {
    <<T as Trait1>::Item1<u32> as Trait2<u32>>::foo();
    assert_eq!(<<T as Trait1>::Item1<u32> as Trait2<u32>>::VALUE, 42)
}

fn foo2<T: Trait1>(_: ff!(T | Trait1::Item1<u32> | Trait2<u32>::Item2)) {
    ff!(T | Trait1::Item1<u32> | Trait2<u32>::foo)();
    assert_eq!(ff!(T | Trait1::Item1<u32> | Trait2<u32>::VALUE), 42)
}

fn main() {
    foo1::<Impl1>(5);
    foo2::<Impl1>(5);
}
