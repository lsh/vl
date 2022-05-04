use clap::{Subcommand, Args};
use crate::parse_encodings;
#[derive(Debug, Args)]
pub struct Arc {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="bin-spacing")]
	pub bin_spacing: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="continuous-band-size")]
	pub continuous_band_size: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="discrete-band-size")]
	pub discrete_band_size: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Arc {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.bin_spacing {
			encodings.push(format!("\"binSpacing\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.continuous_band_size {
			encodings.push(format!("\"continuousBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.discrete_band_size {
			encodings.push(format!("\"discreteBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Area {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line")]
	pub line: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="point")]
	pub point: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Area {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line {
			encodings.push(format!("\"line\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.point {
			encodings.push(format!("\"point\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Bar {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="bin-spacing")]
	pub bin_spacing: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="continuous-band-size")]
	pub continuous_band_size: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-end")]
	pub corner_radius_end: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="discrete-band-size")]
	pub discrete_band_size: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Bar {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.bin_spacing {
			encodings.push(format!("\"binSpacing\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.continuous_band_size {
			encodings.push(format!("\"continuousBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_end {
			encodings.push(format!("\"cornerRadiusEnd\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.discrete_band_size {
			encodings.push(format!("\"discreteBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Image {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="bin-spacing")]
	pub bin_spacing: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="continuous-band-size")]
	pub continuous_band_size: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="discrete-band-size")]
	pub discrete_band_size: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Image {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.bin_spacing {
			encodings.push(format!("\"binSpacing\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.continuous_band_size {
			encodings.push(format!("\"continuousBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.discrete_band_size {
			encodings.push(format!("\"discreteBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Line {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="point")]
	pub point: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Line {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.point {
			encodings.push(format!("\"point\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Point {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Point {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Rect {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="bin-spacing")]
	pub bin_spacing: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="continuous-band-size")]
	pub continuous_band_size: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="discrete-band-size")]
	pub discrete_band_size: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Rect {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.bin_spacing {
			encodings.push(format!("\"binSpacing\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.continuous_band_size {
			encodings.push(format!("\"continuousBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.discrete_band_size {
			encodings.push(format!("\"discreteBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Rule {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Rule {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Text {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Text {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Tick {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="band-size")]
	pub band_size: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="thickness")]
	pub thickness: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Tick {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.band_size {
			encodings.push(format!("\"bandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.thickness {
			encodings.push(format!("\"thickness\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Trail {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="point")]
	pub point: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Trail {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.point {
			encodings.push(format!("\"point\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Circle {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Circle {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Square {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Square {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Debug, Args)]
pub struct Geoshape {
	#[clap(long="align")]
	pub align: Option<String>,
	#[clap(long="angle")]
	pub angle: Option<String>,
	#[clap(long="aria")]
	pub aria: Option<String>,
	#[clap(long="aria-role")]
	pub aria_role: Option<String>,
	#[clap(long="aria-role-description")]
	pub aria_role_description: Option<String>,
	#[clap(long="aspect")]
	pub aspect: Option<String>,
	#[clap(long="baseline")]
	pub baseline: Option<String>,
	#[clap(long="blend")]
	pub blend: Option<String>,
	#[clap(long="color")]
	pub color: Option<String>,
	#[clap(long="corner-radius")]
	pub corner_radius: Option<String>,
	#[clap(long="corner-radius-bottom-left")]
	pub corner_radius_bottom_left: Option<String>,
	#[clap(long="corner-radius-bottom-right")]
	pub corner_radius_bottom_right: Option<String>,
	#[clap(long="corner-radius-top-left")]
	pub corner_radius_top_left: Option<String>,
	#[clap(long="corner-radius-top-right")]
	pub corner_radius_top_right: Option<String>,
	#[clap(long="cursor")]
	pub cursor: Option<String>,
	#[clap(long="description")]
	pub description: Option<String>,
	#[clap(long="dir")]
	pub dir: Option<String>,
	#[clap(long="dx")]
	pub dx: Option<String>,
	#[clap(long="dy")]
	pub dy: Option<String>,
	#[clap(long="ellipsis")]
	pub ellipsis: Option<String>,
	#[clap(long="end-angle")]
	pub end_angle: Option<String>,
	#[clap(long="fill")]
	pub fill: Option<String>,
	#[clap(long="fill-opacity")]
	pub fill_opacity: Option<String>,
	#[clap(long="filled")]
	pub filled: Option<String>,
	#[clap(long="font")]
	pub font: Option<String>,
	#[clap(long="font-size")]
	pub font_size: Option<String>,
	#[clap(long="font-style")]
	pub font_style: Option<String>,
	#[clap(long="font-weight")]
	pub font_weight: Option<String>,
	#[clap(long="height")]
	pub height: Option<String>,
	#[clap(long="href")]
	pub href: Option<String>,
	#[clap(long="inner-radius")]
	pub inner_radius: Option<String>,
	#[clap(long="interpolate")]
	pub interpolate: Option<String>,
	#[clap(long="invalid")]
	pub invalid: Option<String>,
	#[clap(long="limit")]
	pub limit: Option<String>,
	#[clap(long="line-break")]
	pub line_break: Option<String>,
	#[clap(long="line-height")]
	pub line_height: Option<String>,
	#[clap(long="opacity")]
	pub opacity: Option<String>,
	#[clap(long="order")]
	pub order: Option<String>,
	#[clap(long="orient")]
	pub orient: Option<String>,
	#[clap(long="outer-radius")]
	pub outer_radius: Option<String>,
	#[clap(long="pad-angle")]
	pub pad_angle: Option<String>,
	#[clap(long="radius")]
	pub radius: Option<String>,
	#[clap(long="radius-2")]
	pub radius_2: Option<String>,
	#[clap(long="shape")]
	pub shape: Option<String>,
	#[clap(long="size")]
	pub size: Option<String>,
	#[clap(long="smooth")]
	pub smooth: Option<String>,
	#[clap(long="start-angle")]
	pub start_angle: Option<String>,
	#[clap(long="stroke")]
	pub stroke: Option<String>,
	#[clap(long="stroke-cap")]
	pub stroke_cap: Option<String>,
	#[clap(long="stroke-dash")]
	pub stroke_dash: Option<String>,
	#[clap(long="stroke-dash-offset")]
	pub stroke_dash_offset: Option<String>,
	#[clap(long="stroke-join")]
	pub stroke_join: Option<String>,
	#[clap(long="stroke-miter-limit")]
	pub stroke_miter_limit: Option<String>,
	#[clap(long="stroke-offset")]
	pub stroke_offset: Option<String>,
	#[clap(long="stroke-opacity")]
	pub stroke_opacity: Option<String>,
	#[clap(long="stroke-width")]
	pub stroke_width: Option<String>,
	#[clap(long="tension")]
	pub tension: Option<String>,
	#[clap(long="text")]
	pub text: Option<String>,
	#[clap(long="theta")]
	pub theta: Option<String>,
	#[clap(long="theta-2")]
	pub theta_2: Option<String>,
	#[clap(long="time-unit-band-position")]
	pub time_unit_band_position: Option<String>,
	#[clap(long="time-unit-band-size")]
	pub time_unit_band_size: Option<String>,
	#[clap(long="tooltip")]
	pub tooltip: Option<String>,
	#[clap(long="url")]
	pub url: Option<String>,
	#[clap(long="width")]
	pub width: Option<String>,
	#[clap(long="x")]
	pub x: Option<String>,
	#[clap(long="x-2")]
	pub x_2: Option<String>,
	#[clap(long="y")]
	pub y: Option<String>,
	#[clap(long="y-2")]
	pub y_2: Option<String>,
}
impl Geoshape {
	pub fn encode(&self) -> String {
		let mut encodings: Vec<String> = Vec::new();
		if let Some(encoding) = &self.align {
			encodings.push(format!("\"align\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.angle {
			encodings.push(format!("\"angle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria {
			encodings.push(format!("\"aria\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role {
			encodings.push(format!("\"ariaRole\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aria_role_description {
			encodings.push(format!("\"ariaRoleDescription\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.aspect {
			encodings.push(format!("\"aspect\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.baseline {
			encodings.push(format!("\"baseline\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.blend {
			encodings.push(format!("\"blend\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.color {
			encodings.push(format!("\"color\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius {
			encodings.push(format!("\"cornerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_left {
			encodings.push(format!("\"cornerRadiusBottomLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_bottom_right {
			encodings.push(format!("\"cornerRadiusBottomRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_left {
			encodings.push(format!("\"cornerRadiusTopLeft\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.corner_radius_top_right {
			encodings.push(format!("\"cornerRadiusTopRight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.cursor {
			encodings.push(format!("\"cursor\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.description {
			encodings.push(format!("\"description\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dir {
			encodings.push(format!("\"dir\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dx {
			encodings.push(format!("\"dx\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.dy {
			encodings.push(format!("\"dy\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.ellipsis {
			encodings.push(format!("\"ellipsis\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.end_angle {
			encodings.push(format!("\"endAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill {
			encodings.push(format!("\"fill\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.fill_opacity {
			encodings.push(format!("\"fillOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.filled {
			encodings.push(format!("\"filled\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font {
			encodings.push(format!("\"font\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_size {
			encodings.push(format!("\"fontSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_style {
			encodings.push(format!("\"fontStyle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.font_weight {
			encodings.push(format!("\"fontWeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.height {
			encodings.push(format!("\"height\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.href {
			encodings.push(format!("\"href\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.inner_radius {
			encodings.push(format!("\"innerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.interpolate {
			encodings.push(format!("\"interpolate\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.invalid {
			encodings.push(format!("\"invalid\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.limit {
			encodings.push(format!("\"limit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_break {
			encodings.push(format!("\"lineBreak\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.line_height {
			encodings.push(format!("\"lineHeight\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.opacity {
			encodings.push(format!("\"opacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.order {
			encodings.push(format!("\"order\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.orient {
			encodings.push(format!("\"orient\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.outer_radius {
			encodings.push(format!("\"outerRadius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.pad_angle {
			encodings.push(format!("\"padAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius {
			encodings.push(format!("\"radius\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.radius_2 {
			encodings.push(format!("\"radius2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.shape {
			encodings.push(format!("\"shape\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.size {
			encodings.push(format!("\"size\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.smooth {
			encodings.push(format!("\"smooth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.start_angle {
			encodings.push(format!("\"startAngle\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke {
			encodings.push(format!("\"stroke\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_cap {
			encodings.push(format!("\"strokeCap\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash {
			encodings.push(format!("\"strokeDash\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_dash_offset {
			encodings.push(format!("\"strokeDashOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_join {
			encodings.push(format!("\"strokeJoin\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_miter_limit {
			encodings.push(format!("\"strokeMiterLimit\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_offset {
			encodings.push(format!("\"strokeOffset\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_opacity {
			encodings.push(format!("\"strokeOpacity\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.stroke_width {
			encodings.push(format!("\"strokeWidth\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tension {
			encodings.push(format!("\"tension\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.text {
			encodings.push(format!("\"text\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta {
			encodings.push(format!("\"theta\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.theta_2 {
			encodings.push(format!("\"theta2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_position {
			encodings.push(format!("\"timeUnitBandPosition\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.time_unit_band_size {
			encodings.push(format!("\"timeUnitBandSize\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.tooltip {
			encodings.push(format!("\"tooltip\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.url {
			encodings.push(format!("\"url\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.width {
			encodings.push(format!("\"width\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x {
			encodings.push(format!("\"x\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.x_2 {
			encodings.push(format!("\"x2\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y {
			encodings.push(format!("\"y\": {{{}}}", parse_encodings(encoding)));
		}
		if let Some(encoding) = &self.y_2 {
			encodings.push(format!("\"y2\": {{{}}}", parse_encodings(encoding)));
		}
		format!("{{{}}}", encodings.join(", "))
	}
}
#[derive(Subcommand, Debug)]
pub enum Marks {Arc(Arc),
Area(Area),
Bar(Bar),
Image(Image),
Line(Line),
Point(Point),
Rect(Rect),
Rule(Rule),
Text(Text),
Tick(Tick),
Trail(Trail),
Circle(Circle),
Square(Square),
Geoshape(Geoshape),
}
