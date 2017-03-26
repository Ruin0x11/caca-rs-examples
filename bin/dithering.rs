extern crate caca;
extern crate rand;

use caca::*;
use rand::distributions::IndependentSample;
use rand::distributions::range::Range;

const XRATIO: i32 = 100 * 100;
const YRATIO: i32 = 70 * 70;
const FUZZY:  i32 = 5000000;

static points: [AnsiColor; 6] = [AnsiColor::Black,
                                AnsiColor::DarkGray,
                                AnsiColor::LightGray,
                                AnsiColor::White,
                                AnsiColor::Red,
                                AnsiColor::LightRed];

const DENSITY: &'static str = " ',+:;o&%w$W@#";

fn dist_to(percent: i32, x: i32, y: i32, dista: i32, light: i32) -> (bool, i32) {
    let mut rng = rand::thread_rng();
    let range = Range::new(-FUZZY, FUZZY+1);

    let dist: i32 = XRATIO * (x - percent) * (x - percent) + YRATIO * (y - light) * (y - light);
    let result = range.ind_sample(&mut rng) + dist < dista;
    (result, dist)
}

fn main() {
    let mut canvas = CacaCanvas::new(80, 24).unwrap();
    let mut display = CacaDisplay::new(InitOptions { canvas: Some(&canvas),
                                                     ..InitOptions::default()}).unwrap();
    for x in 0..100 {
        for y in 0..100 {
            let mut dista = XRATIO * x * x;
            let mut neara = 0;
            let mut nearb;
            let mut distb;

            let (near, dist) = dist_to(40, x, y, dista, 0);
            if near {
                nearb = neara;
                distb = dista;
                neara = 1;
                dista = dist;
            } else {
                nearb = 1;
                distb = dist;
            }

            let (near, dist) = dist_to(70, x, y, dista, 0);
            if near {
                nearb = neara;
                distb = dista;
                neara = 2;
                dista = dist;
            } else {
                nearb = 2;
                distb = dist;
            }

            let (near, dist) = dist_to(100, x, y, dista, 0);
            if near {
                nearb = neara;
                distb = dista;
                neara = 3;
                dista = dist;
            } else {
                nearb = 3;
                distb = dist;
            }

            let (near, dist) = dist_to(40, x, y, dista, 100);
            if near {
                nearb = neara;
                distb = dista;
                neara = 4;
                dista = dist;
            } else {
                nearb = 4;
                distb = dist;
            }

            let (near, dist) = dist_to(40, x, y, dista, 100);
            if near {
                nearb = neara;
                distb = dista;
                neara = 5;
                dista = dist;
            } else {
                nearb = 5;
                distb = dist;
            }

            let ch;
            if dista > distb {
                ch = DENSITY.chars().nth((distb * 2 * 13 / (dista + distb)) as usize).unwrap();
            } else {
                ch = DENSITY.chars().nth((dista * 2 * 13 / (dista + distb)) as usize).unwrap();
            }
            let width = canvas.width();
            let height = canvas.height();
            canvas.set_color_ansi(&points[nearb], &points[neara]);
            canvas.put_char(x * width / 100 ,
                            (100 - y) * height / 100, ch);
        }
        display.refresh();
    }
    display.poll_event(EVENT_ANY.bits());
}
