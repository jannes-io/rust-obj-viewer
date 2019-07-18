use std::fs;

use crate::geometry::*;
use crate::object::Object;

fn str_to_vec3(s: &str) -> Option<Vec3> {
    let bits: Vec<&str> = s.split(" ").collect();
    if bits.len() != 3 {
        None
    } else {
        Some(Vec3 {
            x: bits[0].parse().unwrap(),
            y: bits[1].parse().unwrap(),
            z: bits[2].parse().unwrap(),
        })
    }
}

fn str_to_vec2(s: &str) -> Option<Vec2> {
    let bits: Vec<&str> = s.split(" ").collect();
    if bits.len() != 2 {
        None
    } else {
        Some(Vec2 {
            x: bits[0].parse().unwrap(),
            y: bits[1].parse().unwrap(),
        })
    }
}

type VertexDescription = (usize, usize, usize);
type TriangleDescription = (VertexDescription, VertexDescription, VertexDescription);

fn str_to_vertex_description(s: &str) -> Option<VertexDescription> {
    let bits: Vec<&str> = s.split("/").collect();
    if bits.len() != 3 {
        None
    } else {
        Some((
            bits[0].parse::<usize>().unwrap() - 1,
            bits[1].parse::<usize>().unwrap() - 1,
            bits[2].parse::<usize>().unwrap() - 1,
        ))
    }
}

fn str_to_triangle_description(s: &str) -> Option<TriangleDescription> {
    let bits: Vec<&str> = s.split(" ").collect();
    if bits.len() != 3 {
        None
    } else {
        Some((
            str_to_vertex_description(bits[0]).expect("Failed to parse vertex"),
            str_to_vertex_description(bits[1]).expect("Failed to parse vertex"),
            str_to_vertex_description(bits[2]).expect("Failed to parse vertex"),
        ))
    }
}

pub fn parse(path: String) -> Object {
    let object_string = fs::read_to_string(path).expect("Cannot read .obj file!");
    let lines = object_string.lines();
    let mut positions: Vec<Vec3> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut uvs: Vec<Vec2> = vec![];
    let mut triangle_descriptions: Vec<TriangleDescription> = vec![];

    for line in lines {
        match &line[0..2] {
            "v " => match str_to_vec3(&line[2..]) {
                Some(vec3) => positions.push(vec3),
                None => panic!("Malformed 'v' value!"),
            },
            "vn" => match str_to_vec3(&line[3..]) {
                Some(vec3) => normals.push(vec3),
                None => panic!("Malformed 'vn' value!"),
            },
            "vt" => match str_to_vec2(&line[3..]) {
                Some(vec2) => uvs.push(vec2),
                None => panic!("Malformed 'vt' value!"),
            },
            "f " => match str_to_triangle_description(&line[2..]) {
                Some(td) => triangle_descriptions.push(td),
                None => panic!("Malformed 'f' value!"),
            },
            _ => {}
        }
    }

    let triangles: Vec<Triangle3> = triangle_descriptions
        .iter()
        .map(|&td| {
            let v1: Vertex3 = Vertex3 {
                pos: positions[(td.0).0],
                uv: uvs[(td.0).1],
                normal: normals[(td.0).2],
            };
            let v2: Vertex3 = Vertex3 {
                pos: positions[(td.1).0],
                uv: uvs[(td.1).1],
                normal: normals[(td.1).2],
            };
            let v3: Vertex3 = Vertex3 {
                pos: positions[(td.2).0],
                uv: uvs[(td.2).1],
                normal: normals[(td.2).2],
            };
            (v1, v2, v3)
        })
        .collect();

    let mut obj = Object::new();
    obj.triangles = triangles;
    obj
}
