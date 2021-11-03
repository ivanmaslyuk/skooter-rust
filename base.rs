pub struct MyView {
    x: i32,
    y: i32,
}

trait ABC {
    fn abc123();
}


trait View {
    fn body(&mut self) -> Option<Box<dyn View>> {
        None
    }

    fn draw(&mut self, &canvas: skia_safe::Canvas, x: i32, y: i32, width: i32, height: i32) {
        let body = self.body();

        body.unwrap().draw(canvas, x, y, width, height);

        self.draw_children(canvas, x, y, width, height);
    }

    fn draw_children(&mut self, &canvas: skia_safe::Canvas, x: i32, y: i32, width: i32, height: i32) {
        for view in self.children {
            view.draw(canvas, x, y, width, height);
        }
    }
}

impl View for MyView {
    #[inline]
    fn body(&mut self) -> impl ABC {
        self.y.expect()
    }
}
