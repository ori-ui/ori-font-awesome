use ori::prelude::*;

use crate::{IconCode, IconFont};

/// Create a new [`Icon`].
pub fn icon(icon: impl Into<IconCode>) -> Icon {
    Icon::new(icon)
}

/// The style of an [`Icon`].
#[derive(Rebuild)]
pub struct IconStyle {
    /// The size of the icon.
    #[rebuild(layout)]
    pub size: f32,

    /// The color of the icon.
    #[rebuild(draw)]
    pub color: Color,
}

impl Style for IconStyle {
    fn default_style() -> StyleBuilder<Self> {
        StyleBuilder::new(|theme: &Theme| Self {
            size: 16.0,
            color: theme.contrast,
        })
    }
}

/// A view that displays a single icon.
///
/// By default, the icon is rendered using the `icon.font` font family.
/// This uses the [Font Awesome 6 Regular Free](https://fontawesome.com/) font by default.
#[derive(Build, Rebuild)]
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
    pub size: Option<f32>,

    /// The color of the icon.
    pub color: Option<Color>,
}

impl Icon {
    /// Create a new icon view.
    pub fn new(icon: impl Into<IconCode>) -> Self {
        Self {
            icon: icon.into(),
            solid: false,
            size: None,
            color: None,
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

impl Stylable for Icon {
    type Style = IconStyle;

    fn style(&self, base: &Self::Style) -> Self::Style {
        IconStyle {
            size: self.size.unwrap_or(base.size),
            color: self.color.unwrap_or(base.color),
        }
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
        let style = self.style(cx.style());
        let mut paragraph = Paragraph::new(1.0, TextAlign::Start, TextWrap::None);

        paragraph.set_text(
            self.icon.code_point(),
            FontAttributes {
                size: style.size,
                family: self.font().family(),
                stretch: FontStretch::Normal,
                weight: self.font().weight(),
                style: FontStyle::Normal,
                ligatures: false,
                color: style.color,
            },
        );

        struct FontsLoaded;
        if !cx.contains_context::<FontsLoaded>() {
            cx.fonts().load(
                include_font!("font/Font Awesome 6 Free-Regular-400.otf"),
                Some("FA6 Regular"),
            );
            cx.fonts().load(
                include_font!("font/Font Awesome 6 Free-Solid-900.otf"),
                Some("FA6 Solid"),
            );
            cx.fonts().load(
                include_font!("font/Font Awesome 6 Brands-Regular-400.otf"),
                Some("FA6 Brands"),
            );
            cx.insert_context(FontsLoaded);
        }

        IconState { style, paragraph }
    }

    fn rebuild(&mut self, state: &mut Self::State, cx: &mut RebuildCx, _data: &mut T, old: &Self) {
        Rebuild::rebuild(self, cx, old);
        self.rebuild_style(cx, &mut state.style);

        state.paragraph.set_text(
            self.icon.as_str(),
            FontAttributes {
                size: state.style.size,
                family: self.font().family(),
                stretch: FontStretch::Normal,
                weight: FontWeight::NORMAL,
                style: FontStyle::Normal,
                ligatures: false,
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
        let size = cx.fonts().measure(&state.paragraph, f32::INFINITY);

        let offset = cx.size() / 2.0 - size / 2.0;
        let rect = Rect::min_size(offset.to_point(), Size::INFINITY);

        cx.paragraph(&state.paragraph, rect);
    }
}
