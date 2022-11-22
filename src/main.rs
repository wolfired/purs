#![crate_name = "purs"]
#![crate_type = "bin"]

#![allow(dead_code)]

use std::error::Error;

use purs::cg::line::line_bresenham;
use purs::obj::Obj;
use purs::tga::image::TGAImage;
use purs::fixed_point::fp::test;

fn main() -> Result<(), Box<dyn Error>> {
    test()?;

    // let a = Into::<FixedPoint<i64, 10>>::into(25.0);
    // let b = Into::<FixedPoint<i64, 10>>::into(15.0);

    // println!("{:?}", Into::<f64>::into((a).sqrt()));

    // let v: Vector<FixedPoint<i32, 8>, 3> = Vector::new_with([1.0.into(); 3]);
    // let f: FixedPoint<i32, 8> = 1.7320508075688772935274463415059.into();
    // println!("{:?}", v.magnitude());
    // println!("{:?}", f);

    // let fp = FixedPoint::<i64, 8>::from(-9);

    // println!("{:?}", fp);

    Ok(())
}

fn step0() -> Result<(), Box<dyn Error>> {
    let obj = Obj::load("E:\\Desktop\\african_head.obj")?;

    let mut img = TGAImage::new(1024, 1024);

    let w = img.get_width() - 1;
    let h = img.get_height() - 1;

    for f in obj.fs {
        let p0: (f32, f32, f32) = obj.vs[f.0 .0 - 1];
        let p1: (f32, f32, f32) = obj.vs[f.1 .0 - 1];

        line_bresenham(
            ((p0.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p0.1 + 1f32) * h as f32 / 2f32) as i64,
            ((p1.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p1.1 + 1f32) * h as f32 / 2f32) as i64,
            |x, y| img.set_color(x as u16, y as u16, 255, 255, 255, 255),
        );

        let p0: (f32, f32, f32) = obj.vs[f.1 .0 - 1];
        let p1: (f32, f32, f32) = obj.vs[f.2 .0 - 1];

        line_bresenham(
            ((p0.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p0.1 + 1f32) * h as f32 / 2f32) as i64,
            ((p1.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p1.1 + 1f32) * h as f32 / 2f32) as i64,
            |x, y| img.set_color(x as u16, y as u16, 255, 255, 255, 255),
        );

        let p0: (f32, f32, f32) = obj.vs[f.2 .0 - 1];
        let p1: (f32, f32, f32) = obj.vs[f.0 .0 - 1];

        line_bresenham(
            ((p0.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p0.1 + 1f32) * h as f32 / 2f32) as i64,
            ((p1.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p1.1 + 1f32) * h as f32 / 2f32) as i64,
            |x, y| img.set_color(x as u16, y as u16, 255, 255, 255, 255),
        );
    }

    img.save("E:\\Desktop\\4444.tga")?;

    Ok(())
}
