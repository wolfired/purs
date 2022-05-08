#![crate_name = "prust"]
#![crate_type = "lib"]
#![feature(stmt_expr_attributes)]

pub mod com {
    pub mod wolfired {
        pub fn max(x: i8, y: i8) -> i8 {
            #[rustfmt::skip]
            if x > y { x } else { y }
        }

        pub struct Sun {
            name: String,
            age: i8,
        }

        impl Sun {
            pub fn new(name: &str, age: i8) -> Self {
                Self {
                    name: name.to_string(),
                    age,
                }
            }

            pub fn hi(&self) {
                println!("你好, 我的名字叫 {}, 今年 {} 岁啦!", self.name, self.age);
            }

            pub fn hello(&self, (x, y): (i8, i8)) {
                println!("{} + {} = {}", x, y, x + y);
            }
        }
    }
}
