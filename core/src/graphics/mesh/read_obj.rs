
use super::{Triangle, Vertex};
use crate::file::{read_file, FileError};
use crate::{Point2f, Point3f};

pub fn read_obj(obj_path: &str) -> Result<Vec<Triangle>, FileError> {
    let (verts, uvs, normals, faces) = read_raw_content(obj_path)?;
    debug!(
        "Read obj file '{}': vertices = {}, uvs = {}, normals = {}, faces = {}",
        obj_path,
        verts.len(),
        uvs.len(),
        normals.len(),
        faces.len()
    );
    let mut triangles: Vec<Triangle> = Vec::new();
    for face in faces {
        let mut triangle = Triangle::default();
        for (i, indices) in face.iter().enumerate() {
            let mut vert = Vertex::default();
            let pos = verts[indices[0] - 1];
            let uv = uvs[indices[1] - 1];
            //normals get calculated
            vert.set_pos(Point3f::new(pos[0], pos[1], pos[2]));
            vert.set_uv(Point2f::new(uv[0], uv[1]));
            triangle.set_vertex(vert, i);
        }
        triangle.update_normals();
        triangles.push(triangle);
    }

    Ok(triangles)
}

fn read_raw_content(
    obj_path: &str,
) -> Result<
    (
        Vec<[f32; 3]>,
        Vec<[f32; 2]>,
        Vec<[f32; 3]>,
        Vec<[[usize; 3]; 3]>,
    ),
    FileError,
> {
    let content = read_file(obj_path)?;
    let mut verts: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut faces: Vec<[[usize; 3]; 3]> = Vec::new();
    for line in content.lines() {
        let fields: Vec<&str> = line.split(" ").collect();
        match fields[0] {
            "v" => {
                if fields.len() != 4 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let x: f32 = fields[1].parse()?;
                let y: f32 = fields[2].parse()?;
                let z: f32 = fields[3].parse()?;
                verts.push([x, y, z]);
            }
            "vt" => {
                if fields.len() != 3 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let u: f32 = fields[1].parse()?;
                let v: f32 = fields[2].parse()?;
                uvs.push([u, v]);
            }
            "vn" => {
                if fields.len() != 4 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let x: f32 = fields[1].parse()?;
                let y: f32 = fields[2].parse()?;
                let z: f32 = fields[3].parse()?;
                normals.push([x, y, z]);
            }
            "f" => {
                if fields.len() != 4 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let mut face: [[usize; 3]; 3] = [[0; 3]; 3];
                for (i, field) in fields[1..].iter().enumerate() {
                    let sub_fields: Vec<&str> = field.split("/").collect();
                    if sub_fields.len() != 3 {
                        return Err(FileError::UnexpectedFormat(line.to_string()));
                    }
                    face[i] = [
                        sub_fields[0].parse()?,
                        sub_fields[1].parse()?,
                        sub_fields[2].parse()?,
                    ];
                }
                faces.push(face);
            }
            _ => {}
        }
    }
    Ok((verts, uvs, normals, faces))
}
