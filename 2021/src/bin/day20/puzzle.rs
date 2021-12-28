use crate::algorithm::Algorithm;
use crate::image::Image;
use advent2021::ParseError;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn load_algo<T: BufRead>(lines: &mut Lines<T>) -> Result<Algorithm, ParseError> {
    lines
        .next()
        .ok_or_else(|| ParseError::new("want algorithm; got EOF"))??
        .parse()
}

fn load_image<T: BufRead>(lines: &mut Lines<T>) -> Result<Image, io::Error> {
    let mut image = Image::new();
    for (i, line) in lines.enumerate() {
        for (j, _) in line?.bytes().enumerate().filter(|&(_, c)| c == b'#') {
            image.insert((i as i32, j as i32));
        }
    }
    Ok(image)
}

fn skip_blank_line<T: BufRead>(lines: &mut Lines<T>) -> Result<(), ParseError> {
    let line = lines
        .next()
        .ok_or_else(|| ParseError::new("want blank line; got EOF"))??;
    if !line.is_empty() {
        let what = format!("want blank line; got {}", line);
        return Err(ParseError::new(what));
    }
    Ok(())
}

pub struct Puzzle {
    pub algo: Algorithm,
    pub image: Image,
}

impl Puzzle {
    pub fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let input = input.as_ref();
        let lines = &mut BufReader::new(File::open(input)?).lines();
        let algo = load_algo(lines)?;
        skip_blank_line(lines)?;
        let image = load_image(lines)?;
        Ok(Puzzle { algo, image })
    }
}
