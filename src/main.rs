use anyhow::anyhow;
use printpdf::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use time::{Date, Month, Weekday};

type Result<T> = std::result::Result<T, anyhow::Error>;

struct PageSize {
    width_mm: f64,
    height_mm: f64,
}

fn generate_pdf_for_date(
    start_date: &Date,
    page_size: &PageSize,
    font_filename: &str,
) -> Result<Vec<u8>> {
    let page_width = page_size.width_mm;
    let page_height = page_size.height_mm;

    let (doc, page1, layer1) = PdfDocument::new(
        "PDF_Document_title",
        Mm(page_width),
        Mm(page_height),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let mut number_of_days = 0;
    let mut day = *start_date;
    let mut days: Vec<Date> = Vec::new();
    while day.month() == start_date.month() {
        days.push(day);

        day = day
            .next_day()
            .ok_or_else(|| anyhow!("reached the end of Dates"))?;

        number_of_days += 1;
    }

    let gray_shade = 0.9;

    let column_label_font_size_mm = 5.0;
    let column_label_height = 7.0;
    let column_header_offset_x = 1.0;
    let column_header_offset_y = (column_label_height - column_label_font_size_mm) / 2.0;

    let font_file = File::open(font_filename)?;

    let font = doc.add_external_font(font_file)?;

    let page_padding = 5.0;
    let cell_padding = 1.0;

    let effective_page_width = page_width - page_padding * 2.0;
    let effective_page_height = page_height - page_padding * 2.0;

    let table_height = effective_page_height - column_label_height;

    let table_rows = number_of_days;
    let table_cell_height = table_height / (table_rows as f64);

    let row_label_font_size_mm = table_cell_height * 0.75;
    let row_label_offset_y = (table_cell_height - row_label_font_size_mm) / 2.0;

    let row_label_width = 50.0;

    let full_table_left = page_padding;
    let full_table_top = page_height - page_padding;

    let table_body_left = page_padding + row_label_width;
    let table_body_bottom = page_padding;

    let table_width = effective_page_width - row_label_width;

    let table_body_top = table_body_bottom + table_height;
    let table_body_right = table_body_left + table_width;

    let table_columns = 24;
    let table_cell_width = table_width / (table_columns as f64);

    current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    current_layer.set_outline_thickness(0.5);
    current_layer.add_shape(Line {
        points: vec![
            (
                Point::new(Mm(table_body_left), Mm(table_body_bottom)),
                false,
            ),
            (Point::new(Mm(table_body_left), Mm(table_body_top)), false),
            (Point::new(Mm(table_body_right), Mm(table_body_top)), false),
            (
                Point::new(Mm(table_body_right), Mm(table_body_bottom)),
                false,
            ),
        ],
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });

    current_layer.add_shape(Line {
        points: vec![
            (
                Point::new(Mm(full_table_left), Mm(table_body_bottom)),
                false,
            ),
            (Point::new(Mm(full_table_left), Mm(table_body_top)), false),
        ],
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });

    current_layer.add_shape(Line {
        points: vec![
            (Point::new(Mm(table_body_left), Mm(full_table_top)), false),
            (Point::new(Mm(table_body_right), Mm(full_table_top)), false),
        ],
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    });

    for i in 0..table_rows {
        let y0 = table_body_bottom + (i as f64) * table_cell_height;
        let y1 = table_body_bottom + ((i + 1) as f64) * table_cell_height;

        current_layer.set_fill_color(Color::Rgb(Rgb::new(
            gray_shade, gray_shade, gray_shade, None,
        )));

        if i % 2 == 0 {
            current_layer.add_shape(Line {
                points: vec![
                    (Point::new(Mm(full_table_left), Mm(y0)), false),
                    (Point::new(Mm(full_table_left), Mm(y1)), false),
                    (Point::new(Mm(table_body_right), Mm(y1)), false),
                    (Point::new(Mm(table_body_right), Mm(y0)), false),
                ],
                is_closed: true,
                has_fill: true,
                has_stroke: false,
                is_clipping_path: false,
            });
        }
    }

    for i in 0..=table_columns {
        let x0 = table_body_left + (i as f64) * table_cell_width;

        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));

        let fat_column = i < table_columns && (i % 6) == 0;

        current_layer.set_outline_thickness(if fat_column { 1.0 } else { 0.5 });

        current_layer.add_shape(Line {
            points: vec![
                (Point::new(Mm(x0), Mm(table_body_bottom)), false),
                (Point::new(Mm(x0), Mm(full_table_top)), false),
            ],
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        });
    }

    for i in 0..=table_rows {
        let y = table_body_bottom + (i as f64) * table_cell_height;

        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_outline_thickness(0.5);

        current_layer.add_shape(Line {
            points: vec![
                (Point::new(Mm(full_table_left), Mm(y)), false),
                (Point::new(Mm(table_body_right), Mm(y)), false),
            ],
            is_closed: false,
            has_fill: false,
            has_stroke: true,
            is_clipping_path: false,
        });
    }

    for i in 0..table_columns {
        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_outline_thickness(0.5);

        let x0 = table_body_left + (i as f64) * table_cell_width;

        let text = format!("{:02}", i);

        current_layer.use_text(
            text,
            Mm(column_label_font_size_mm).into_pt().0,
            Mm(x0 + cell_padding + column_header_offset_x),
            Mm(table_body_top + cell_padding + column_header_offset_y),
            &font,
        );
    }

    for (index, day) in days.iter().enumerate() {
        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        current_layer.set_outline_thickness(0.5);

        let y_bottom = table_body_bottom + ((table_rows - index - 1) as f64) * table_cell_height;

        let short_weekday_name = match day.weekday() {
            Weekday::Monday => "Mon",
            Weekday::Tuesday => "Tue",
            Weekday::Wednesday => "Wed",
            Weekday::Thursday => "Thu",
            Weekday::Friday => "Fri",
            Weekday::Saturday => "Sat",
            Weekday::Sunday => "Sun",
        };

        let text = format!("{} {}", day, short_weekday_name);

        current_layer.use_text(
            text,
            Mm(row_label_font_size_mm).into_pt().0,
            Mm(full_table_left + cell_padding),
            Mm(y_bottom + cell_padding + row_label_offset_y),
            &font,
        );
    }

    let mut buf = BufWriter::new(Vec::new());

    doc.save(&mut buf)?;

    Ok(buf.into_inner()?)
}

fn main() -> Result<()> {
    let a4_landscape = PageSize {
        width_mm: 297.0,
        height_mm: 210.0,
    };

    let start_date = Date::from_calendar_date(2023, Month::January, 1)?;

    let data = generate_pdf_for_date(
        &start_date,
        &a4_landscape,
        "fonts/LiberationSans-Regular.ttf",
    )?;

    let mut file = File::create("output.pdf")?;
    file.write_all(&data)?;

    Ok(())
}
