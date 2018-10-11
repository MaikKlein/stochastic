extern crate criterion_plot as plot;
extern crate csv;
#[macro_use]
extern crate serde_derive;
use plot::prelude::*;

#[derive(Debug, Deserialize)]
pub struct TempData {
    pub year: u32,
    pub surface: f32,
    pub lower_troposphere_uah: f32,
    pub lower_troposphere_rss: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct LinearEq {
    pub m: f32,
    pub b: f32,
}
impl LinearEq {
    pub fn eval_at(&self, x: f32) -> f32 {
        self.m * x + self.b
    }
}

pub fn parse_temp_data() -> Vec<TempData> {
    let mut temp_data_rdr =
        csv::Reader::from_path("data/temperature_fig-1.csv").expect("temp data not found");
    temp_data_rdr.deserialize().filter_map(|r| r.ok()).collect()
}

fn lineareq_from_year(start_year: u32, temp_data: &[TempData]) -> LinearEq {
    let data: Vec<_> = temp_data
        .iter()
        .filter(|t| t.year >= start_year)
        .map(|t| t.lower_troposphere_rss)
        .enumerate()
        .collect();
    let count = data.len() as f32;
    let x_mean = data.iter().map(|(idx, _)| *idx as f32).sum::<f32>() / count;
    let xx_mean = data.iter().map(|(idx, _)| idx.pow(2) as f32).sum::<f32>() / count;
    let y_mean = data.iter().map(|(_, temp)| temp).sum::<f32>() / count;
    let xy_mean = data.iter()
        .map(|(idx, temp)| *idx as f32 * temp)
        .sum::<f32>() / count;
    let m = (x_mean * y_mean - xy_mean) / (x_mean.powi(2) - xx_mean);
    let b = y_mean - m * x_mean;
    LinearEq { m, b }
}

fn main() {
    let temp_data = parse_temp_data();
    let data: Vec<_> = temp_data
        .iter()
        .map(|t| t.lower_troposphere_rss)
        .collect();
    let le_1980 = lineareq_from_year(1980, &temp_data);
    let le_1998 = lineareq_from_year(1998, &temp_data);
    let le_2008 = lineareq_from_year(2008, &temp_data);
    println!("{:#?}", le_1980);
    println!("{:#?}", le_1998);
    println!("{:#?}", le_2008);

    let ref xs: Vec<_> = (0..temp_data.len()).map(|i| i as f32).collect();
    let ref xs_1998: Vec<_> = temp_data.iter().enumerate().filter(|(idx, t)| t.year >= 1998).map(|(idx, _)| idx as f32).collect();
    let ref xs_2008: Vec<_> = temp_data.iter().enumerate().filter(|(idx, t)| t.year >= 2008).map(|(idx, _)| idx as f32).collect();
    Figure::new()
        .configure(Key, |k| {
            k.set(Boxed::Yes)
                .set(Position::Inside(Vertical::Top, Horizontal::Left))
        })
        .plot(
            Lines{
                x: xs_2008,
                y: xs.iter().map(|&x| le_2008.eval_at(x)),
            },
            |lp| {
                lp.set(Color::Red)
                    .set(Label("Linear 2008"))
                    .set(LineType::Dash)
                    .set(PointSize(1.))
                    .set(PointType::Circle)
            },
        )
        .plot(
            Lines{
                x: xs_1998,
                y: xs.iter().map(|&x| le_1998.eval_at(x)),
            },
            |lp| {
                lp.set(Color::Cyan)
                    .set(Label("Linear 1998"))
                    .set(LineType::Dash)
                    .set(PointSize(1.))
                    .set(PointType::Circle)
            },
        )
        .plot(
            Lines{
                x: xs,
                y: xs.iter().map(|&x| le_1980.eval_at(x)),
            },
            |lp| {
                lp.set(Color::Green)
                    .set(Label("Linear 1980"))
                    .set(LineType::Dash)
                    .set(PointSize(1.))
                    .set(PointType::Circle)
            },
        )
        .plot(
            LinesPoints {
                x: xs,
                y: xs.iter().map(|&x| data[x as usize]),
            },
            |lp| {
                lp.set(Color::DarkViolet)
                    .set(Label("Temp"))
                    .set(LineType::Dash)
                    .set(PointSize(1.))
                    .set(PointType::FilledCircle)
            },
        )
            .draw();
}
