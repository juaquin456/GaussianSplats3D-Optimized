use ply_rs::ply::{DefaultElement, Property};
use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub struct Gaussian {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub f_dc: [f32; 3], // DC coefficient
    pub f_rest: [f32; 45], // SH coefficient
    pub opacity: f32,
    pub scale: [f32; 3],
    pub rot: [f32; 4],
}
fn get_f32(elem: &DefaultElement, key: &str) -> f32 {
    match elem.get(key) {
        Some(x) => {
            match x {
                Property::Float(x) => *x,
                Property::Double(x) => *x as f32,
                Property::Int(x) => *x as f32,
                _ => {panic!("Unexpected property type")}
            }
        }
        None => panic!("Unexpected property type for {}", key),
    }
}

impl From<&DefaultElement> for Gaussian {
    fn from(elem: &DefaultElement) -> Self {
        let position = [
            get_f32(elem, "x"),
            get_f32(elem, "y"),
            get_f32(elem, "z"),
        ];

        let normal = [
            get_f32(elem, "nxx"),
            get_f32(elem, "ny"),
            get_f32(elem, "nz"),
        ];

        let f_dc = [
            get_f32(elem, "f_dc_0"),
            get_f32(elem, "f_dc_1"),
            get_f32(elem, "f_dc_2"),
        ];

        let mut f_rest = [0.0f32; 45];
        for (i, val) in f_rest.iter_mut().enumerate() {
            *val = get_f32(elem, format!("f_rest_{}", i).to_string().as_str());
        }

        let opacity = get_f32(elem, "opacity");

        let scale = [
            get_f32(elem, "scale_0"),
            get_f32(elem, "scale_1"),
            get_f32(elem, "scale_2"),
        ];

        let rot = [
            get_f32(elem, "rot_0"),
            get_f32(elem, "rot_1"),
            get_f32(elem, "rot_2"),
            get_f32(elem, "rot_3"),
        ];

        Gaussian {
            position,
            normal,
            f_dc,
            f_rest,
            opacity,
            scale,
            rot,
        }
    }
}

impl Into<DefaultElement> for Gaussian {
    fn into(self) -> DefaultElement {
        let mut elem = DefaultElement::new();
        elem["x"] = Property::Float(self.position[0]);
        elem["y"] = Property::Float(self.position[1]);
        elem["z"] = Property::Float(self.normal[0]);
        
        elem["nxx"] = Property::Float(self.normal[1]);
        elem["nyx"] = Property::Float(self.normal[2]);
        elem["nz"] = Property::Float(self.normal[3]);
        
        for i in 0..3 {
            elem[format!("f_rest_{}", i).as_str()] = Property::Float(self.f_dc[i]);
        }
        
        for i in 0..45 {
            elem[format!("f_rest_{}", i).as_str()] = Property::Float(self.f_dc[i]);
        }
        
        elem["opacity"] = Property::Float(self.opacity);

        elem["scale_0"] = Property::Float(self.scale[0]);
        elem["scale_1"] = Property::Float(self.scale[1]);
        elem["scale_2"] = Property::Float(self.scale[2]);
        
        for i in 0..4 {
            elem[format!("rot_{}", i).as_str()] = Property::Float(self.rot[i]);
        }
         
        elem
    }
}