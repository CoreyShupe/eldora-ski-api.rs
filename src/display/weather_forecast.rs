use chrono::{DateTime, Local, Utc};
use prettytable::{Cell, Row, Table, row};

pub async fn print_data(client: &impl crate::EldoraSkiApi) {
    let forecast_data = client.get_weather_forecast().await;

    print!("\x1B[2J\x1B[H");
    std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush stdout");

    println!(
        "{}===\n\tTemp: {} Icon: {}\n",
        "===+".repeat(9),
        forecast_data.current.temperature,
        forecast_data.current.weather_icon
    );

    // sensors
    let sensors = forecast_data.sensors;
    let mut sensors_table = Table::new();
    sensors_table.add_row(row![
        "Sensor",
        "Temperature",
        "Relative Humidity",
        "Last Published"
    ]);
    for sensor in sensors {
        if sensor.relative_humidity.is_none() && sensor.temperature.is_none() {
            continue;
        }

        let pub_date = DateTime::parse_from_rfc3339(sensor.pub_date.as_str())
            .expect("Invalid date format")
            .with_timezone(&Utc);

        // Convert to local timezone
        let local_datetime: DateTime<Local> = pub_date.with_timezone(&Local);

        // Format it in a friendly way
        let formatted_pub_date = local_datetime.format("%B %d, %Y %I:%M:%S %p").to_string();

        sensors_table.add_row(row![
            Cell::new(sensor.name.as_str()),
            Cell::new(&sensor.temperature.unwrap().to_string()),
            Cell::new(&sensor.relative_humidity.unwrap().to_string()),
            Cell::new(&formatted_pub_date),
        ]);
    }

    sensors_table.printstd();

    // blank line
    println!();

    // forecast
    let weekly_forecast = forecast_data.forecast;

    let mut weekly_forecast_table = Table::new();

    let mut day_row = vec![Cell::new("")];
    let mut temp_row = vec![Cell::new("Temp")];
    let mut desc_row = vec![Cell::new("Desc")];
    let mut precipitation_row = vec![Cell::new("Precipitation")];
    let mut wind_row = vec![Cell::new("Wind")];

    for forecast in weekly_forecast.iter() {
        day_row.push(Cell::new(&forecast.day[0..3]));
        temp_row.push(Cell::new(&format!(
            "{}→{}",
            forecast.temperature_low, forecast.temperature_high
        )));
        desc_row.push(Cell::new(&forecast.description_short));
        precipitation_row.push(Cell::new(&format!(
            "{}→{}",
            forecast.precipitation_low, forecast.precipitation_high
        )));
        wind_row.push(Cell::new(&format!(
            "{}{}",
            forecast.wind_speed, forecast.wind_direction
        )))
    }

    weekly_forecast_table.add_row(Row::new(day_row));
    weekly_forecast_table.add_row(Row::new(temp_row));
    weekly_forecast_table.add_row(Row::new(desc_row));
    weekly_forecast_table.add_row(Row::new(precipitation_row));
    weekly_forecast_table.add_row(Row::new(wind_row));
    weekly_forecast_table.printstd();
}
