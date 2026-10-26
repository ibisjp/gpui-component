// Copyright (c) 2026 ibis inc.
// Licensed under the Apache License, Version 2.0. See LICENSE-APACHE and LICENSE-2.0.
//! Geometry for a cascade that avoids covering its parent until explicitly opened.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct SubmenuPlacement {
    pub left: f32,
    pub width: f32,
    pub opens_left: bool,
    pub explicit_only: bool,
}

pub(super) fn place_submenu(
    parent_left: f32,
    parent_right: f32,
    child_width: f32,
    viewport_width: f32,
    edge_margin: f32,
    gap: f32,
) -> SubmenuPlacement {
    let edge_margin = edge_margin.max(0.).min(viewport_width.max(0.) / 2.);
    let available = (viewport_width - 2. * edge_margin).max(0.);
    let width = child_width.max(0.).min(available);
    let right_limit = (viewport_width - edge_margin).max(edge_margin);
    let right = parent_right + gap;
    let left = parent_left - gap - width;
    if right + width <= right_limit {
        SubmenuPlacement {
            left: right,
            width,
            opens_left: false,
            explicit_only: false,
        }
    } else if left >= edge_margin {
        SubmenuPlacement {
            left,
            width,
            opens_left: true,
            explicit_only: false,
        }
    } else {
        SubmenuPlacement {
            left: right
                .max(edge_margin)
                .min((right_limit - width).max(edge_margin)),
            width,
            opens_left: false,
            explicit_only: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_right_without_covering_parent_when_it_fits() {
        let p = place_submenu(100., 300., 250., 1000., 4., 4.);
        assert_eq!(
            p,
            SubmenuPlacement {
                left: 304.,
                width: 250.,
                opens_left: false,
                explicit_only: false
            }
        );
    }
    #[test]
    fn opens_left_when_right_does_not_fit() {
        let p = place_submenu(720., 980., 400., 1000., 4., 4.);
        assert_eq!(p.left, 316.);
        assert!(p.opens_left && !p.explicit_only);
        assert!(p.left + p.width < 720., "the parent row remains visible");
    }
    #[test]
    fn child_width_not_parent_width_determines_side() {
        let narrow = place_submenu(650., 850., 100., 1000., 4., 4.);
        let wide = place_submenu(650., 850., 300., 1000., 4., 4.);
        assert!(!narrow.opens_left);
        assert!(wide.opens_left);
    }
    #[test]
    fn neither_side_fits_requires_explicit_open_and_clamps_right_origin() {
        let p = place_submenu(200., 500., 400., 800., 4., 4.);
        assert!(p.explicit_only && !p.opens_left);
        assert_eq!(p.left, 396.);
        assert_eq!(p.left + p.width, 796.);
    }
    #[test]
    fn menu_wider_than_viewport_is_narrowed() {
        let p = place_submenu(100., 250., 600., 320., 4., 4.);
        assert!(p.explicit_only);
        assert_eq!(p.left, 4.);
        assert_eq!(p.width, 312.);
    }
    #[test]
    fn zero_sized_viewport_has_a_valid_empty_plan() {
        let p = place_submenu(0., 0., 600., 0., 4., 4.);
        assert_eq!(p.width, 0.);
        assert_eq!(p.left, 0.);
        assert!(p.explicit_only);
    }
}
