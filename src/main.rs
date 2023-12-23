use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use std::{
    fs::File,
    io::Read,
    path::{self, PathBuf},
};
use structopt::StructOpt;

mod memory;
use memory::Memory;

mod image_direction;
use image_direction::ImageDirection;

// CLI arguments struct
#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "i", long = "input", parse(from_os_str))]
    input_path: PathBuf,
    // TODO: implement this
    //#[structopt(short = "tz", long = "timezone", default_value = "Europe/Paris")]
    //timezone: String,
}

fn main() {
    // Parse CLI arguments
    let opt = Opt::from_args();
    let input_path = opt.input_path;

    // Check that the input is valid
    let memories_path = input_path.join("memories.json");
    if !memories_path.exists()
        || !memories_path.is_file()
        || memories_path.extension().unwrap() != "json"
    {
        eprintln!("The input path does not contain a (valid) memories.json file");
        std::process::exit(1);
    }

    // Parse memories.json
    let memory = parse_memories_json(&input_path);

    // Iterate over the memories and write the metadata to the images
    for memory in memory {
        let taken_time = memory.taken_time.format("%Y-%m-%d %H:%M:%S").to_string();

        write_metadata(
            &input_path,
            &memory.front_image.path,
            ImageDirection::Front,
            &taken_time,
        );

        write_metadata(
            &input_path,
            &memory.back_image.path,
            ImageDirection::Back,
            &taken_time,
        );
    }
}

fn parse_memories_json(input_path: &path::Path) -> Vec<Memory> {
    let memories_path = input_path.join("memories.json");
    let mut file = File::open(memories_path).expect("Failed to open memories.json metadata file");
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .expect("Failed to read memories.json metadata file");

    let memory: Vec<Memory> =
        serde_json::from_str(&json_string).expect("Failed to parse memories.json metadata file");

    memory
}

fn write_metadata(
    input_path: &path::Path,
    image_path: &str,
    image_direction: ImageDirection,
    taken_time: &str,
) {
    let image_path = correct_image_path(input_path, image_path, image_direction);
    let mut metadata = Metadata::new_from_path(&image_path).expect("Failed to fetch metadata");
    metadata.set_tag(ExifTag::DateTimeOriginal(taken_time.to_string()));
    let yeet = metadata.write_to_file(&image_path);
    match yeet {
        Ok(_) => {}
        Err(_) => {
            eprintln!(
                "Failed to write metadata to {}",
                image_path.to_str().unwrap()
            );
        }
    }
    println!("Wrote metadata to {}", image_path.to_str().unwrap());
}

fn correct_image_path(
    input_path: &path::Path,
    image_path: &str,
    image_direction: ImageDirection,
) -> PathBuf {
    let filename = image_path
        .to_string()
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();

    match image_direction {
        ImageDirection::Front => {
            let final_path = input_path.join("Photos/bereal/").join(filename);
            final_path.clone()
        }
        ImageDirection::Back => {
            let final_path = input_path.join("Photos/post/").join(filename);
            final_path.clone()
        }
    }
}
