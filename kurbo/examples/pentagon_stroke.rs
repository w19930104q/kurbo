// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Pentagon stroke example demonstrating stroke-width invariance
//! under scaling and mirroring.
//!
//! Scenario: a regular pentagon is stroked with a fixed width under
//! three conditions — identity, uniform scale ×2, and mirror across Y.
//! The stroke width remains 8.0 in all cases; only the geometry changes.
//!
//! The miter-join stroke outline should have consistent visual weight
//! regardless of transformation, i.e. the norm offset at each vertex
//! stays |0.5 × width| in local coordinates even when tangents change
//! magnitude under scaling or sign under mirroring.

use std::f64::consts::PI;

use kurbo::{Affine, BezPath, Cap, Join, Stroke, StrokeOpts, stroke};

fn pentagon(cx: f64, cy: f64, r: f64) -> BezPath {
    let n = 5;
    let mut path = BezPath::new();
    for i in 0..n {
        let angle = 2.0 * PI * i as f64 / n as f64 - PI / 2.0;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        if i == 0 {
            path.move_to((x, y));
        } else {
            path.line_to((x, y));
        }
    }
    path.close_path();
    path
}

fn main() {
    let tolerance = 0.25;
    let stroke_width = 8.0;
    let r = 80.0;

    let style = Stroke::new(stroke_width)
        .with_join(Join::Miter)
        .with_caps(Cap::Butt);
    let opts = StrokeOpts::default();

    // --- Identity ---
    let path_id = pentagon(120.0, 150.0, r);
    let stroked_id = stroke(path_id, &style, &opts, tolerance);

    // --- Scale ×2: path coordinates are doubled, stroke width stays 8 ---
    let path_scaled = Affine::scale(2.0) * pentagon(120.0, 150.0, r);
    let stroked_scaled = stroke(path_scaled, &style, &opts, tolerance);
    // Scale stroked output down to match display size
    let stroked_scaled_disp = Affine::translate((220.0, 0.0))
        * Affine::scale(0.5)
        * stroked_scaled;

    // --- Mirror Y: path reflected, stroke width stays 8 ---
    let path_mirror =
        Affine::new([-1.0, 0.0, 0.0, 1.0, 0.0, 0.0]) * pentagon(120.0, 150.0, r);
    let stroked_mirror = stroke(path_mirror, &style, &opts, tolerance);
    let stroked_mirror_disp = Affine::translate((440.0, 0.0)) * stroked_mirror;

    println!(
        "<svg width='660' height='300' xmlns='http://www.w3.org/2000/svg'>"
    );
    println!("  <rect width='660' height='300' fill='#f8f8f8'/>");

    // Labels
    for (x, label) in [(120, "Identity"), (340, "Scale ×2"), (560, "Mirror Y")] {
        println!(
            "  <text x='{x}' y='25' text-anchor='middle' font-size='14' fill='#333'>{label}</text>"
        );
    }

    // Stroked fills
    let fills = [
        (stroked_id.to_svg(), "#4ecdc4", "#1a535c"),
        (stroked_scaled_disp.to_svg(), "#ff6b6b", "#c0392b"),
        (stroked_mirror_disp.to_svg(), "#45b7d1", "#2980b9"),
    ];
    for (d, fill, stroke_color) in &fills {
        println!(
            "  <path d='{d}' fill='{fill}' stroke='{stroke_color}' stroke-width='1' fill-opacity='0.6'/>"
        );
    }

    // Dashed pentagon outlines for reference
    let ref_style = "fill='none' stroke='#333' stroke-width='1.5' stroke-dasharray='4,4'";
    let pent_id = pentagon(120.0, 150.0, r).to_svg();
    let pent_scaled = (Affine::translate((220.0, 0.0))
        * Affine::scale(0.5)
        * Affine::scale(2.0)
        * pentagon(120.0, 150.0, r))
    .to_svg();
    let pent_mirror = (Affine::translate((440.0, 0.0))
        * Affine::new([-1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
        * pentagon(120.0, 150.0, r))
    .to_svg();

    println!("  <path d='{pent_id}' {ref_style}/>");
    println!("  <path d='{pent_scaled}' {ref_style}/>");
    println!("  <path d='{pent_mirror}' {ref_style}/>");

    // Stroke-width annotation: a short perpendicular line segment showing width
    for (cx, cy, _dx, dy) in [
        (120.0, 230.0, 0.0, stroke_width),
        (340.0, 230.0, 0.0, stroke_width),
        (560.0, 230.0, 0.0, stroke_width),
    ] {
        println!(
            "  <line x1='{cx}' y1='{cy}' x2='{cx}' y2='{}' stroke='#e74c3c' stroke-width='2'/>",
            cy + dy
        );
        println!(
            "  <text x='{cx}' y='{}' text-anchor='middle' font-size='11' fill='#e74c3c'>w={stroke_width}</text>",
            cy + dy + 14.0
        );
    }

    println!("</svg>");
}
