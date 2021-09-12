macro_rules! impl_ui_for_simple_enum {
    ($fn_name:ident, $name:ty: $($variant:ident),* ) => {
        fn $fn_name(element: &mut $name, ui: &mut bevy_egui::egui::Ui, context: &Context) -> bool {
            let mut changed = false;
            bevy_inspector_egui::egui::ComboBox::from_id_source(context.id())
                .selected_text(format!("{:?}", element))
                .show_ui(ui, |ui| {
                $(
                    if ui.selectable_label(matches!(element, <$name>::$variant), format!("{:?}", <$name>::$variant)).clicked() {
                        *element = <$name>::$variant;
                        changed = true;
                    }
                )*
            });
            changed
        }
    }

}

pub(crate) use impl_ui_for_simple_enum;
