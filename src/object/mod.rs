use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

pub struct obj {
    vertices: Vec<f32>,
}
impl obj {
    /// Load the data from an .obj file into and obj struct.
    /// The order of the attibutes in obj.vertices is: position, texture_coordinates (if requested), normal (if requested)
    pub fn load_new(path: &str, use_texture_coordinates: bool, use_normals: bool) -> obj {
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

                    let mut data_to_vertex = |data: &mut std::str::Split<&str>| {
                        let pi = usize::from_str(data.next().unwrap_or(""))
                            .unwrap_or_else(|_| 1 as usize)
                            - 1;
                        let ti = usize::from_str(data.next().unwrap_or(""))
                            .unwrap_or_else(|_| 1 as usize)
                            - 1;
                        let ni = usize::from_str(data.next().unwrap_or(""))
                            .unwrap_or_else(|_| 1 as usize)
                            - 1;

                        // position attribute
                        vertices.push(*positions.get(3 * pi + 0).unwrap());
                        vertices.push(*positions.get(3 * pi + 1).unwrap());
                        vertices.push(*positions.get(3 * pi + 2).unwrap());
                        // texture_coordinates attribute
                        if use_texture_coordinates {
                            vertices.push(*uvs.get(2 * ti + 0).unwrap());
                            vertices.push(*uvs.get(2 * ti + 1).unwrap());
                        }
                        // normal attribute
                        if use_normals {
                            vertices.push(*normals.get(3 * ni + 0).unwrap());
                            vertices.push(*normals.get(3 * ni + 1).unwrap());
                            vertices.push(*normals.get(3 * ni + 2).unwrap());
                        }
                    };

                    match face {
                        (Some(v1), Some(v2), Some(v3), None) => {
                            let mut data1 = v1.split("/");
                            let mut data2 = v2.split("/");
                            let mut data3 = v3.split("/");

                            data_to_vertex(&mut data1);
                            data_to_vertex(&mut data2);
                            data_to_vertex(&mut data3);
                        }
                        (Some(v1), Some(v2), Some(v3), Some(v4)) => {
                            // first tris
                            let mut data1 = v1.split("/");
                            let mut data2 = v2.split("/");
                            let mut data3 = v3.split("/");

                            data_to_vertex(&mut data1);
                            data_to_vertex(&mut data2);
                            data_to_vertex(&mut data3);

                            // second tris
                            let mut data1 = v1.split("/");
                            let mut data3 = v3.split("/");
                            let mut data4 = v4.split("/");

                            data_to_vertex(&mut data1);
                            data_to_vertex(&mut data3);
                            data_to_vertex(&mut data4);
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
