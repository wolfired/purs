#![crate_name = "purs"]
#![crate_type = "bin"]

use purs::instant_buffer::{ICodec, InstantBuffer};

#[derive(Debug, Default)]
struct Id {
    id: u8,
}

#[derive(Debug, Default)]
struct User {
    name: String,
    age: u8,
    is_admin: bool,
    email: Mail,
}

#[derive(Debug, Default)]
struct Mail {
    email: String,
}

impl<const N: usize> ICodec<N> for Id {
    fn encode(&self, rb: &InstantBuffer<N>) {
        self.id.encode(rb);
    }

    fn decode(&mut self, rb: &InstantBuffer<N>) {
        self.id.decode(rb);
    }
}

impl<const N: usize> ICodec<N> for User {
    fn encode(&self, rb: &InstantBuffer<N>) {
        self.name.encode(rb);
        self.age.encode(rb);
        self.is_admin.encode(rb);
        self.email.encode(rb);
    }

    fn decode(&mut self, rb: &InstantBuffer<N>) {
        self.name.decode(rb);
        self.age.decode(rb);
        self.is_admin.decode(rb);
        self.email.decode(rb);
    }
}

impl<const N: usize> ICodec<N> for Mail {
    fn encode(&self, rb: &InstantBuffer<N>) {
        self.email.encode(rb);
    }

    fn decode(&mut self, rb: &InstantBuffer<N>) {
        self.email.decode(rb);
    }
}

fn main() {
    let u0 = User {
        name: "link".to_owned(),
        age: 35,
        is_admin: true,
        email: Mail {
            email: "wjianl@qq.com".to_owned(),
        },
    };

    let m0 = Mail {
        email: "wolfired@qq.com".to_owned(),
    };

    let rb: InstantBuffer<{ 1024 * 4 }> = InstantBuffer::new();

    for i in 0..100 {
        let id = Id { id: i % 2 + 1 };
        match id.id {
            1 => {
                rb.write_object(&id);
                rb.write_object(&u0);
                println!("write obj {:?}", u0);
            }
            2 => {
                rb.write_object(&id);
                rb.write_object(&m0);
                println!("write obj {:?}", m0);
            }
            _ => {}
        }

        if let Some(id) = rb.read_object::<Id>() {
            match id.id {
                1 => {
                    let obj = rb.read_object::<User>().unwrap();
                    println!("read obj {:?}", obj);
                }
                2 => {
                    let obj = rb.read_object::<Mail>().unwrap();
                    println!("read obj {:?}", obj);
                }
                ud @ _ => {
                    println!("undefine id {}", ud);
                }
            }
        }
    }

    println!("{}", rb);
}
