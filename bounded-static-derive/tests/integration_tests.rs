use bounded_static::{IntoBoundedStatic, ToBoundedStatic, ToStatic};
use std::borrow::Cow;

#[test]
fn test_struct_named_fields_1() {
    #[derive(ToStatic)]
    struct Foo<'a> {
        value: Cow<'a, str>,
    }
    let value = String::from("value");
    let data = Foo {
        value: Cow::from(&value),
    };
    let owned = data.to_static();
    ensure_static(owned);
}

#[test]
fn test_struct_named_fields_2() {
    #[derive(ToStatic)]
    struct Foo<'a, 'b> {
        u8_value: u8,
        static_str: &'static str,
        owned_str: String,
        value: Cow<'a, str>,
        bar: Vec<Bar<'b>>,
    }
    #[derive(ToStatic)]
    struct Bar<'a> {
        u8_value: u8,
        static_str: &'static str,
        owned_str: String,
        value: Cow<'a, str>,
    }
    let value = String::from("value");
    let bar = Bar {
        u8_value: 0,
        static_str: "",
        owned_str: String::from(""),
        value: Cow::from(&value),
    };
    let data = Foo {
        u8_value: 0,
        static_str: "",
        owned_str: String::from(""),
        value: Cow::from(&value),
        bar: vec![bar],
    };
    let owned = data.to_static();
    ensure_static(owned);
}

#[test]
fn test_no_generics_or_lifetimes() {
    #[derive(ToStatic)]
    struct Foo(u32);
    let data = Foo(0);
    ensure_static(data.to_static())
}

#[test]
fn test_struct_named_fields_no_generics() {
    #[derive(ToStatic)]
    struct Foo {
        foo: String,
        bar: &'static str,
    }
    let data = Foo {
        foo: String::from("value"),
        bar: "test",
    };
    let owned = data.to_static();
    ensure_static(owned);
}

#[test]
fn test_struct_unnamed_fields() {
    #[derive(ToStatic)]
    struct Foo<'a>(String, Cow<'a, str>, u16, Bar<'a>);
    #[derive(ToStatic)]
    struct Bar<'a> {
        bar: Cow<'a, str>,
    }
    let value = String::from("value");
    let data = Foo(
        String::from("test"),
        Cow::from(&value),
        99,
        Bar {
            bar: Cow::from(&value),
        },
    );
    ensure_static(data.to_static());
}

#[test]
fn test_struct_unnamed_fields_no_generics() {
    #[derive(ToStatic)]
    struct Foo(String, &'static str);
    let data = Foo(String::from("value"), "test");
    let owned = data.to_static();
    ensure_static(owned);
}

#[test]
fn test_unit_struct() {
    #[derive(ToStatic)]
    struct Foo;
    let data = Foo;
    ensure_static(data.to_static());
}

#[test]
fn test_struct_complex_lifetimes() {
    #[derive(ToStatic)]
    struct Foo<'a, 'b, R, T: 'b>
    where
        'b: 'a,
        R: 'a,
        T: 'a,
    {
        baz: T,
        a: Cow<'a, str>,
        b: Cow<'b, str>,
        r: R,
    }

    let value = String::from("value");
    let data = Foo {
        baz: 0isize,
        a: Cow::from(&value),
        b: Cow::from(&value),
        r: "test",
    };
    let owned = data.to_static();
    ensure_static(owned);
}

#[test]
fn test_struct_named_fields_into() {
    #[derive(ToStatic)]
    struct Foo<'a> {
        value: Cow<'a, str>,
    }
    let value = String::from("value");
    let data = Foo {
        value: Cow::from(&value),
    };
    let owned = data.into_static();
    ensure_static(owned);
}

#[test]
fn test_struct_unnamed_fields_into() {
    #[derive(ToStatic)]
    struct Foo<'a>(String, Cow<'a, str>, u16, Bar<'a>);
    #[derive(ToStatic)]
    struct Bar<'a> {
        bar: Cow<'a, str>,
    }
    let value = String::from("value");
    let data = Foo(
        String::from("test"),
        Cow::from(&value),
        99,
        Bar {
            bar: Cow::from(&value),
        },
    );
    ensure_static(data.into_static());
}

#[test]
fn test_unit_struct_into() {
    #[derive(ToStatic)]
    struct Foo;
    let data = Foo;
    ensure_static(data.into_static());
}

#[test]
fn test_enum() {
    #[derive(ToStatic)]
    enum Foo<'a> {
        Unit,
        Named { name: String, age: i8 },
        First(Cow<'a, str>, Cow<'a, str>),
        Second(Bar<'a>),
        Third(i128, bool, &'static str),
    }
    #[derive(ToStatic)]
    struct Bar<'a> {
        bar: Cow<'a, str>,
    }
    let value = String::from("value");
    let unit = Foo::Unit;
    ensure_static(unit.to_static());
    let named = Foo::Named {
        name: String::from("test"),
        age: 10,
    };
    ensure_static(named.to_static());
    let first = Foo::First(Cow::from(&value), Cow::from(&value));
    ensure_static(first.to_static());
    let second = Foo::Second(Bar {
        bar: Cow::from(&value),
    });
    ensure_static(second.to_static());
    let third = Foo::Third(100, true, "test");
    ensure_static(third.to_static());
}

