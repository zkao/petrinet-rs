/// https://disco.process.io/dummy
/// f0: () -> T0 T1.
/// f1: T1 -> T2.
/// f2: T0 -> T3 T4.
/// f3: T3 T2 -> T5.
/// f4: T5 T4 -> ().

trait F0<T0, T1> {
    fn f0() -> (T0, T1);
}
trait F1<T1, T2> {
    fn f1 (x: T1) -> T2;
}
trait F2<T0, T3, T4>{
    fn f2(x: T0) -> (T3, T4);
}
trait F3<T3, T2, T5> {
    fn f3(x: (T3, T2)) -> T5;
}
trait F4<T5, T4> {
    fn f4(x: (T5, T4))  -> ();
}

#[derive(Default)]
struct T0;
#[derive(Default)]
struct T1;
#[derive(Default)]
struct T2;
#[derive(Default)]
struct T3;
#[derive(Default)]
struct T4;
#[derive(Default)]
struct T5;

#[derive(Default)]
struct PetriState;

impl F0<T0, T1> for PetriState {
    fn f0() -> (T0, T1) {
        Default::default()
    }
}

impl F1<T1, T2> for PetriState {
    fn f1 (x: T1) -> T2;
}
