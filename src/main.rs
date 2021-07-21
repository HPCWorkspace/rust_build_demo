mod toolsbox;

#[allow(dead_code)]
#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
        let mut rev = vec![];
        for x in xs.iter() {
            rev.insert(0, x.clone())
        }
        rev
    }

    quickcheck! {
        fn prop(xs: Vec<u32>) -> bool {
            xs == reverse(&reverse(&xs))
        }
    }
}

// #[cfg(test)]
// mod tests {
//     //proptest
//     //使用proptest工具测试，这里会发现错误
//     use crate::parse_date;
//     use proptest::prelude::*;
//     proptest! {
//         #[test]
//         fn doesnt_crash(s in "\\PC*") {
//             parse_date(&s);
//         }
//     }
// }

#[allow(dead_code)]
fn parse_date(s: &str) -> Option<(u32, u32, u32)> {
    if 10 != s.len() {
        return None;
    }
    if "-" != &s[4..5] || "-" != &s[7..8] {
        return None;
    }

    let year = &s[0..4];
    let month = &s[6..7];
    let day = &s[8..10];

    year.parse::<u32>().ok().and_then(|y| {
        month
            .parse::<u32>()
            .ok()
            .and_then(|m| day.parse::<u32>().ok().map(|d| (y, m, d)))
    })
}

fn sanitizer() {
    let x = vec![1, 2, 3, 4];
    let _y = unsafe { *x.as_ptr().offset(6) };
    println!("{}", _y)
}

/// Rust builds the toolchain example
/// err link for deadlinks:
/// [`std::future::Futuresss`]
/// right link for deadlinks:
/// [`std::future::Future`]
/// https://github.com/surechensssss/rust_build_demo
#[allow(rustdoc::bare_urls)]
#[allow(rustdoc::broken_intra_doc_links)]
fn main() {
    println!("Hello, world!");
    let mut n: i32 = 64;
    let p1 = &n as *const i32;
    let p2 = &mut n as *mut i32;
    unsafe {
        println!("r1 is: {}", *p1);
        println!("r2 is: {}", *p2);
    }

    use dangerous::*;
    let input = dangerous::input(b"hello");
    let result: Result<_, Invalid> = input.read_partial(|r| r.read_u8());
    assert_eq!(result, Ok((b'h', dangerous::input(b"ello"))));

    // let s = "Hello world";  /* .unwrap() */
    // let _ = s.find("wo").unwrap();
    // // let _ = s.find("wo").unwrap();
    // let ignore = "s.unwrap();";

    // 使用编译器内置的快速内存错误检测功能
    sanitizer();
}
