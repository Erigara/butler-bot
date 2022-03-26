use handlebars::{
    Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError, TemplateError,
};

fn fmt_temperature(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{}", param.value().render().replace("-", "—"));
    out.write(rendered.as_ref())?;
    Ok(())
}

fn fmt_date(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{}", param.value().render().replace("-", "·"));
    out.write(rendered.as_ref())?;
    Ok(())
}

fn fmt_time(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{:0>2}", param.value().render().replace("00", ""));
    out.write(rendered.as_ref())?;
    Ok(())
}

fn fmt_wwo_code(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;

    let wwo_code = param.value().render();

    let description = match wwo_code.as_str() {
        "113" => "Sunny",
        "116" => "PartlyCloudy",
        "119" => "Cloudy",
        "122" => "VeryCloudy",
        "143" => "Fog",
        "176" => "LightShowers",
        "179" => "LightSleetShowers",
        "182" => "LightSleet",
        "185" => "LightSleet",
        "200" => "ThunderyShowers",
        "227" => "LightSnow",
        "230" => "HeavySnow",
        "248" => "Fog",
        "260" => "Fog",
        "263" => "LightShowers",
        "266" => "LightRain",
        "281" => "LightSleet",
        "284" => "LightSleet",
        "293" => "LightRain",
        "296" => "LightRain",
        "299" => "HeavyShowers",
        "302" => "HeavyRain",
        "305" => "HeavyShowers",
        "308" => "HeavyRain",
        "311" => "LightSleet",
        "314" => "LightSleet",
        "317" => "LightSleet",
        "320" => "LightSnow",
        "323" => "LightSnowShowers",
        "326" => "LightSnowShowers",
        "329" => "HeavySnow",
        "332" => "HeavySnow",
        "335" => "HeavySnowShowers",
        "338" => "HeavySnow",
        "350" => "LightSleet",
        "353" => "LightShowers",
        "356" => "HeavyShowers",
        "359" => "HeavyRain",
        "362" => "LightSleetShowers",
        "365" => "LightSleetShowers",
        "368" => "LightSnowShowers",
        "371" => "HeavySnowShowers",
        "374" => "LightSleetShowers",
        "377" => "LightSleet",
        "386" => "ThunderyShowers",
        "389" => "ThunderyHeavyRain",
        "392" => "ThunderySnowShowers",
        "395" => "HeavySnowShowers",
        _ => "Unknown",
    };

    let symbol = match description {
        "Cloudy" => "☁️",
        "Fog" => "🌫",
        "HeavyRain" => "🌧",
        "HeavyShowers" => "🌧",
        "HeavySnow" => "❄️",
        "HeavySnowShowers" => "❄️",
        "LightRain" => "🌦",
        "LightShowers" => "🌦",
        "LightSleet" => "🌧",
        "LightSleetShowers" => "🌧",
        "LightSnow" => "🌨",
        "LightSnowShowers" => "🌨",
        "PartlyCloudy" => "⛅️",
        "Sunny" => "☀️",
        "ThunderyHeavyRain" => "🌩",
        "ThunderyShowers" => "⛈",
        "ThunderySnowShowers" => "⛈",
        "VeryCloudy" => "☁️",
        _ => "✨",
    };

    out.write(symbol.as_ref())?;
    Ok(())
}

pub fn create_registry<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut reg = Handlebars::new();
    reg.register_helper("fmt_temperature", Box::new(fmt_temperature));
    reg.register_helper("fmt_date", Box::new(fmt_date));
    reg.register_helper("fmt_time", Box::new(fmt_time));
    reg.register_helper("fmt_wwo_code", Box::new(fmt_wwo_code));
    reg.register_template_file("weather", "templates/weather.hbs")?;

    Ok(reg)
}
