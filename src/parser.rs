use std::{fs::{self, File}, io::BufReader, path::Path};

use lib_rs::geometry::{AxisAlignedBox, Parallelogram, Sphere};
use serde::Deserialize;

use crate::{camera::Camera, renderer::Material, scene::Scene};
#[derive(Deserialize,Debug)]
struct Config{
    camera:CameraConfig,
    samples:u32,
    scene:Vec<Object>

}
#[derive(Debug,Deserialize)]
struct Object{
    geometry:Geometry,
    material:Material
}
#[derive(Deserialize,Debug)]
#[serde(tag="type")]
enum Geometry{
    Sphere(Sphere),
    Parallelogram(Parallelogram),
    AxisAlignedBox(AxisAlignedBox),
    // Circle(Circle),
    // Plane(Plane),

}
#[derive(Debug,Deserialize)]
struct CameraConfig{
    position:[f32;3],
    look_at:[f32;3],
    fov:f32,
    width:u32,
    height:u32,
}

// parse json scene description to scene
pub struct Parser{

}
impl Parser{
    pub fn parse(path:impl AsRef<Path>)->anyhow::Result<(Camera,Scene,u32)>{
        let reader = BufReader::new(File::open(path.as_ref())?);
        let config:Config = serde_json::from_reader(reader)?;
        todo!()
    }
}