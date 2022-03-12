use handlebars::{
    Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError, TemplateError,
};


fn fmt_weather(
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

pub fn new<'a>() -> Result<Handlebars<'a>, TemplateError> {
    let mut reg = Handlebars::new();
    reg.register_helper("fmt_weather", Box::new(fmt_weather));
    reg.register_helper("fmt_date", Box::new(fmt_date));
    reg.register_template_file("weather", "templates/weather.hbs")?;

    Ok(reg)
}