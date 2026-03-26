use clap::Parser;
use rich_rust::console::Console;
use rich_rust::renderables::table::{Cell, Column, Table};
use rich_rust::style::Style;
use rich_rust::text::JustifyMethod;
use serial_enumerator::{get_serial_list, SerialInfo};

#[derive(Parser)]
#[command(author, version, about = "List available serial ports")]
struct Args {
    /// Output as CSV (header + comma-separated rows)
    #[arg(long, conflicts_with = "json")]
    plain: bool,

    /// Output as JSON array
    #[arg(long, conflicts_with = "plain")]
    json: bool,
}

#[derive(serde::Serialize)]
struct SerialItem {
    name: String,
    vendor: String,
    product: String,
    usb: String,
}

impl SerialItem {
    fn from_serial_info(serial_info: SerialInfo) -> SerialItem {
        let dash = || Some(String::from("--"));
        let driver_info = serial_info.driver;
        SerialItem {
            name: serial_info.name,
            vendor: serial_info.vendor.or_else(|| driver_info).or_else(dash).unwrap(),
            product: serial_info.product.or_else(dash).unwrap(),
            usb: serial_info
                .usb_info
                .map(|u| format!("{}:{}", u.vid, u.pid))
                .or_else(dash)
                .unwrap(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let mut serials_info = get_serial_list();
    serials_info.sort_by(|a, b| a.name.cmp(&b.name));
    let serials: Vec<SerialItem> = serials_info
        .into_iter()
        .map(SerialItem::from_serial_info)
        .collect();

    if args.json {
        println!("{}", serde_json::to_string_pretty(&serials).unwrap());
    } else if args.plain {
        println!("name,vendor,product,usb");
        for s in &serials {
            println!("{},{},{},{}", s.name, s.vendor, s.product, s.usb);
        }
    } else {
        print_rich_table(&serials);
    }
}

fn print_rich_table(serials: &[SerialItem]) {
    let mut table = Table::new()
        .with_column(Column::new("Name").style(Style::new().bold()))
        .with_column(Column::new("Vendor").justify(JustifyMethod::Center))
        .with_column(Column::new("Product").justify(JustifyMethod::Center))
        .with_column(Column::new("USB").justify(JustifyMethod::Center));

    for s in serials {
        table.add_row_cells([
            Cell::new(s.name.as_str()),
            Cell::new(s.vendor.as_str()),
            Cell::new(s.product.as_str()),
            Cell::new(s.usb.as_str()),
        ]);
    }

    Console::new().print_renderable(&table);
}
