#![allow(clippy::similar_names, clippy::assertions_on_result_states)]
use quote::{format_ident, quote};

use super::*;

#[test]
fn attrs_fields_success() {
    let input = r#"
#[derive(Inject)]
#[provides(transient = "dyn FooTrait")]
pub struct Foo {
    #[inject(default)]
    bar: u8,
    #[inject(ctor = "-1")]
    baz: i64,
    #[inject(transient)]
    my_transient: Box<dyn BarTrait>,
    #[inject(singleton)]
    my_singleton: Arc<dyn BazTrait>,
    #[inject(transient = true)]
    my_transient_long: Box<dyn BarTrait>,
    #[inject(singleton = true)]
    my_singleton_long: Arc<dyn BazTrait>,
}"#;
    let parsed = syn::parse_str(input).unwrap();
    let receiver = DeriveAttrInput::from_derive_input(&parsed).unwrap();

    let fields = receiver.data.take_struct().unwrap();
    let get_field = |name: &str| -> &DeriveField {
        fields
            .fields
            .iter()
            .find(|el| el.ident().as_ref().unwrap() == &format_ident!("{name}"))
            .unwrap()
    };
    let bar = get_field("bar");
    let baz = get_field("baz");
    let my_transient = get_field("my_transient");
    let my_singleton = get_field("my_singleton");
    let my_transient_long = get_field("my_transient_long");
    let my_singleton_long = get_field("my_singleton_long");

    assert!(bar.default);
    assert_eq!(bar.ctor, None);

    assert!(!baz.default);
    assert_eq!(baz.ctor, Some("-1".to_owned()));

    assert!(!my_transient.default);
    assert!(my_transient.ctor().is_none());
    assert!(my_transient.transient);
    assert!(!my_transient.singleton);

    assert!(!my_transient_long.default);
    assert!(my_transient_long.ctor().is_none());
    assert!(my_transient_long.transient);
    assert!(!my_transient_long.singleton);

    assert!(!my_singleton.default);
    assert!(my_singleton.ctor().is_none());
    assert!(!my_singleton.transient);
    assert!(my_singleton.singleton);

    assert!(!my_singleton_long.default);
    assert!(my_singleton_long.ctor().is_none());
    assert!(!my_singleton_long.transient);
    assert!(my_singleton_long.singleton);
}

#[test]
fn attrs_transient_success() {
    let input = r#"
#[derive(Inject)]
#[provides(transient = "dyn FooTrait")]
pub struct Foo {
    #[inject(default)]
    counter: u8,
}"#;

    let parsed = syn::parse_str(input).unwrap();
    let receiver = DeriveAttrInput::from_derive_input(&parsed).unwrap();

    assert_eq!(receiver.ident, format_ident!("Foo"));
    // let tokens = quote! { dyn FooTrait };
    // assert_eq!(receiver.transient, Some(syn::Type::from(tokens)));
    assert_eq!(receiver.singleton, None);
}

#[test]
fn attrs_singleton_success() {
    let input = r#"
#[derive(Inject)]
#[provides(singleton = "Foo")]
pub struct Foo {
    #[inject(default)]
    counter: u8,
}"#;

    let parsed = syn::parse_str(input).unwrap();
    let receiver = DeriveAttrInput::from_derive_input(&parsed).unwrap();

    assert_eq!(receiver.ident, format_ident!("Foo"));
    assert_eq!(receiver.transient, None);
    assert_eq!(receiver.singleton, Some("Foo".to_owned()));
}

#[test]
fn attrs_singleton_failure() {
    let input = "
#[derive(Inject)]
#[provides(singleton)]
pub struct Foo {
    #[inject(default)]
    counter: u8,
}";

    let parsed = syn::parse_str(input).unwrap();
    let receiver = DeriveAttrInput::from_derive_input(&parsed);

    assert!(receiver.is_err());
}

#[test]
fn attrs_transient_failure() {
    let input = "
#[derive(Inject)]
#[provides(transient)]
pub struct Foo {
    #[inject(default)]
    counter: u8,
}";

    let parsed = syn::parse_str(input).unwrap();
    let receiver = DeriveAttrInput::from_derive_input(&parsed);

    assert!(receiver.is_err());
}