#[test]
fn test_enum_into() {
    #[derive(ToStatic)]
    enum Foo<'a> {
        Unit,
        Named { name: String, age: i8 },
        First(Cow<'a, str>, Cow<'a, str>),
        Second(Bar<'a>),
        Third(i128, bool, &'static str),
    }
    #[derive(ToStatic)]
    struct Bar<'a> {
        bar: Cow<'a, str>,
    }
    let value = String::from("value");
    let unit = Foo::Unit;
    ensure_static(unit.into_static());
    let named = Foo::Named {
        name: String::from("test"),
        age: 10,
    };
    ensure_static(named.into_static());
    let first = Foo::First(Cow::from(&value), Cow::from(&value));
    ensure_static(first.into_static());
    let second = Foo::Second(Bar {
        bar: Cow::from(&value),
    });
    ensure_static(second.into_static());
    let third = Foo::Third(100, true, "test");
    ensure_static(third.into_static());
}

#[test]
fn test_thread_spawn() {
    #[derive(Debug, PartialEq, ToStatic)]
    struct Foo<'a> {
        foo: Cow<'a, str>,
        bar: Vec<Bar<'a>>,
    }
    #[derive(Debug, PartialEq, ToStatic)]
    enum Bar<'a> {
        First,
        Second(Cow<'a, str>),
    }
    let value = String::from("data");
    let data = Foo {
        foo: Cow::from(&value),
        bar: vec![Bar::First, Bar::Second(Cow::from(&value))],
    };
    let data_static = data.into_static();
    std::thread::spawn(move || {
        assert_eq!(data_static.foo, "data");
        assert_eq!(
            data_static.bar,
            vec![Bar::First, Bar::Second("data".into())]
        )
    })
    .join()
    .unwrap();
}

#[test]
fn test_const_generics_struct() {
    #[derive(ToStatic)]
    struct Foo<'a, const N: usize, const M: usize> {
        value: Cow<'a, str>,
        left: [usize; N],
        right: [usize; M],
    }
    let value = String::from("value");
    let data = Foo {
        value: Cow::from(&value),
        left: [0],
        right: [0, 1, 2],
    };
    let owned = data.to_static();
    ensure_static(owned);
}

#[test]
fn test_const_generics_struct_into() {
    #[derive(ToStatic)]
    struct Foo<'a, const N: usize, const M: usize, const Q: bool> {
        value: Cow<'a, str>,
        left: [usize; N],
        right: [usize; M],
    }
    let value = String::from("value");
    let data = Foo::<'_, 1, 3, true> {
        value: Cow::from(&value),
        left: [0],
        right: [0, 1, 2],
    };
    let owned = data.into_static();
    ensure_static(owned);
}

#[test]
fn test_generic_bound_1() {
    #[derive(ToStatic)]
    struct Baz<'a, T: Into<String> + 'a> {
        t: T,
        r: Cow<'a, str>,
    }
    let value = String::from("test");
    let data = Baz {
        t: "",
        r: Cow::from(&value),
    };
    ensure_static(data.to_static());
}

#[test]
fn test_generic_bound_2() {
    trait Foo {}
    trait Bar {}

    impl Foo for String {}
    impl Bar for String {}
    impl<T: ToOwned + ?Sized> Foo for Cow<'_, T> {}
    impl<T: ToOwned + ?Sized> Bar for Cow<'_, T> {}

    #[derive(ToStatic)]
    struct Baz<T: Foo + Bar, R: Foo> {
        t: T,
        r: R,
    }
    let value = String::from("test");
    let data = Baz {
        t: Cow::from(&value),
        r: String::from("test"),
    };
    ensure_static(data.to_static());
}

#[test]
fn test_generic_bound_3() {
    #[derive(ToStatic)]
    struct Baz<'a, T: Into<String>>(T, Cow<'a, str>);
    let value = String::from("test");
    let data = Baz("", Cow::from(&value));
    ensure_static(data.to_static());
}

#[test]
fn test_generic_bound_where_1() {
    #[derive(ToStatic)]
    struct Baz<'a, T: Foo>(T, Cow<'a, str>)
    where
        T: Into<String>;
    trait Foo {}
    impl Foo for &str {}
    let value = String::from("test");
    let data = Baz("", Cow::from(&value));
    ensure_static(data.to_static());
}

#[test]
fn test_generic_bound_where_2() {
    #[derive(ToStatic)]
    struct Baz<'a, T: Foo>(T, Cow<'a, str>)
    where
        T: Into<String> + 'a + Bar;
    trait Foo {}
    impl Foo for &str {}
    trait Bar {}
    impl Bar for &str {}
    let value = String::from("test");
    let data = Baz("", Cow::from(&value));
    ensure_static(data.into_static());
}

fn ensure_static<S: 'static>(s: S) {
    drop(s);
}
