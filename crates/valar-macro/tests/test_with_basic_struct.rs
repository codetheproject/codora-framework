use valar_macro::valar;

// #[derive(valar)]
// // #[valar(new)]
// struct Foo<'a, T, E> {
//     #[valar(refrence)]
//     _t: &'a T,

//     #[valar(refrence)]
//     _e: E,
// }

// impl<'a, T, E> Foo<'a, T, E> {
//     fn new(_t: &'a T, _e: E) -> Foo<'a, T, E> {
//         todo!()
//     }
// }

#[test]
fn test_accessor_with_basic_struct() {
    #[derive(valar, Default)]
    #[valar(access(refrence), new(visibility(private)))]
    struct User {
        #[valar(new(default))]
        name: String,

        #[allow(dead_code)]
        #[valar(access(skip), new(value = "0"))]
        age: i32,

        #[valar(access(refrence))]
        profile: Profile,
    }

    impl User {
        fn new(name: String, age: i32, profile: Profile) -> Self {
            Self { name, age, profile }
        }
    }

    #[derive(valar, Default)]
    #[valar(new())]
    struct Profile(#[valar(new(value = "We got some stuff working as we speak"))] String);

    impl Profile {
        fn new(_0: String) -> Self {
            Self(_0)
        }
    }

    // @westshgit one of the maintainers is not 30, by the way
    let user = User::new(String::from("West"), 30, Profile::new(String::from("Maintainer")));
    // assert_eq!(user.name(), &String::from("West"));
    // assert_eq!(user.profile.value(), &String::from("Maintainer"));
}
