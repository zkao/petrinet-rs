/// https://disco.process.io/dummy
/// f0: () -> T0 T1.
/// f1: T1 -> T2.
/// f2: T0 -> T3 T4.
/// f3: T3 T2 -> T5.
/// f4: T5 T4 -> ().

#[derive(Default, Debug)]
struct Empty;
trait F0<T0, T1, Tuple: From<Empty> + Into<(T0, T1)>> {
    type In;
    type Out;
}
trait F1<T1, T2: From<T1>> {
    type In;
    type Out;
}
trait F2<T0: Into<(T3, T4)>, T3, T4> {
    type In;
    type Out;
}
trait F3<T3, T2, T5: From<(T3, T2)>> {
    type In;
    type Out;
}
trait F4<T4, T5, Tuple: From<(T4, T5)> + Into<Empty>> {
    type In;
    type Out;
}

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

    impl F0<A0, A1, (A0, A1)> for PetriState {
        type In = Empty;
        type Out = (A0, A1);
    }
    impl From<Empty> for (A0, A1) {
        fn from(_: Empty) -> Self {
            dbg!(Default::default())
        }
    }

    impl F1<A1, A2> for PetriState {
        type In = A1;
        type Out = A2;
    }

    impl From<A1> for A2 {
        fn from(_: A1) -> A2 {
            dbg!(Default::default())
        }
    }

    impl F2<A0, A3, A4> for PetriState {
        type In = (A3, A4);
        type Out = A0;
    }

    impl From<A0> for (A3, A4) {
        fn from(_: A0) -> (A3, A4) {
            dbg!(Default::default())
        }
    }

    impl F3<A3, A2, A5> for PetriState {
        type In = (A3, A2);
        type Out = A5;
    }

    impl From<(A3, A2)> for A5 {
        fn from(_: (A3, A2)) -> A5 {
            dbg!(Default::default())
        }
    }

    impl F4<A4, A5, (A4, A5)> for PetriState {
        type In = (A4, A5);
        type Out = Empty;
    }

    impl From<(A4, A5)> for Empty {
        fn from(_: (A4, A5)) -> Empty {
            dbg!(Default::default())
        }
    }

    #[derive(Default, Debug)]
    struct State<T0, T1, T2, T3, T4, T5>(
        Option<T0>,
        Option<T1>,
        Option<T2>,
        Option<T3>,
        Option<T4>,
        Option<T5>,
    )
    where
        T0: Into<(T3, T4)>,
        T1: Into<T2>,
        T2: From<T1>,
        T5: From<(T3, T2)>,
        (T0, T1): From<Empty> + Into<(T0, T1)>,
        (T4, T5): From<(T4, T5)> + Into<Empty>;

    fn run() -> Option<()> {
        let init: Empty = Default::default();
        let d: State<A0, A1, A2, A3, A4, A5> = Default::default();
        dbg!(&d);

        // let (a0, a1): (A0, A1) =
        // // let (a0, a1): (<A0>, <A1>) = (ia0.into(), ia1.into());
        // d.0 = a0.into();
        // d.1 = a1.into();
        // dbg!(&d);

        // d.2 = d.1.map(|x| x.into());
        // dbg!(&d);
        // (d.3, d.4) = d.0.map(|a0| {
        //     let (a3, a4) = a0.into();
        //     (Some(a3), Some(a4))
        // })?;

        // dbg!(&d);
        // let a5 = (a3, a2).into();
        // let _: Empty = (a4, a5).into();
        None
    }
    run();
}
