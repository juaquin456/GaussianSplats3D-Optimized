use nalgebra::{Matrix3, Quaternion, UnitQuaternion, Vector3};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CameraPose {
    pub id: u32,
    pub center: Vector3<f32>,
}

pub fn read_colmap_images<P: AsRef<Path>>(path: P) -> Vec<CameraPose> {
    let path_ref = path.as_ref();

    let f = File::open(path_ref).expect("Failed to open images.bin");
    let mut reader = BufReader::new(f);
    let mut cameras = Vec::new();

    let num_reg_images = read_u64(&mut reader);

    for _i in 0..num_reg_images {
        let image_id = read_u32(&mut reader);

        let qw = read_f64(&mut reader);
        let qx = read_f64(&mut reader);
        let qy = read_f64(&mut reader);
        let qz = read_f64(&mut reader);

        let tx = read_f64(&mut reader);
        let ty = read_f64(&mut reader);
        let tz = read_f64(&mut reader);

        let _camera_id = read_u32(&mut reader);
        let name = read_string_until_null(&mut reader);

        let q = UnitQuaternion::from_quaternion(Quaternion::new(qw, qx, qy, qz));
        let r_matrix: Matrix3<f64> = q.to_rotation_matrix().into_inner();
        let t = Vector3::new(tx, ty, tz);

        let center = -(r_matrix.transpose() * t);

        cameras.push(CameraPose {
            id: image_id,
            center: Vector3::new(center.x as f32, center.y as f32, center.z as f32),
        });

        let num_points_2d = read_u64(&mut reader);

        if num_points_2d > 0 {
            match num_points_2d.checked_mul(24) {
                Some(bytes_to_skip) => {
                    if bytes_to_skip > i64::MAX as u64 {
                        eprintln!("ERROR: Offset is too large.");
                        break;
                    }
                    if let Err(e) = reader.seek_relative(bytes_to_skip as i64) {
                        eprintln!("Error seeking/skipping bytes: {:?}", e);
                        break;
                    }
                }
                None => {
                    eprintln!(
                        "Error: 'num_points_2d' appears corrupt (value: {}). Byte offset overflow.",
                        num_points_2d
                    );
                    eprintln!(
                        "Reader misalignment detected after image: '{}'",
                        name
                    );
                    break;
                }
            }
        }
    }
    cameras
}

fn read_u64<R: Read>(reader: &mut R) -> u64 {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).unwrap();
    u64::from_le_bytes(buf)
}

fn read_u32<R: Read>(reader: &mut R) -> u32 {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn read_f64<R: Read>(reader: &mut R) -> f64 {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf).unwrap();
    f64::from_le_bytes(buf)
}

fn read_string_until_null<R: Read>(reader: &mut R) -> String {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 1];
    loop {
        reader.read_exact(&mut buf).unwrap();
        if buf[0] == 0 {
            break;
        }
        bytes.push(buf[0]);
    }
    String::from_utf8(bytes).unwrap_or_default()
}