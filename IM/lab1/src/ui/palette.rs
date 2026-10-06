use egui::Color32;

/// Поле «Жизни»: белый лист.
pub const FIELD: Color32 = Color32::WHITE;
/// Живая клетка «Жизни»: чёрная на белом поле.
pub const ALIVE: Color32 = Color32::BLACK;
/// Вид «Добыча» — чёрный, как обычная живая клетка.
pub const PREY: Color32 = Color32::BLACK;
/// Вид «Хищник» — красный, чтобы фронты хищничества читались на белом поле.
pub const PREDATOR: Color32 = Color32::from_rgb(0xD3, 0x2F, 0x2F);
pub const EMPTY: Color32 = Color32::from_rgb(0x1E, 0x1E, 0x1E);
pub const CITIZEN: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);
pub const WATER: Color32 = Color32::from_rgb(0x1E, 0x6F, 0xFF);
pub const FOOD: Color32 = Color32::from_rgb(0x2E, 0xCC, 0x40);
pub const ENERGY: Color32 = Color32::from_rgb(0xFF, 0xD4, 0x00);
pub const POPULATION: Color32 = Color32::from_rgb(0xD9, 0xD9, 0xD9);
pub const CONFLICT: Color32 = Color32::from_rgb(0x8A, 0x2B, 0xE2);
pub const OVERLAP: Color32 = Color32::from_rgb(0xFF, 0x7A, 0x00);
pub const ANTICELL: Color32 = Color32::from_rgb(0xE0, 0x1B, 0x24);
pub const BORDER: Color32 = Color32::from_rgb(0x00, 0x00, 0x00);

/// Normalised `0..1` interpolation between `transparent` and `color`, used by
/// the stress / fatigue / loyalty heat maps (§8.6).
pub fn heat(color: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let [r, g, b, _] = color.to_array();
    Color32::from_rgba_unmultiplied(r, g, b, (255.0 * t) as u8)
}
