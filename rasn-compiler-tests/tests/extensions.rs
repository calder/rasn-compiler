#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(
    extension,
    r#"
    Foo ::= SEQUENCE {
       foo [1] BOOLEAN OPTIONAL,
       bar [2] BOOLEAN OPTIONAL,
       ...,
       [[
          fizz [3] BOOLEAN OPTIONAL,
          buzz [4] BOOLEAN OPTIONAL,
       ]]
    }
    "#,
    r#"
    #[doc="Innertype"]
    #[derive(AsnType,Debug,Clone,Decode,Encode,PartialEq,Eq,Hash)]
    pub struct Foo ExtGroupFizz{
        #[rasn(tag(context, 3))]
        pub fizz:Option<bool>,

        #[rasn(tag(context, 4))]
        pub buzz:Option<bool>,
    }

    impl FooExtGroupFizz{
        pub fn new(fizz: Option<bool>, buzz: Option<bool>) -> Self {
            Self { fizz, buzz }
        }
    }

    #[derive(AsnType,Debug,Clone,Decode,Encode,PartialEq,Eq,Hash)]
    #[non_exhaustive]
    pub struct Foo{
        #[rasn(tag(context,1))]
        pub foo:Option<bool>,

        #[rasn(tag(context,2))]
        pub bar:Option<bool>,

        #[rasn(extension_addition_group,identifier="SEQUENCE")]
        pub ext_group_fizz:Option<FooExtGroupFizz>,
    }

    impl Foo {
        pub fn new(foo: Option<bool>, bar: Option<bool>, ext_group_fizz: Option<FooExtGroupFizz>,) -> Self {
            Self { foo, bar, ext_group_fizz, }
        }
    }
    "#
);
