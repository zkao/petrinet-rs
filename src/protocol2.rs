/// https://disco.process.io/dummy
/// f0: () -> T0 T1.
/// f1: T1 -> T2.
/// f2: T0 -> T3 T4.
/// f3: T3 T2 -> T5.
/// f4: T5 T4 -> ().

#[derive(Default, Debug)]
struct Empty;

trait F0<T0, T1, Tuple: From<Empty> + Into<(T0, T1)>> {}
trait F1<T1, T2: From<T1>> {}
trait F2<T0: Into<(T3, T4)>, T3, T4> {}
trait F3<T3, T2, T5: From<(T3, T2)>> {}
trait F4<T4, T5, Tuple: From<(T4, T5)> + Into<Empty>> {}

fn main() {
    #[derive(Default, Debug)]
    struct A0;
    #[derive(Default, Debug)]
    struct A1;
    #[derive(Default, Debug)]
    struct A2;
    #[derive(Default, Debug)]
    struct A3;
    #[derive(Default, Debug)]
    struct A4;
    #[derive(Default, Debug)]
    struct A5;

    #[derive(Default, Debug)]
    struct PetriState;

    impl F0<A0, A1, (A0, A1)> for PetriState {}

    impl From<Empty> for (A0, A1) {
        fn from(_: Empty) -> (A0, A1) {
            dbg!(Default::default())
        }
    }

    impl F1<A1, A2> for PetriState {}

    impl From<A1> for A2 {
        fn from(_: A1) -> A2 {
            dbg!(Default::default())
        }
    }

    impl F2<A0, A3, A4> for PetriState {}

    impl From<A0> for (A3, A4) {
        fn from(_: A0) -> (A3, A4) {
            dbg!(Default::default())
        }
    }

    impl F3<A3, A2, A5> for PetriState {}

    impl From<(A3, A2)> for A5 {
        fn from(_: (A3, A2)) -> A5 {
            dbg!(Default::default())
        }
    }

    impl F4<A4, A5, (A4, A5)> for PetriState {}

    impl From<(A4, A5)> for Empty {
        fn from(_: (A4, A5)) -> Empty {
            dbg!(Default::default())
        }
    }
    let init: Empty = Default::default();
    let (a0, a1) = init.into() ;
    let a2: A2 = a1.into();
    let (a3, a4) = a0.into();
    let a5 = (a3, a2).into();
    let _: Empty = (a4, a5).into();
}
