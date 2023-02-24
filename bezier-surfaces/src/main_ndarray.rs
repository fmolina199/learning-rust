use ndarray::{array, Array1, Array2};
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use std::thread;

struct Const {
	barycentric: Array2<f32>,
}

impl Const {
	fn new() -> Const {
		Const {
			barycentric: array![
				[0.0, 0.0, 1.0],
				[0.0, 0.1, 0.9],
				[0.0, 0.2, 0.8],
				[0.0, 0.3, 0.7],
				[0.0, 0.4, 0.6],
				[0.0, 0.5, 0.5],
				[0.0, 0.6, 0.4],
				[0.0, 0.7, 0.3],
				[0.0, 0.8, 0.2],
				[0.0, 0.9, 0.1],
				[0.0, 1.0, 0.0],
				[0.1, 0.0, 0.9],
				[0.1, 0.1, 0.8],
				[0.1, 0.2, 0.7],
				[0.1, 0.3, 0.6],
				[0.1, 0.4, 0.5],
				[0.1, 0.5, 0.4],
				[0.1, 0.6, 0.3],
				[0.1, 0.7, 0.2],
				[0.1, 0.8, 0.1],
				[0.1, 0.9, 0.0],
				[0.2, 0.0, 0.8],
				[0.2, 0.1, 0.7],
				[0.2, 0.2, 0.6],
				[0.2, 0.3, 0.5],
				[0.2, 0.4, 0.4],
				[0.2, 0.5, 0.3],
				[0.2, 0.6, 0.2],
				[0.2, 0.7, 0.1],
				[0.2, 0.8, 0.0],
				[0.3, 0.0, 0.7],
				[0.3, 0.1, 0.6],
				[0.3, 0.2, 0.5],
				[0.3, 0.3, 0.4],
				[0.3, 0.4, 0.3],
				[0.3, 0.5, 0.2],
				[0.3, 0.6, 0.1],
				[0.3, 0.7, 0.0],
				[0.4, 0.0, 0.6],
				[0.4, 0.1, 0.5],
				[0.4, 0.2, 0.4],
				[0.4, 0.3, 0.3],
				[0.4, 0.4, 0.2],
				[0.4, 0.5, 0.1],
				[0.4, 0.6, 0.0],
				[0.5, 0.0, 0.5],
				[0.5, 0.1, 0.4],
				[0.5, 0.2, 0.3],
				[0.5, 0.3, 0.2],
				[0.5, 0.4, 0.1],
				[0.5, 0.5, 0.0],
				[0.6, 0.0, 0.4],
				[0.6, 0.1, 0.3],
				[0.6, 0.2, 0.2],
				[0.6, 0.3, 0.1],
				[0.6, 0.4, 0.0],
				[0.7, 0.0, 0.3],
				[0.7, 0.1, 0.2],
				[0.7, 0.2, 0.1],
				[0.7, 0.3, 0.0],
				[0.8, 0.0, 0.2],
				[0.8, 0.1, 0.1],
				[0.8, 0.2, 0.0],
				[0.9, 0.0, 0.1],
				[0.9, 0.1, 0.0],
				[1.0, 0.0, 0.0]
			],
		}
	}

	fn get_barycentric(&self) -> &Array2<f32> {
		&self.barycentric
	}
}

struct SingConst {
	singleton: Option<Const>,
}

impl SingConst {
	fn get_instance(&mut self) -> &Const {
		if self.singleton.is_none() {
			self.singleton = Some(Const::new());
		}
		self.singleton.as_ref().unwrap()
	}
}

static mut BARYCENTRIC: SingConst = SingConst { singleton: None };

struct MyWindowHandler {
	triangle: Vec<Array1<f32>>,
}

impl WindowHandler for MyWindowHandler {
	fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
		graphics.clear_screen(Color::from_rgb(1.0, 1.0, 1.0));
		let b = unsafe { BARYCENTRIC.get_instance().get_barycentric() };
		let shape = b.shape();
		for i in 0..shape[0] {
			let s_1 = b[[i, 0]];
			let s_2 = s_1 * s_1;
			let s_3 = s_2 * s_1;

			let t_1 = b[[i, 1]];
			let t_2 = t_1 * t_1;
			let t_3 = t_2 * t_1;

			let u_1 = b[[i, 2]];
			let u_2 = u_1 * u_1;
			let u_3 = u_2 * u_1;

			let p = &self.triangle[3] * t_3
				+ &self.triangle[2] * 3.0 * s_1 * t_2
				+ &self.triangle[6] * 3.0 * t_2 * u_1
				+ &self.triangle[1] * 3.0 * s_2 * t_1
				+ &self.triangle[5] * 6.0 * s_1 * t_1 * u_1
				+ &self.triangle[8] * 3.0 * t_1 * u_2
				+ &self.triangle[0] * s_3
				+ &self.triangle[9] * u_3
				+ &self.triangle[4] * 3.0 * s_2 * u_1
				+ &self.triangle[7] * 3.0 * s_1 * u_2;

			graphics.draw_circle((p[0] * 10.0 + 15.0, p[1] * 10.0 + 15.0), 5.0, Color::BLUE);
			graphics.draw_circle((p[1] * 10.0 + 500.0, p[2] * 10.0 + 500.0), 5.0, Color::RED);
		}
		helper.request_redraw();
	}
}

fn main() {
	let a = array![
		[1.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000],
		[0.500, 0.500, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000],
		[0.250, 0.500, 0.250, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000],
		[0.125, 0.375, 0.375, 0.125, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000],
		[0.000, 0.000, 0.000, 0.000, 1.000, 0.000, 0.000, 0.000, 0.000, 0.000],
		[0.000, 0.000, 0.000, 0.000, 0.500, 0.500, 0.000, 0.000, 0.000, 0.000],
		[0.000, 0.000, 0.000, 0.000, 0.250, 0.500, 0.250, 0.000, 0.000, 0.000],
		[0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 1.000, 0.000, 0.000],
		[0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.500, 0.500, 0.000],
		[0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 0.000, 1.000]
	];

	let window = Window::new_centered("Title", (1000, 1000)).unwrap();
	window.run_loop(MyWindowHandler {
		triangle: vec![
			array![00.0, 00.0, 00.0],
			array![00.0, 05.0, 00.0],
			array![00.0, 10.0, 00.0],
			array![00.0, 15.0, 00.0],
			array![05.0, 00.0, 00.0],
			array![07.5, 07.5, 150.0],
			array![05.0, 10.0, 00.0],
			array![10.0, 00.0, 00.0],
			array![10.0, 05.0, 00.0],
			array![15.0, 00.0, 00.0],
		],
	});
}
