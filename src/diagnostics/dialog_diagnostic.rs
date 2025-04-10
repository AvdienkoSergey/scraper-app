use crate::library::dialog::DialogBuilder;

pub fn dialog_diagnostic() -> Result<(), std::io::Error> {
    let dialog = DialogBuilder::new();
    
    dialog.title("🚀 Диагностика работы с диалогами");

    let hero_vec = vec!["Евпатий", "Билл", "Есик", "Абдулла"];
    let car_color_vec = vec!["Красный", "Синий", "Зеленый", "Желтый"];
    
    let hero = dialog.select(
        "Выберите любимого героя",
        &hero_vec
    )?;

    let hero_name = hero_vec[hero];
    
    let car = dialog.input("Введите марку автомобиля", Some("BMW"))?;

    let colors = dialog.multi_select(
      "Выберите цвет машины",
      &car_color_vec
    )?;

    let color = if colors.is_empty() {
      "прозрачного"
    } else if colors.len() < 2 {
      let color_name = car_color_vec[colors[0]];
      match color_name {
        "Красный" => "красного",
        "Синий" => "синего",
        "Зеленый" => "зеленого",
        "Желтый" => "желтого",
        _ => "непонятного",
      }
       
    } else {
        "непонятного"
    };
    
    let confirm = dialog.confirm("Хотите отправить героя на борьбу с монстрами?", true)?;
    
    if confirm {
        dialog.success(&format!("{} отправлен на борьбу с монстрами на машине {} {} цвета", hero_name, car, color));
    } else {
        dialog.info("Отправка героя отменена");
    }
    
    Ok(())
}