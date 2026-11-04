//! Built-in, offline Twemoji stickers and their attribution.
use crate::studio::Studio;
use concat_project::Command;

/// Attribution for the unmodified Twemoji artwork, rasterized to PNG.
pub const CREDIT: &str = "Twemoji graphics — Twitter, Inc. and other contributors. https://github.com/twitter/twemoji\nLicensed under CC BY 4.0: https://creativecommons.org/licenses/by/4.0/\nSVG artwork rasterized to PNG for CrabCut.";
/// A sticker offered by the gallery.
pub struct Sticker {
    /// Stable Twemoji codepoint identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Transparent PNG, 384 pixels square.
    pub png: &'static [u8],
}
/// The sixty built-in stickers.
pub const ITEMS: &[Sticker] = &[
    Sticker {
        id: "1f602",
        name: "Risa",
        png: include_bytes!("../stickers/1f602.png"),
    },
    Sticker {
        id: "1f60d",
        name: "Enamorado",
        png: include_bytes!("../stickers/1f60d.png"),
    },
    Sticker {
        id: "1f60e",
        name: "Cool",
        png: include_bytes!("../stickers/1f60e.png"),
    },
    Sticker {
        id: "1f62e",
        name: "Sorpresa",
        png: include_bytes!("../stickers/1f62e.png"),
    },
    Sticker {
        id: "1f914",
        name: "Pensando",
        png: include_bytes!("../stickers/1f914.png"),
    },
    Sticker {
        id: "1f92f",
        name: "Explota la cabeza",
        png: include_bytes!("../stickers/1f92f.png"),
    },
    Sticker {
        id: "1f62d",
        name: "Llorando",
        png: include_bytes!("../stickers/1f62d.png"),
    },
    Sticker {
        id: "1f973",
        name: "Fiesta",
        png: include_bytes!("../stickers/1f973.png"),
    },
    Sticker {
        id: "1f525",
        name: "Fuego",
        png: include_bytes!("../stickers/1f525.png"),
    },
    Sticker {
        id: "1f4af",
        name: "Cien",
        png: include_bytes!("../stickers/1f4af.png"),
    },
    Sticker {
        id: "1f44f",
        name: "Aplauso",
        png: include_bytes!("../stickers/1f44f.png"),
    },
    Sticker {
        id: "1f440",
        name: "Ojos",
        png: include_bytes!("../stickers/1f440.png"),
    },
    Sticker {
        id: "1f44d",
        name: "Me gusta",
        png: include_bytes!("../stickers/1f44d.png"),
    },
    Sticker {
        id: "1f44e",
        name: "No me gusta",
        png: include_bytes!("../stickers/1f44e.png"),
    },
    Sticker {
        id: "1f44c",
        name: "OK",
        png: include_bytes!("../stickers/1f44c.png"),
    },
    Sticker {
        id: "270c",
        name: "Victoria",
        png: include_bytes!("../stickers/270c.png"),
    },
    Sticker {
        id: "1f91e",
        name: "Suerte",
        png: include_bytes!("../stickers/1f91e.png"),
    },
    Sticker {
        id: "1f64c",
        name: "Celebración",
        png: include_bytes!("../stickers/1f64c.png"),
    },
    Sticker {
        id: "1f64f",
        name: "Gracias",
        png: include_bytes!("../stickers/1f64f.png"),
    },
    Sticker {
        id: "1f4aa",
        name: "Fuerza",
        png: include_bytes!("../stickers/1f4aa.png"),
    },
    Sticker {
        id: "1f449",
        name: "Mira aquí",
        png: include_bytes!("../stickers/1f449.png"),
    },
    Sticker {
        id: "1f446",
        name: "Arriba",
        png: include_bytes!("../stickers/1f446.png"),
    },
    Sticker {
        id: "1f447",
        name: "Abajo",
        png: include_bytes!("../stickers/1f447.png"),
    },
    Sticker {
        id: "1f448",
        name: "Izquierda",
        png: include_bytes!("../stickers/1f448.png"),
    },
    Sticker {
        id: "2764",
        name: "Corazón",
        png: include_bytes!("../stickers/2764.png"),
    },
    Sticker {
        id: "1f49b",
        name: "Corazón amarillo",
        png: include_bytes!("../stickers/1f49b.png"),
    },
    Sticker {
        id: "1f9e1",
        name: "Corazón naranja",
        png: include_bytes!("../stickers/1f9e1.png"),
    },
    Sticker {
        id: "2728",
        name: "Brillos",
        png: include_bytes!("../stickers/2728.png"),
    },
    Sticker {
        id: "2b50",
        name: "Estrella",
        png: include_bytes!("../stickers/2b50.png"),
    },
    Sticker {
        id: "1f4a5",
        name: "Impacto",
        png: include_bytes!("../stickers/1f4a5.png"),
    },
    Sticker {
        id: "2705",
        name: "Correcto",
        png: include_bytes!("../stickers/2705.png"),
    },
    Sticker {
        id: "274c",
        name: "Error",
        png: include_bytes!("../stickers/274c.png"),
    },
    Sticker {
        id: "2753",
        name: "Pregunta",
        png: include_bytes!("../stickers/2753.png"),
    },
    Sticker {
        id: "2757",
        name: "Atención",
        png: include_bytes!("../stickers/2757.png"),
    },
    Sticker {
        id: "1f4ac",
        name: "Comentario",
        png: include_bytes!("../stickers/1f4ac.png"),
    },
    Sticker {
        id: "1f4a1",
        name: "Idea",
        png: include_bytes!("../stickers/1f4a1.png"),
    },
    Sticker {
        id: "1f4f8",
        name: "Foto",
        png: include_bytes!("../stickers/1f4f8.png"),
    },
    Sticker {
        id: "1f3a5",
        name: "Vídeo",
        png: include_bytes!("../stickers/1f3a5.png"),
    },
    Sticker {
        id: "1f3a4",
        name: "Micrófono",
        png: include_bytes!("../stickers/1f3a4.png"),
    },
    Sticker {
        id: "1f3a7",
        name: "Auriculares",
        png: include_bytes!("../stickers/1f3a7.png"),
    },
    Sticker {
        id: "1f4f1",
        name: "Móvil",
        png: include_bytes!("../stickers/1f4f1.png"),
    },
    Sticker {
        id: "1f4bb",
        name: "Portátil",
        png: include_bytes!("../stickers/1f4bb.png"),
    },
    Sticker {
        id: "1f3ae",
        name: "Gaming",
        png: include_bytes!("../stickers/1f3ae.png"),
    },
    Sticker {
        id: "1f3b5",
        name: "Música",
        png: include_bytes!("../stickers/1f3b5.png"),
    },
    Sticker {
        id: "1f514",
        name: "Notificación",
        png: include_bytes!("../stickers/1f514.png"),
    },
    Sticker {
        id: "1f381",
        name: "Regalo",
        png: include_bytes!("../stickers/1f381.png"),
    },
    Sticker {
        id: "1f680",
        name: "Cohete",
        png: include_bytes!("../stickers/1f680.png"),
    },
    Sticker {
        id: "1f451",
        name: "Corona",
        png: include_bytes!("../stickers/1f451.png"),
    },
    Sticker {
        id: "1f980",
        name: "Cangrejo",
        png: include_bytes!("../stickers/1f980.png"),
    },
    Sticker {
        id: "1f431",
        name: "Gato",
        png: include_bytes!("../stickers/1f431.png"),
    },
    Sticker {
        id: "1f436",
        name: "Perro",
        png: include_bytes!("../stickers/1f436.png"),
    },
    Sticker {
        id: "1f98b",
        name: "Mariposa",
        png: include_bytes!("../stickers/1f98b.png"),
    },
    Sticker {
        id: "1f308",
        name: "Arcoíris",
        png: include_bytes!("../stickers/1f308.png"),
    },
    Sticker {
        id: "2600",
        name: "Sol",
        png: include_bytes!("../stickers/2600.png"),
    },
    Sticker {
        id: "1f334",
        name: "Palmera",
        png: include_bytes!("../stickers/1f334.png"),
    },
    Sticker {
        id: "1f30d",
        name: "Mundo",
        png: include_bytes!("../stickers/1f30d.png"),
    },
    Sticker {
        id: "1f355",
        name: "Pizza",
        png: include_bytes!("../stickers/1f355.png"),
    },
    Sticker {
        id: "2615",
        name: "Café",
        png: include_bytes!("../stickers/2615.png"),
    },
    Sticker {
        id: "1f389",
        name: "Confeti",
        png: include_bytes!("../stickers/1f389.png"),
    },
    Sticker {
        id: "1f3c6",
        name: "Trofeo",
        png: include_bytes!("../stickers/1f3c6.png"),
    },
];
impl Studio {
    /// Imports a sticker into this project and places it above the picture.
    pub fn place_sticker(&mut self, id: &str, track_id: Option<String>, start: f64) {
        let Some(sticker) = ITEMS.iter().find(|item| item.id == id) else {
            return;
        };
        let folder = self.host.dirs.data.join("stickers");
        let path = folder.join(format!("twemoji-{}.png", sticker.id));
        let result = std::fs::create_dir_all(&folder).and_then(|()| {
            if path.is_file() {
                Ok(())
            } else {
                std::fs::write(&path, sticker.png)
            }
        });
        if let Err(error) = result {
            self.notify(&error.to_string(), true);
            return;
        }
        match self.adopt_library_file(sticker.name, &path, "stickers") {
            Ok(media_id) => {
                self.close_source();
                if let Some(id) = self.apply(Command::AddStickerClip {
                    media_id,
                    track_id,
                    start,
                }) {
                    self.selection = vec![id];
                }
            }
            Err(error) => self.notify(&error, true),
        }
    }
}
