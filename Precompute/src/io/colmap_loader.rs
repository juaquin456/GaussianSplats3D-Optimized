use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct CameraPose {
    pub id: u32,
    pub center: [f32; 3],
}

pub fn read_colmap_images<P: AsRef<Path>>(path: P) -> Vec<CameraPose> {
    let path_ref = path.as_ref();

    let f = File::open(path_ref).expect("No se pudo abrir images.bin");
    let mut reader = BufReader::new(f);
    let mut cameras = Vec::new();

    let num_reg_images = read_u64(&mut reader);

    for i in 0..num_reg_images {
        let image_id = read_u32(&mut reader);

        let qw = read_f64(&mut reader);
        let qx = read_f64(&mut reader);
        let qy = read_f64(&mut reader);
        let qz = read_f64(&mut reader);

        let tx = read_f64(&mut reader);
        let ty = read_f64(&mut reader);
        let tz = read_f64(&mut reader);

        let camera_id = read_u32(&mut reader);

        let name = read_string_until_null(&mut reader);

        let r_matrix = quaternion_to_matrix(qw, qx, qy, qz);
        let r_transposed = [
            [r_matrix[0][0], r_matrix[1][0], r_matrix[2][0]],
            [r_matrix[0][1], r_matrix[1][1], r_matrix[2][1]],
            [r_matrix[0][2], r_matrix[1][2], r_matrix[2][2]],
        ];

        let cx = -(r_transposed[0][0] * tx + r_transposed[0][1] * ty + r_transposed[0][2] * tz);
        let cy = -(r_transposed[1][0] * tx + r_transposed[1][1] * ty + r_transposed[1][2] * tz);
        let cz = -(r_transposed[2][0] * tx + r_transposed[2][1] * ty + r_transposed[2][2] * tz);

        cameras.push(CameraPose {
            id: image_id,
            center: [cx as f32, cy as f32, cz as f32],
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
                        eprintln!("ERROR al saltar bytes: {:?}", e);
                        break;
                    }
                },
                None => {
                    eprintln!("ERROR: 'num_points_2d' is currupted (valor: {}). Overflow al calcular bytes.", num_points_2d);
                    eprintln!("Esto significa que el lector se desalineó en la imagen anterior: '{}'", name);
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

fn quaternion_to_matrix(w: f64, x: f64, y: f64, z: f64) -> [[f64; 3]; 3] {
    [
        [1.0 - 2.0*y*y - 2.0*z*z, 2.0*x*y - 2.0*z*w,       2.0*x*z + 2.0*y*w],
        [2.0*x*y + 2.0*z*w,       1.0 - 2.0*x*x - 2.0*z*z, 2.0*y*z - 2.0*x*w],
        [2.0*x*z - 2.0*y*w,       2.0*y*z + 2.0*x*w,       1.0 - 2.0*x*x - 2.0*y*y],
    ]
}