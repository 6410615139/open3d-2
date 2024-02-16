use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug)]
struct Vertex {
    x: f64,
    y: f64,
    z: f64,
    nx: f64,
    ny: f64,
    nz: f64,
    red: u8,
    green: u8,
    blue: u8,
}

fn main() -> io::Result<()> {
    let file_path = Path::new("../temp/cropped_z-95_-5.ply");
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut found_endheader = false;

    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if found_endheader {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 9 {
                let vertex = Vertex {
                    x: parts[0].parse().unwrap(),
                    y: parts[1].parse().unwrap(),
                    z: parts[2].parse().unwrap(),
                    nx: parts[3].parse().unwrap(),
                    ny: parts[4].parse().unwrap(),
                    nz: parts[5].parse().unwrap(),
                    red: parts[6].parse().unwrap(),
                    green: parts[7].parse().unwrap(),
                    blue: parts[8].parse().unwrap(),
                };
                vertices.push(vertex);
            }
        } else if line.contains("end_header") {
            found_endheader = true;
        }
    }

    let red_th = 10;
    let green_th = 10;
    let blue_th = 10;
    let black_th = 50;
    let yellow_th = 10;

    // Filtering vertices based on conditions
    let filtered_vertices: Vec<&Vertex> = vertices.iter()
        .filter(|v| v.nz >= 0.9 && v.nx > -0.2 && v.nx < 0.2 && v.ny > -0.2 && v.ny < 0.2
            && (((v.green as i32 + v.blue as i32 + v.red as i32) / 3) > 50)
            && ((v.red as i32 - red_th) < ((v.green as i32 + v.blue as i32) / 2))
            && ((v.green as i32 - green_th) < ((v.red as i32 + v.blue as i32) / 2))
            && ((v.blue as i32 - blue_th) < ((v.red as i32 + v.green as i32) / 2))
            && ((v.blue as i32 + yellow_th) > ((v.red as i32 + v.green as i32) / 2)))
        .collect();

        let output_file_name = format!("../temp/filtered_{}{}{}_black{}_yellow{}.ply", red_th, green_th, blue_th, black_th, yellow_th);
        let output_path = Path::new(&output_file_name);
    let mut output_file = File::create(&output_path)?;

    // Re-read the original file for the header
    let file = File::open(&file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains("end_header") {
            writeln!(output_file, "{}", line)?;
            break;
        } else {
            if line.starts_with("element vertex") {
                writeln!(output_file, "element vertex {}", filtered_vertices.len())?;
            } else {
                writeln!(output_file, "{}", line)?;
            }
        }
    }

    // Write filtered vertices
    for vertex in filtered_vertices {
        writeln!(output_file, "{} {} {} {} {} {} {} {} {}", vertex.x, vertex.y, vertex.z, vertex.nx, vertex.ny, vertex.nz, vertex.red, vertex.green, vertex.blue)?;
    }

    Ok(())
}
