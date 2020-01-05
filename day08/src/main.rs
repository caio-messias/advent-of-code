use std::fs;

fn read_input() -> Vec<char>{
    return fs::read_to_string("input").unwrap()
        .chars()
        .collect();
}

#[derive(Debug)]
struct Layer {
    data: Vec<Vec<char>>,
}

impl Layer {
    fn count_zeroes(&self) -> usize {
        self.data.iter().fold(0, |zeroes, chars|
            zeroes + chars.iter().filter(|&c| *c == '0' ).count()
        )
    }

    fn count_ones_and_twos(&self) -> (usize, usize) {
        self.data.iter().fold((0, 0), |(ones, twos), chars|
            (ones + chars.iter().filter(|&c| *c == '1' ).count(),
             twos + chars.iter().filter(|&c| *c == '2' ).count())
        )
    }
}

fn build_layers(chars: &[char], width: usize, height: usize) -> Vec<Layer> {
    let mut layers = vec![];

    for layer_chars in chars.chunks(width * height) {
        let mut layer = vec![];
        for line_of_chars in layer_chars.chunks(width) {
            layer.push(line_of_chars.to_vec());
        }
        layers.push(Layer { data: layer })
    }

    return layers
}

fn checksum(layers: &Vec<Layer>) -> i32 {
    let (fewest_zeroes_layer, _num_zeros) = layers.iter()
        .fold((&layers[0], layers[0].count_zeroes()),
            |(fewest_zeroes_layer, num_zeros), current_layer|
                if current_layer.count_zeroes() < num_zeros {
                    (current_layer, current_layer.count_zeroes())
                } else {
                    (fewest_zeroes_layer, num_zeros)
                });

    let ones_and_twos = fewest_zeroes_layer.count_ones_and_twos();
    return (ones_and_twos.0 * ones_and_twos.1) as i32;
}

fn build_image(layers: &Vec<Layer>) -> Vec<Vec<char>> {
    let width = layers[0].data[0].len();
    let height = layers[0].data.len();
    let mut image = vec![vec!['0'; width]; height];

    for i in 0..height {
        for j in 0..width {
            for layer in layers {
                if layer.data[i][j] == '2' { continue }
                else {
                    image[i][j] = layer.data[i][j];
                    break;
               }
           }
        }
    }

    return image;
}

fn print_image(image: &Vec<Vec<char>>) {
    let height = image.len();
    let width = image[0].len();

    for i in 0..height {
        for j in 0..width {
            if image[i][j] == '1' {
                print!("*");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let chars = read_input();

    //Part 1
    let layers = build_layers(&chars, 25, 6);
    println!("Part 1: {}", checksum(&layers));

    //Part 2
    let image = build_image(&layers);
    print_image(&image);
}
