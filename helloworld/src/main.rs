use std::rc::Rc;

struct User {
    username: String,
    email: String,
}

fn add(a:u32,b:u32) -> u32{
    return a*b;
}


fn change_user(mut user:User) -> User{
    user.username=String::from("fff");
    return user;
}



fn get_user() -> User{
    let user1 = User {
        email: String::from("123"),
        username: String::from("holly"),
    };
    return user1;
//    let userRef:&User=&user1;
//    return userRef.to_owned();
}


fn main() {

    //let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//    for i in "中国人".chars(){
//        println!("{}",i)
//    }


    let init="中国人";
    let a=init.chars().collect::<Vec<_>>();
    let b=&a[0..2];
    println!("{}",b.into_iter().collect::<String>());


//    let g=&String::from("中国人");
//    let h=&g[0..g.len()];


    //let b = a.collect();

//    let slice = &"abc"[0..2];
//    println!("{}", slice);


//    let sum=add(1,2);
//    println!("Hello, world! {} ",sum);
//
//    let mut  user1 = User {
//        email: String::from("123"),
//        username: String::from("456"),
//    };
//
//
//    user1=change_user(user1);
//    println!("say  {}",user1.username);
//    println!("Hello, world! {} ",sum);
//
//
//
//    let s = "123456";
//    let ss: String = s.to_owned();
//    println!("Hello, world! {} ",ss);
//
//
//    let user= get_user();
//    println!("get user {}" ,user.username)
//    let cl1=|x,y| x==y;
//    println!("{}",cl1(1,1));

}




//trait StringUtils {
//    fn substring(&self, start: usize, len: usize) -> Self;
//}
//
//impl StringUtils for String {
//    fn substring(&self, start: usize, len: usize) -> Self {
//        self.chars().skip(start).take(len).collect()
//    }
//}
//
//// Usage:
//fn main() {
//    let phrase: String = "this is a string".to_string();
//    println!("{}", phrase.substring(5, 8)); // prints "is a str"
//}


//
//impl StringUtils for str {
//    fn substring(&self, start: usize, len: usize) -> &str {
//        let mut char_pos = 0;
//        let mut byte_start = 0;
//        let mut it = self.chars();
//        loop {
//            if char_pos == start { break; }
//            if let Some(c) = it.next() {
//                char_pos += 1;
//                byte_start += c.len_utf8();
//            }
//            else { break; }
//        }
//        char_pos = 0;
//        let mut byte_end = byte_start;
//        loop {
//            if char_pos == len { break; }
//            if let Some(c) = it.next() {
//                char_pos += 1;
//                byte_end += c.len_utf8();
//            }
//            else { break; }
//        }
//        &self[byte_start..byte_end]
//    }
//    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
//        let start = match range.start_bound() {
//            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
//            Bound::Unbounded => 0,
//        };
//        let len = match range.end_bound() {
//            Bound::Included(bound) => *bound + 1,
//            Bound::Excluded(bound) => *bound,
//            Bound::Unbounded => self.len(),
//        } - start;
//        self.substring(start, len)
//    }
//}
//
//fn main() {
//    let s = "abcdèfghij";
//    // All three statements should print:
//    // "abcdè, abcdèfghij, dèfgh, dèfghij."
//    println!("{}, {}, {}, {}.",
//             s.substring(0, 5),
//             s.substring(0, 50),
//             s.substring(3, 5),
//             s.substring(3, 50));
//    println!("{}, {}, {}, {}.",
//             s.slice(..5),
//             s.slice(..50),
//             s.slice(3..8),
//             s.slice(3..));
//    println!("{}, {}, {}, {}.",
//             s.slice(..=4),
//             s.slice(..=49),
//             s.slice(3..=7),
//             s.slice(3..));
//}
//
