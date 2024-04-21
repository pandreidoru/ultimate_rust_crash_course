// Run: cargo run --release blur image.png blurred.png
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::str::FromStr;

/// Image processing application
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Blur(CmdArgs),
    Brighten(CmdArgs),
    Crop(CropArgs),
    Rotate(RotateArgs),
}

#[derive(Args, Debug)]
struct CmdArgs {
    #[command(flatten)]
    files: IOFiles,
    factor: Option<i32>,
}

#[derive(Args, Debug)]
struct IOFiles {
    infile: String,
    outfile: String,
}

#[derive(Args, Debug)]
struct CropArgs {
    #[command(flatten)]
    files: IOFiles,
    #[command(flatten)]
    point: Point,
    #[command(flatten)]
    dimensions: Dimensions,
}

#[derive(Args, Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Args, Debug)]
struct Dimensions {
    width: u32,
    height: u32,
}

#[derive(Args, Debug)]
struct RotateArgs {
    #[command(flatten)]
    files: IOFiles,
    #[arg(value_enum)]
    angle: Angle,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Angle {
    _90,
    _180,
    _270,
}

impl FromStr for Angle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "90" => Ok(Angle::_90),
            "180" => Ok(Angle::_180),
            "270" => Ok(Angle::_270),
            _ => Err("Invalid angle".to_string()),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.cmd {
        Commands::Blur(args) => {
            blur(&args)
        },
        Commands::Brighten(args) => {
            brighten(&args)
        },
        Commands::Crop(args) => {
            crop(&args)
        },
        Commands::Rotate(args) => {
            rotate(&args)
        }
    }
}

fn blur(args: &CmdArgs) {
    let img = image::open(&args.files.infile).expect("Failed to open INFILE.");

    let img2 = img.blur(args.factor.unwrap_or(10) as f32);

    img2.save(&args.files.outfile).expect("Failed writing OUTFILE.");
}

fn brighten(args: &CmdArgs) {
    let img = image::open(&args.files.infile).expect("Failed to open INFILE.");

    let img2 = img.brighten(args.factor.unwrap_or(10));

    img2.save(&args.files.outfile).expect("Failed writing OUTFILE.");
}

fn crop(args: &CropArgs) {
    let mut img = image::open(&args.files.infile).expect("Failed to open INFILE.");

    let img2 = img.crop(args.point.x, args.point.y, args.dimensions.width, args.dimensions.height);

    img2.save(&args.files.outfile).expect("Failed writing OUTFILE.");
}

fn rotate(args: &RotateArgs) {
    let img = image::open(&args.files.infile).expect("Failed to open image");

    let rotated_img = match args.angle {
        Angle::_90 => img.rotate90(),
        Angle::_180 => img.rotate180(),
        Angle::_270 => img.rotate270(),
    };

    rotated_img.save(&args.files.outfile).expect("Failed writing OUTFILE.");
}
//         "invert" => {
//             if args.len() != 2 {
//                 print_usage_and_exit();
//             }
//             let infile = args.remove(0);
//             let outfile = args.remove(0);
//             invert(infile, outfile);
//         },
//         "grayscale" => {
//             if args.len() != 2 {
//                 print_usage_and_exit();
//             }
//             let infile = args.remove(0);
//             let outfile = args.remove(0);
//             grayscale(infile, outfile);
//         },
//         "generate" => {
//             if args.len() != 1 {
//                 print_usage_and_exit();
//             }
//             let outfile = args.remove(0);
//             generate(outfile);
//         }
//         // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
//         "fractal" => {
//             if args.len() != 1 {
//                 print_usage_and_exit();
//             }
//             let outfile = args.remove(0);
//             fractal(outfile);
//         }
//
//
//
// fn invert(infile: String, outfile: String) {
//     let mut img = image::open(infile).expect("Failed to open image");
//     img.invert();
//     img.save(outfile).expect("Failed writing OUTFILE.");
// }
//
// fn grayscale(infile: String, outfile: String) {
//     let img = image::open(infile).expect("Failed to open INFILE.");
//     let img2 = img.grayscale();
//     img2.save(outfile).expect("Failed writing OUTFILE.");
// }
//
// fn generate(outfile: String) {
//     let width = 800;
//     let height = 600;
//     let mut imgbuf = image::ImageBuffer::new(width, height);
//
//     for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
//         *pixel = image::Rgb([255u8, 0u8, 0u8]);
//     }
//     // Challenge: parse some color data from the command-line, pass it through
//     // to this function to use for the solid color.
//
//     // Challenge 2: Generate something more interesting!
//
//     imgbuf.save(outfile).unwrap();
// }
//
// // This code was adapted from https://github.com/PistonDevelopers/image
// fn fractal(outfile: String) {
//     let width = 800;
//     let height = 800;
//
//     let mut imgbuf = image::ImageBuffer::new(width, height);
//
//     let scale_x = 3.0 / width as f32;
//     let scale_y = 3.0 / height as f32;
//
//     // Iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         // Use red and blue to be a pretty gradient background
//         let red = (0.3 * x as f32) as u8;
//         let blue = (0.3 * y as f32) as u8;
//
//         // Use green as the fractal foreground (here is the fractal math part)
//         let cx = y as f32 * scale_x - 1.5;
//         let cy = x as f32 * scale_y - 1.5;
//
//         let c = num_complex::Complex::new(-0.4, 0.6);
//         let mut z = num_complex::Complex::new(cx, cy);
//
//         let mut green = 0;
//         while green < 255 && z.norm() <= 2.0 {
//             z = z * z + c;
//             green += 1;
//         }
//
//         // Actually set the pixel. red, green, and blue are u8 values!
//         *pixel = image::Rgb([red, green, blue]);
//     }
//
//     imgbuf.save(outfile).unwrap();
// }
//
// // **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
// //
// // Make all of the subcommands stackable!
// //
// // For example, if you run:
// //
// //   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
// //
// // ...then your program would:
// // - read infile.png
// // - apply a blur of 2.5
// // - invert the colors
// // - rotate the image 180 degrees clockwise
// // - brighten the image by 10
// // - and write the result to outfile.png
// //
// // Good luck!
