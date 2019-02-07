use glm::{ Vector3 };

use crate::graphics::mesh::{ Vertex, Triangle };
use crate::utility::Float;
use super::{ read_file, FileError };

pub fn read_obj(obj_path: &str) -> Result<Vec<Triangle>, FileError> {
    let (verts, uvs, normals, faces) = read_raw_content(obj_path)?;
    debug!("Read obj file '{}': vertices = {}, uvs = {}, normals = {}, faces = {}", obj_path, verts.len(), uvs.len(), normals.len(), faces.len());
    let mut triangles: Vec<Triangle> = Vec::new();
    for face in faces {
        let mut triangle = Triangle::default();
        for (i, indices) in face.iter().enumerate() {
            let mut vert = Vertex::default();
            let pos = verts[indices[0] - 1];
            let uv = uvs[indices[1] - 1];
            //normals get calculated
            vert.set_pos(Vector3::new(pos[0], pos[1], pos[2]));
            vert.set_uv(Vector3::new(uv[0], uv[1], 0.));
            triangle.set_vertex(vert, i);
        }
        triangle.update_normal();
        triangles.push(triangle);
    }

    Ok(triangles)
}
        
fn read_raw_content(obj_path: &str) -> Result<(Vec<[Float; 3]>,
                                               Vec<[Float; 2]>,
                                               Vec<[Float; 3]>,
                                               Vec<[[usize; 3]; 3]>),
                                               FileError> {
    let content = read_file(obj_path)?;
    let mut verts: Vec<[Float; 3]> = Vec::new();
    let mut uvs: Vec<[Float; 2]> = Vec::new();
    let mut normals: Vec<[Float; 3]> = Vec::new();
    let mut faces: Vec<[[usize; 3]; 3]> = Vec::new();
    for line in content.lines() {
        let fields: Vec<&str> = line.split(" ").collect();
        match fields[0] {
            "v" => {
                if fields.len() != 4 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let x: Float = fields[1].parse()?;
                let y: Float = fields[2].parse()?;
                let z: Float = fields[3].parse()?;
                verts.push([x, y, z]);
            },
            "vt" => {
                if fields.len() != 3 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let u: Float = fields[1].parse()?;
                let v: Float = fields[2].parse()?;
                uvs.push([u, v]);
            },
            "vn" => {
                if fields.len() != 4 {
                    return Err(FileError::UnexpectedFormat(line.to_string()));
                }
                let x: Float = fields[1].parse()?;
                let y: Float = fields[2].parse()?;
                let z: Float = fields[3].parse()?;
                normals.push([x, y, z]);
            },
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
                    face[i] = [sub_fields[0].parse()?,
                                sub_fields[1].parse()?,
                                sub_fields[2].parse()?];
                }
                faces.push(face);
            },
            _ => {}
        }
    }
    Ok((verts, uvs, normals, faces))
}