// the idea behind these traits is that
// (1) this is a general interface for 1d, 2d, 3d vectors and
// (2) vectors can be stored quite arbitrary in the memory, opposite to plain {x, y, z} structures,
//     that can yield better performance

pub trait X { fn x(&self) -> &f64; }
pub trait Y { fn y(&self) -> &f64; }
pub trait Z { fn z(&self) -> &f64; }

pub trait Vector3: X + Y + Z {}

pub trait Field3: Vector3 {
    fn e(&self) -> &Vector3;
    fn b(&self) -> &Vector3;
}

// implementation

pub struct SimpleV3 { pub x: f64, pub y: f64, pub z: f64 }
impl X for SimpleV3 { fn x(&self) -> &f64 { &self.x } }
impl Y for SimpleV3 { fn y(&self) -> &f64 { &self.y } }
impl Z for SimpleV3 { fn z(&self) -> &f64 { &self.z } }

//pub trait EX { fn ex(&self) -> &f64; }
//pub trait EY { fn ey(&self) -> &f64; }
//pub trait EZ { fn ez(&self) -> &f64; }
//
//pub trait BX { fn bx(&self) -> &f64; }
//pub trait BY { fn by(&self) -> &f64; }
//pub trait BZ { fn bz(&self) -> &f64; }

/*pub struct Vector3 { x: f64, y: f64, z: f64 }
impl X for Vector3 { fn x(&self) -> &f64 { &self.x } }
impl Y for Vector3 { fn y(&self) -> &f64 { &self.y } }
impl Z for Vector3 { fn z(&self) -> &f64 { &self.z } }

pub struct Field { e: Vector3, b: Vector3}
trait Field: Vector3 { fn e(&self) -> Vector3; fn b(&self) -> Vector3 }

trait Field {
    type Vector;
    fn e(&self) -> &Self::Vector;
    fn b(&self) -> &Self::Vector;
}*/
