// Run: cargo run --release blur image.png blurred.png
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{Args, Parser, Subcommand};

/// Image processing application
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Blur(BlurArgs),
    Generate(IFile),
}

#[derive(Args, Debug)]
struct BlurArgs {
    infile: String,
    outfile: String,
    factor: f32,
}

#[derive(Args, Debug)]
struct IFile {
    infile: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.cmd {
        Commands::Blur(args) => {
            blur(&args.infile, &args.outfile, args.factor)
        },
        Commands::Generate(file) => {
            println!("{}", file.infile)
        }
    }
}

fn blur(infile: &String, outfile: &String, factor: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(factor);
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

//     let mut args: Vec<String> = std::env::args().skip(1).collect();
//     if args.is_empty() {
//         print_usage_and_exit();
//     }
//     let subcommand = args.remove(0);
//     match subcommand.as_str() {
//         "blur" => {
//             if args.len() != 2 {
//                 print_usage_and_exit();
//             }
//             let infile = args.remove(0);
//             let outfile = args.remove(0);
//             // **OPTION**
//             // Improve the blur implementation -- see the blur() function below
//             blur(infile, outfile);
//         },
//         "brighten" => {
//             if args.len() != 2 {
//                 print_usage_and_exit();
//             }
//             let infile = args.remove(0);
//             let outfile = args.remove(0);
//             brighten(infile, outfile);
//         },
//         "crop" => {
//             if args.len() != 2 {
//                 print_usage_and_exit();
//             }
//             let infile = args.remove(0);
//             let outfile = args.remove(0);
//             crop(infile, outfile);
//         },
//         "rotate" => {
//             if args.len() != 2 {
//                 print_usage_and_exit();
//             }
//             let infile = args.remove(0);
//             let outfile = args.remove(0);
//             rotate(infile, outfile);
//         },
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
//         // **OPTION**
//         // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
//
//         // For everything else...
//         _ => {
//             print_usage_and_exit();
//         }
//     }
// }
//
// fn print_usage_and_exit() {
//     println!("USAGE (when in doubt, use a .png extension on your filenames)");
//     println!("blur INFILE OUTFILE");
//     println!("fractal OUTFILE");
//     // **OPTION**
//     // Print useful information about what subcommands and arguments you can use
//     // println!("...");
//     std::process::exit(-1);
// }
//
// fn brighten(infile: String, outfile: String) {
//     let img = image::open(infile).expect("Failed to open INFILE.");
//     let img2 = img.brighten(30);
//
//     // Challenge: parse the brightness amount from the command-line and pass it
//     // through to this function.
//
//     img2.save(outfile).expect("Failed writing OUTFILE.");
// }
//
// fn crop(infile: String, outfile: String) {
//     let mut img = image::open(infile).expect("Failed to open INFILE.");
//     let img2 = img.crop(10, 10, 200, 300);
//
//     // Challenge: parse the four values from the command-line and pass them
//     // through to this function.
//
//     img2.save(outfile).expect("Failed writing OUTFILE.");
// }
//
// fn rotate(infile: String, outfile: String) {
//     let img = image::open(&infile).expect("Failed to open image");
//     let rotated_img = img.rotate180();
//
//     // Challenge: parse the rotation amount from the command-line, pass it
//     // through to this function to select which method to call.
//
//     rotated_img.save(&outfile).expect("Failed writing OUTFILE.");
// }
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
