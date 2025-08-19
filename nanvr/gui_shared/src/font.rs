use egui::{
    Context, FontData, FontFamily,
    epaint::{
        self,
        text::{FontInsert, InsertFontFamily},
    },
};
use egui_fonts::{HACK_REGULAR, OPENMOJI_BLACK_GLYF, UBUNTU_LIGHT};

pub fn add_fonts(ctx: &Context) {
    ctx.add_font(FontInsert::new(
        "Hack",
        FontData::from_static(HACK_REGULAR),
        vec![InsertFontFamily {
            family: FontFamily::Monospace,
            priority: epaint::text::FontPriority::Highest,
        }],
    ));
    ctx.add_font(FontInsert::new(
        "Ubuntu-Light",
        FontData::from_static(UBUNTU_LIGHT),
        vec![
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: epaint::text::FontPriority::Lowest,
            },
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: epaint::text::FontPriority::Lowest,
            },
        ],
    ));
    ctx.add_font(FontInsert::new(
        "OpenMoji-black-glyf",
        FontData::from_static(OPENMOJI_BLACK_GLYF),
        vec![
            InsertFontFamily {
                family: FontFamily::Monospace,
                priority: epaint::text::FontPriority::Lowest,
            },
            InsertFontFamily {
                family: FontFamily::Proportional,
                priority: epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}
