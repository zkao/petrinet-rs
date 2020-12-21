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
    fn enabled(input: Option<&Self::In>, output: Option<&Self::Out>) -> bool {
        input.is_some() && output.is_none()
    }
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
        (): MInto<(A0, A1)>,
        (T3, T4): MFrom<A0>,
        A1: MInto<T2>,
        A0: MInto<(T3, T4)>,
        T5: MFrom<(T3, T2)>,
        (): MFrom<(T4, T5)>,
    {
        type In = ();
        type Out = (A0, A1);
    }

    impl<T0: Default, T1: Default> MFrom<()> for (T0, T1)
    {
        fn mfrom(_: ()) -> Self {
            Default::default()
        }
    }

    impl<T0: Default, T3, T4, T5> F1<A1, A2> for State<T0, A1, A2, T3, T4, T5>
    where
        (): MInto<(T0, A1)>,
        (T3, T4): MFrom<T0>,
        A1: MInto<A2>,
        T0: MInto<(T3, T4)>,
        T5: MFrom<(T3, A2)>,
        (): MFrom<(T4, T5)>,
    {
        type In = A1;
        type Out = A2;
    }

    impl MFrom<A1> for A2 {
        fn mfrom(_: A1) -> A2 {
            dbg!(Default::default())
        }
    }

    impl<T1: Default, T2, T5> F2<A0, A3, A4> for State<A0, T1, T2, A3, A4, T5>
    where
        (): MInto<(A0, T1)>,
        (A3, A4): MFrom<A0>,
        T1: MInto<T2>,
        A0: MInto<(A3, A4)>,
        T5: MFrom<(A3, T2)>,
        (): MFrom<(A4, T5)>,
    {
        type In = (A3, A4);
        type Out = A0;
    }

    impl MFrom<A0> for (A3, A4) {
        fn mfrom(_: A0) -> (A3, A4) {
            dbg!(Default::default())
        }
    }

    impl<T0: Default, T1: Default, T4> F3<A3, A2, A5> for State<T0, T1, A2, A3, T4, A5>
    where
        (): MInto<(T0, T1)>,
        (A3, T4): MFrom<T0>,
        T1: MInto<A2>,
        T0: MInto<(A3, T4)>,
        A5: MFrom<(A3, A2)>,
        (): MFrom<(T4, A5)>,
    {
        type In = (A3, A2);
        type Out = A5;
    }

    impl<T3, T2> MFrom<(T3, T2)> for A5 {
        fn mfrom(_: (T3, T2)) -> A5 {
            dbg!(Default::default())
        }
    }

    impl<T0: Default, T1: Default, T2, T3> F4<(), A4, A5> for State<T0, T1, T2, T3, A4, A5>
    where
        (): MInto<(T0, T1)>,
        (T3, A4): MFrom<T0>,
        T1: MInto<T2>,
        T0: MInto<(A3, A4)>,
        (A3, A2): MInto<A5>,
        (): MFrom<(A4, A5)>,
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
    struct State<T0, T1, T2, T3, T4, T5>
    where
        (T0, T1): MFrom<()>,
        (T3, T4): MFrom<T0>,
        T1: MInto<T2>,
        T0: MInto<(T3, T4)>,
        (T3, T2): MInto<T5>,
        (): MFrom<(T4, T5)>,
    {
        t0: Option<T0>,
        t1: Option<T1>,
        t2: Option<T2>,
        t3: Option<T3>,
        t4: Option<T4>,
        t5: Option<T5>,
    }

    impl<T0, T1, T2, T3, T4, T5> State<T0, T1, T2, T3, T4, T5>
    where
        (T0, T1): MFrom<()>,
        (T3, T4): MFrom<T0>,
        T1: MInto<T2>,
        T0: MInto<(T3, T4)>,
        (T3, T2): MInto<T5>,
        (): MFrom<(T4, T5)>,
    {
        fn enabled(&self) -> Vec<String> {
            let mut fns: Vec<String> = Vec::new();

            let input: Option<&()> = Some(&()); // always available
            let output: Option<&(T0, T1)> = self.t0.zip(self.t1).as_ref();
            if F0::enabled(input, output) {
                fns.push("f0: () -> T0 T1".to_string());
            }
            if F1::enabled(self.t1.as_ref(), self.t2.as_ref()) {
                fns.push("f1: T1 -> T2".to_string());
            }

            let input: Option<&T0> = self.t0.as_ref();
            let output: Option<&(T3, T4)> = self.t3.zip(self.t4).as_ref();
            if F2::enabled(input, output) {
                fns.push("f2: T0 -> T3 T4".to_string());
            }

            let input: Option<&(T3, T2)> = self.t3.zip(self.t2).as_ref();
            let output: Option<&T5> = self.t5.as_ref();
            if F3::enabled(input, output) {
                fns.push("f3: T3 T2 -> T5".to_string());
            }
            let input: Option<&(T5, T4)> = self.t5.zip(self.t4).as_ref();
            let output: Option<&()> = None.as_ref();
            if F4::enabled(input, output) {
                fns.push("f4: T5 T4 -> () ".to_string());
            }
            dbg!(fns)
        }
        // // at the place that receives all the messages
        // fn update(mut self, s: String) -> Self {
        //     match s {
        //         s if s.eq(&"EventM0Happened".to_string()) => {
        //             self.0 = Some(A0);
        //             self
        //         }
        //         s if s.eq(&"EventM1Happened".to_string()) => {
        //             self.1 = Some(A1);
        //             self
        //         }
        //         s if s.eq(&"EventM2Happened".to_string()) => {
        //             self.2 = Some(A2);
        //             self
        //         }
        //         s if s.eq(&"EventM3Happened".to_string()) => {
        //             self.3 = Some(A3);
        //             self
        //         }
        //         s if s.eq(&"EventM4Happened".to_string()) => {
        //             self.4 = Some(A4);
        //             self
        //         }
        //         s if s.eq(&"EventM5Happened".to_string()) => {
        //             self.5 = Some(A5);
        //             self
        //         }
        //         s if s.eq(&"EventM6Happened".to_string()) => {
        //             self.6 = Some(A6);
        //             self
        //         }
        //         s => {
        //             println!("Unsuppported event {}", s);
        //             self
        //         }
        //     }
        // }
    }
    fn run() -> Option<()> {
        // let d = State::init();
        // // let d = d.update("EventM0Happened".to_string());
        // // let mut d = d.update("EventM1Happened".to_string());
        // d.enabled();
        // let (a3, a4) = d.0?.fire();
        // let a2: A2 = d.1?.fire();

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
