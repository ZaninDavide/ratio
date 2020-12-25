use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

struct VertexData<'a> {
    position: (&'a f32, &'a f32, &'a f32),
    normal: (&'a f32, &'a f32, &'a f32),
    uv: (&'a f32, &'a f32),
    tangent: (&'a f32, &'a f32, &'a f32),
    bitangent: (&'a f32, &'a f32, &'a f32),
}
impl VertexData<'_> {
    pub fn push(&self, vertices: &mut Vec<f32>, use_texture_coordinates: bool, use_normals: bool, use_tangent_and_bitangent: bool) {
        // position attribute
        vertices.push(*self.position.0);
        vertices.push(*self.position.1);
        vertices.push(*self.position.2);
        // texture_coordinates attribute
        if use_texture_coordinates {
            vertices.push(*self.uv.0);
            vertices.push(*self.uv.1);
        }
        // normal attribute
        if use_normals {
            vertices.push(*self.normal.0);
            vertices.push(*self.normal.1);
            vertices.push(*self.normal.2);
        }

        if use_tangent_and_bitangent {
            vertices.push(*self.tangent.0);
            vertices.push(*self.tangent.1);
            vertices.push(*self.tangent.2);
            vertices.push(*self.bitangent.0);
            vertices.push(*self.bitangent.1);
            vertices.push(*self.bitangent.2);
        }
    }
}

pub struct obj {
    vertices: Vec<f32>,
}
impl obj {
    /// Load the data from an .obj file into and obj struct.
    /// The order of the attibutes in obj.vertices is: position, texture_coordinates (if requested), normal (if requested)
    pub fn load_new(
        path: &str,
        use_texture_coordinates: bool,
        use_normals: bool,
        use_tangent_and_bitangent: bool,
    ) -> obj {
        let mut vertices: Vec<f32> = Vec::new();

        let mut positions: Vec<f32> = Vec::new();
        let mut normals: Vec<f32> = Vec::new();
        let mut uvs: Vec<f32> = Vec::new();

        let file = File::open(path).expect("obj::load_new Failed to read file");
        let reader = std::io::BufReader::new(file);
        for line in reader
            .lines()
            .map(|line| line.expect("Failed to read line"))
        {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("o") => {
                    // println!("object");
                }
                Some("v") => {
                    let vert = (words.next(), words.next(), words.next());
                    match vert {
                        (Some(x), Some(y), Some(z)) => {
                            let (x, y, z) = (
                                f32::from_str(x).unwrap(),
                                f32::from_str(y).unwrap(),
                                f32::from_str(z).unwrap(),
                            );

                            positions.push(x); // Px
                            positions.push(y); // Py
                            positions.push(z); // Pz
                        }
                        _ => {
                            println!(
                                "obj::load_new Each vertex has to be made of three values. Found: {:?}",
                                vert
                            );
                        }
                    }
                }
                Some("vt") => {
                    if use_texture_coordinates {
                        let uv = (words.next(), words.next());
                        match uv {
                            (Some(x), Some(y)) => {
                                uvs.push(f32::from_str(x).unwrap()); // Tx
                                uvs.push(f32::from_str(y).unwrap()); // Ty
                            }
                            _ => {
                                println!(
                                "obj::load_new Texture coordinates have to be made of two values. Found: {:?}",
                                uv
                            );
                            }
                        }
                    }
                }
                Some("vn") => {
                    if use_normals {
                        let norm = (words.next(), words.next(), words.next());
                        match norm {
                            (Some(x), Some(y), Some(z)) => {
                                normals.push(f32::from_str(x).unwrap()); // Nx
                                normals.push(f32::from_str(y).unwrap()); // Ny
                                normals.push(f32::from_str(z).unwrap()); // Nz
                            }
                            _ => {
                                println!(
                                "obj::load_new Normals have to be made of three values. Found: {:?}",
                                norm
                            );
                            }
                        }
                    }
                }
                Some("f") => {
                    let face = (words.next(), words.next(), words.next(), words.next());

                    // USE THE DATA TO FILL THE VERTEX ARRAY WITH VALUES
                    let mut data_to_vertex = |data: &mut std::str::Split<&str>| -> VertexData {
                        let pi = usize::from_str(data.next().unwrap_or(""))
                            .unwrap_or_else(|_| 1 as usize)
                            - 1;
                        let ti = usize::from_str(data.next().unwrap_or(""))
                            .unwrap_or_else(|_| 1 as usize)
                            - 1;
                        let ni = usize::from_str(data.next().unwrap_or(""))
                            .unwrap_or_else(|_| 1 as usize)
                            - 1;

                        VertexData {
                            position: (
                                positions.get(3 * pi + 0).unwrap(), 
                                positions.get(3 * pi + 1).unwrap(), 
                                positions.get(3 * pi + 2).unwrap()
                            ),
                            normal: (
                                normals.get(3 * ni + 0).unwrap(),
                                normals.get(3 * ni + 1).unwrap(),
                                normals.get(3 * ni + 2).unwrap(),
                            ),
                            uv: (
                                uvs.get(2 * ti + 0).unwrap(), 
                                uvs.get(2 * ti + 1).unwrap()
                            ),
                            tangent: (&0.0, &0.0, &0.0),
                            bitangent: (&0.0, &0.0, &0.0),
                        }
                    };

                    let calculate_t_and_bt =
                        |v1: &VertexData,
                         v2: &VertexData,
                         v3: &VertexData|
                         -> ((f32, f32, f32), (f32, f32, f32)) {
                            let edge1 = (
                                v2.position.0 - v1.position.0,
                                v2.position.1 - v1.position.1,
                                v2.position.2 - v1.position.2,
                            );
                            let edge2 = (
                                v3.position.0 - v1.position.0,
                                v3.position.1 - v1.position.1,
                                v3.position.2 - v1.position.2,
                            );
                            let d_uv1 = (v2.uv.0 - v1.uv.0, v2.uv.1 - v1.uv.1);
                            let d_uv2 = (v3.uv.0 - v1.uv.0, v3.uv.1 - v1.uv.1);

                            let f = 1.0 / (d_uv1.0 * d_uv2.1 - d_uv2.0 * d_uv1.1);

                            let tangent = (
                                f * (d_uv2.1 * edge1.0 - d_uv1.1 * edge2.0),
                                f * (d_uv2.1 * edge1.1 - d_uv1.1 * edge2.1),
                                f * (d_uv2.1 * edge1.2 - d_uv1.1 * edge2.2),
                            );
                            let bitangent = (
                                f * (-d_uv2.0 * edge1.0 + d_uv1.0 * edge2.0),
                                f * (-d_uv2.0 * edge1.1 + d_uv1.0 * edge2.1),
                                f * (-d_uv2.0 * edge1.2 + d_uv1.0 * edge2.2),
                            );

                            return (tangent, bitangent);
                        };

                    match face {
                        (Some(v1), Some(v2), Some(v3), None) => {
                            let mut data1 = v1.split("/");
                            let mut data2 = v2.split("/");
                            let mut data3 = v3.split("/");

                            let mut vertex1 = data_to_vertex(&mut data1);
                            let mut vertex2 = data_to_vertex(&mut data2);
                            let mut vertex3 = data_to_vertex(&mut data3);

                            // calculate the tangent and bitangent TODO
                            let (tangent, bitangent) = calculate_t_and_bt(&vertex1, &vertex2, &vertex3);

                            // asign the calculated tangent and bitangent to each vertex
                            vertex1.tangent = (&tangent.0, &tangent.1, &tangent.2);
                            vertex2.tangent = (&tangent.0, &tangent.1, &tangent.2);
                            vertex3.tangent = (&tangent.0, &tangent.1, &tangent.2);

                            vertex1.bitangent = (&bitangent.0, &bitangent.1, &bitangent.2);
                            vertex2.bitangent = (&bitangent.0, &bitangent.1, &bitangent.2);
                            vertex3.bitangent = (&bitangent.0, &bitangent.1, &bitangent.2);

                            vertex1.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                            vertex2.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                            vertex3.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);

                        }
                        (Some(v1), Some(v2), Some(v3), Some(v4)) => {
                            // get quad data

                            let mut data1 = v1.split("/");
                            let mut data2 = v2.split("/");
                            let mut data3 = v3.split("/");
                            let mut data4 = v4.split("/");

                            let mut vertex1 = data_to_vertex(&mut data1);
                            let mut vertex2 = data_to_vertex(&mut data2);
                            let mut vertex3 = data_to_vertex(&mut data3);
                            let mut vertex4 = data_to_vertex(&mut data4);

                            // calculate the tangent and bitangent for both triangles of the quad
                            let (tangent1, bitangent1) = calculate_t_and_bt(&vertex1, &vertex2, &vertex3);
                            let (tangent2, bitangent2) = calculate_t_and_bt(&vertex1, &vertex3, &vertex4);
                            
                            // first tris

                            vertex1.tangent = (&tangent1.0, &tangent1.1, &tangent1.2);
                            vertex2.tangent = (&tangent1.0, &tangent1.1, &tangent1.2);
                            vertex3.tangent = (&tangent1.0, &tangent1.1, &tangent1.2);

                            vertex1.bitangent = (&bitangent1.0, &bitangent1.1, &bitangent1.2);
                            vertex2.bitangent = (&bitangent1.0, &bitangent1.1, &bitangent1.2);
                            vertex3.bitangent = (&bitangent1.0, &bitangent1.1, &bitangent1.2);

                            vertex1.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                            vertex2.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                            vertex3.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);

                            // second tris

                            vertex1.tangent = (&tangent2.0, &tangent2.1, &tangent2.2);
                            vertex3.tangent = (&tangent2.0, &tangent2.1, &tangent2.2);
                            vertex4.tangent = (&tangent2.0, &tangent2.1, &tangent2.2);

                            vertex1.bitangent = (&bitangent2.0, &bitangent2.1, &bitangent2.2);
                            vertex3.bitangent = (&bitangent2.0, &bitangent2.1, &bitangent2.2);
                            vertex4.bitangent = (&bitangent2.0, &bitangent2.1, &bitangent2.2);

                            vertex1.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                            vertex3.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                            vertex4.push(&mut vertices, use_texture_coordinates, use_normals, use_tangent_and_bitangent);
                        }
                        _ => {
                            println!("obj::load_new Only faces with 3 and 4 vertices are accepted")
                        }
                    }
                }
                None => {}
                _ => {}
            }
        }

        obj { vertices }
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn get_vertices_count(&self) -> usize {
        self.vertices.len()
    }
}
