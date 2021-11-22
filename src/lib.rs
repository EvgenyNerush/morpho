// the idea behind these traits is that
// (1) this is a general interface for 1d, 2d, 3d vectors and
// (2) vectors can be stored quite arbitrary in the memory, opposite to plain {x, y, z} structures,
//     that potentially can lead to better performance

pub trait X { fn x(&self) -> &f64; }
pub trait Y { fn y(&self) -> &f64; }
pub trait Z { fn z(&self) -> &f64; }

pub trait Vector3: X + Y + Z {}

/*pub trait Field3: Vector3 {
    fn e(&self) -> &Vector3;
    fn b(&self) -> &Vector3;
}*/

//
// Implementation. To avoid confusion between the trait and structures which implement it,
// the structures always have "_" (underscore) in their names.
//

pub struct Vector_3 { pub x: f64, pub y: f64, pub z: f64 }
impl Vector3 for Vector_3 {}
impl X for Vector_3 { fn x(&self) -> &f64 { &self.x } }
impl Y for Vector_3 { fn y(&self) -> &f64 { &self.y } }
impl Z for Vector_3 { fn z(&self) -> &f64 { &self.z } }

/*struct SimpleF3 { pub e: Vector_3, pub b: Vector_3 }
impl Field3 for Vector_3 {
    fn e(&self) -> &Vector3 { &self.e }
    fn b(&self) -> &Vector3 { &self.b }
}*/

// ok, that if we need different dimensions, e.g. particle with (x,y,x) and (py, pz)?
