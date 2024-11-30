use ori::{core::text::FontAttributes, prelude::*};

use crate::{IconCode, IconFont};

/// Create a new [`Icon`].
pub fn icon(icon: impl Into<IconCode>) -> Icon {
    Icon::new(icon)
}

/// A view that displays a single icon.
///
/// By default, the icon is rendered using the `icon.font` font family.
/// This uses the [Font Awesome 6 Regular Free](https://fontawesome.com/) font by default.
#[derive(Styled, Build, Rebuild)]
pub struct Icon {
    /// The codepoint of the icon to display.
    #[rebuild(layout)]
    pub icon: IconCode,

    /// Whether the icon is solid or regular.
    ///
    /// This only affects the rendering of the icon if the icon is available in both styles.
    #[rebuild(layout)]
    pub solid: bool,

    /// The size of the icon.
    #[rebuild(layout)]
    #[styled(default = 16.0)]
    pub size: Styled<f32>,

    /// The color of the icon.
    #[rebuild(draw)]
    #[styled(default -> Theme::CONTRAST or Color::BLACK)]
    pub color: Styled<Color>,
}

impl Icon {
    /// Create a new icon view.
    pub fn new(icon: impl Into<IconCode>) -> Self {
        Self {
            icon: icon.into(),
            solid: false,
            size: IconStyle::SIZE.into(),
            color: IconStyle::COLOR.into(),
        }
    }

    /// Get the font to use for the icon.
    pub fn font(&self) -> IconFont {
        if self.icon.fonts().contains(&IconFont::Solid)
            && self.icon.fonts().contains(&IconFont::Regular)
        {
            if self.solid {
                return IconFont::Solid;
            } else {
                return IconFont::Regular;
            }
        }

        self.icon.fonts()[0]
    }
}

#[doc(hidden)]
pub struct IconState {
    style: IconStyle,
    paragraph: Paragraph,
}

impl<T> View<T> for Icon {
    type State = IconState;

    fn build(&mut self, cx: &mut BuildCx, _data: &mut T) -> Self::State {
        let style = IconStyle::styled(self, cx.styles());
        let mut paragraph = Paragraph::new(1.0, TextAlign::Center, TextWrap::None);

        paragraph.set_text(
            self.icon.as_str(),
            FontAttributes {
                size: style.size,
                family: self.font().family(),
                stretch: FontStretch::Normal,
                weight: FontWeight::NORMAL,
                style: FontStyle::Normal,
                color: style.color,
            },
        );

        struct FontsLoaded;
        if !cx.contains_context::<FontsLoaded>() {
            cx.fonts_mut().load(include_font!("font"));
            cx.insert_context(FontsLoaded);
        }

        IconState { style, paragraph }
    }

    fn rebuild(&mut self, state: &mut Self::State, cx: &mut RebuildCx, _data: &mut T, _old: &Self) {
        state.style.rebuild(self, cx);

        state.paragraph.set_text(
            self.icon.as_str(),
            FontAttributes {
                size: state.style.size,
                family: self.font().family(),
                stretch: FontStretch::Normal,
                weight: FontWeight::NORMAL,
                style: FontStyle::Normal,
                color: state.style.color,
            },
        );
    }

    fn event(
        &mut self,
        _state: &mut Self::State,
        _cx: &mut EventCx,
        _data: &mut T,
        _event: &Event,
    ) -> bool {
        false
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        _cx: &mut LayoutCx,
        _data: &mut T,
        space: Space,
    ) -> Size {
        space.fit(Size::all(state.style.size))
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawCx, _data: &mut T) {
        cx.text(&state.paragraph, cx.rect());
    }
}
