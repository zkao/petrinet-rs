/// https://disco.process.io/dummy
/// f0: () -> T0 T1.
/// f1: T1 -> T2.
/// f2: T0 -> T3 T4.
/// f3: T3 T2 -> T5.
/// f4: T5 T4 -> ().
trait MFrom<T>: Sized {
    fn mfrom(_: T) -> Self;
}
trait MInto<T>: Sized {
    fn into(self) -> T;
}
impl<T, U> MInto<U> for T
where
    U: MFrom<T>,
{
    fn into(self) -> U {
        U::mfrom(self)
    }
}

impl<T> MFrom<T> for T {
    fn mfrom(t: T) -> T {
        t
    }
}

/// `I` and `O` are the input and output types of the composed petrinet that may
/// be used to glue two petrinets together. They are types external to the
/// petrinet specification. They may be a () type to keep an isolated petrinet.
/// One-safe nets are enforced on the following definitions. Removing the
/// outuput.is_none() clause may be used to allow natural number petrinets.
trait F0<I, T0, T1> {
    type In: MFrom<I>;
    type Out: MInto<(T0, T1)>;
    fn enabled(input: Option<&Self::In>, output: Option<&Self::Out>) -> bool {
        input.is_some() && output.is_none()
    }
}
trait F1<T1, T2> {
    type In: MFrom<T1>;
    type Out: MInto<T2>;
    fn enabled(input: Option<&Self::In>, output: Option<&Self::Out>) -> bool {
        input.is_some() && output.is_none()
    }
}
trait F2<T0, T3, T4> {
    type In: MFrom<(T3, T4)>;
    type Out: MInto<T0>;
    fn enabled(input: Option<&Self::In>, output: Option<&Self::Out>) -> bool {
        input.is_some() && output.is_none()
    }
}
trait F3<T3, T2, T5> {
    type In: MFrom<(T3, T2)>;
    type Out: MInto<T5>;
    fn enabled(input: Option<&Self::In>, output: Option<&Self::Out>) -> bool {
        input.is_some() && output.is_none()
    }
}
trait F4<O, T4, T5> {
    type In: MFrom<(T4, T5)>;
    type Out: MInto<O>;
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

    /// Partial specification of the types (type currying?). The generic type
    /// parameters do not matter for this implementation
    impl<T2, T3, T4, T5> F0<(), A0, A1> for State<A0, A1, T2, T3, T4, T5>
    where
        T2: MFrom<A1>,
        T5: MFrom<(T3, T2)>,
        (T3, T4): MFrom<A0>,
    {
        type In = ();
        type Out = (A0, A1);
    }

    impl MFrom<()> for (A0, A1) {
        fn mfrom(_: ()) -> Self {
            dbg!(Default::default())
        }
    }

    impl<T0, T3, T4, T5> F1<A1, A2> for State<T0, A1, A2, T3, T4, T5>
    where
        T5: MFrom<(T3, A2)>,
        (T3, T4): MFrom<T0>,
    {
        type In = A1;
        type Out = A2;
    }

    impl MFrom<A1> for A2 {
        fn mfrom(_: A1) -> A2 {
            dbg!(Default::default())
        }
    }

    impl<T1, T2: MFrom<T1>, T5: MFrom<(A3, T2)>> F2<A0, A3, A4> for State<A0, T1, T2, A3, A4, T5> {
        type In = (A3, A4);
        type Out = A0;
    }

    impl MFrom<A0> for (A3, A4) {
        fn mfrom(_: A0) -> (A3, A4) {
            dbg!(Default::default())
        }
    }

    impl<T0, T1, T4> F3<A3, A2, A5> for State<T0, T1, A2, A3, T4, A5>
    where
        T1: MInto<A2>,
        T0: MInto<(A3, T4)>,
    {
        type In = (A3, A2);
        type Out = A5;
    }

    impl MFrom<(A3, A2)> for A5 {
        fn mfrom(_: (A3, A2)) -> A5 {
            dbg!(Default::default())
        }
    }

    impl<T0, T2, T1, T3> F4<(), A4, A5> for State<T0, T1, T2, T3, A4, A5>
    where
        T2: MFrom<T1>,
        T0: MInto<(T3, A4)>,
        (T3, T2): MInto<A5>,
    {
        type In = (A4, A5);
        type Out = ();
    }

    impl MFrom<(A4, A5)> for () {
        fn mfrom(_: (A4, A5)) -> () {
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
        T0: MInto<(T3, T4)>,
        T1: MInto<T2>,
        (T3, T2): MInto<T5>,
        (T0, T1): MInto<(T0, T1)>,
        (T4, T5): MFrom<(T4, T5)>;

    fn run() -> Option<()> {
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
        // let _: () = (a4, a5).into();
        None
    }
    run();
}
