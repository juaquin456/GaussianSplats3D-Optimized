use std::convert::From;
use nalgebra::Vector3;
use ply_rs::ply::{DefaultElement, Property};



#[derive(Copy, Clone, Debug)]
pub struct Gaussian {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub f_dc: [f32; 3],
    pub f_rest: [f32; 45],
    pub opacity: f32,
    pub scale: [f32; 3],
    pub rot: [f32; 4],
    pub importance_score: f32,
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

fn get_f32_optional(elem: &DefaultElement, key: &str, default: f32) -> f32 {
    match elem.get(key) {
        Some(x) => match x {
            Property::Float(x) => *x,
            Property::Double(x) => *x as f32,
            Property::Int(x) => *x as f32,
            _ => default
        },
        None => default,
    }
}

impl Gaussian {
    pub fn position_vec(&self) -> Vector3<f32> {
        Vector3::new(self.position[0], self.position[1], self.position[2])
    }
    pub fn distance_to(&self, point: &Vector3<f32>) -> f32 {
        (self.position_vec() - point).norm()
    }

    pub fn distance_to_array(&self, point: &[f32; 3]) -> f32 {
        let dx = self.position[0] - point[0];
        let dy = self.position[1] - point[1];
        let dz = self.position[2] - point[2];
        (dx*dx + dy*dy + dz*dz).sqrt()
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
            get_f32(elem, "nx"),
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

        let importance_score = get_f32_optional(elem, "importance_score", 0.0);

        Gaussian {
            position,
            normal,
            f_dc,
            f_rest,
            opacity,
            scale,
            rot,
            importance_score,
        }
    }
}

impl Into<DefaultElement> for Gaussian {
    fn into(self) -> DefaultElement {
        let mut elem = DefaultElement::new();
        elem.insert("x".to_string(), Property::Float(self.position[0]));
        elem.insert("y".to_string(), Property::Float(self.position[1]));
        elem.insert("z".to_string(), Property::Float(self.position[2]));

        elem.insert("nx".to_string(), Property::Float(self.normal[0]));
        elem.insert("ny".to_string(), Property::Float(self.normal[1]));
        elem.insert("nz".to_string(), Property::Float(self.normal[2]));

        for i in 0..3 {
            elem.insert(format!("f_dc_{}", i), Property::Float(self.f_dc[i]));
        }

        for i in 0..45 {
            elem.insert(format!("f_rest_{}", i), Property::Float(self.f_rest[i]));
        }

        elem.insert("opacity".to_string(), Property::Float(self.opacity));

        elem.insert("scale_0".to_string(), Property::Float(self.scale[0]));
        elem.insert("scale_1".to_string(), Property::Float(self.scale[1]));
        elem.insert("scale_2".to_string(), Property::Float(self.scale[2]));

        for i in 0..4 {
            elem.insert(format!("rot_{}", i), Property::Float(self.rot[i]));
        }

        elem.insert("importance_score".to_string(), Property::Float(self.importance_score));

        elem
    }
}