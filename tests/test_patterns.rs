#[cfg(test)]
mod tests {
    use new_rt::{
        Axis, Color, Light, Material, Matrix, Object, Pattern, Point, Shape, Sphere, Transform,
        Vector, World,
    };

    #[test]
    fn test_creating_stripe_pattern() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, false);

        assert_eq!(white.r, pattern.a.r);
        assert_eq!(white.g, pattern.a.g);
        assert_eq!(white.b, pattern.a.b);

        assert_eq!(black.r, pattern.b.r);
        assert_eq!(black.g, pattern.b.g);
        assert_eq!(black.b, pattern.b.b);
    }

    #[test]
    fn test_stripe_pattern_is_const_x() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, false);

        let point = Point::new(0.0, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(0.0, 1.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(0.0, 2.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);

        let point = Point::new(0.0, 0.0, 1.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(0.0, 0.0, 2.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);

        let point = Point::new(0.9, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
        let point = Point::new(1.0, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 0.);
        let point = Point::new(-0.1, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 0.);
        let point = Point::new(-1.1, 0.0, 0.0);
        let color1 = pattern.stripe_at(&point);
        assert_eq!(color1.r, 1.);
    }

    // ce test ne passe pas
    #[test]
    fn test_lighting_with_a_pattern_applied() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, false);

        let mut material = Material::new();
        material.pattern = Some(pattern);
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;

        let mut obj = Object::new(Shape::Sphere(Sphere::new(1.)));
        obj.material = material.clone();

        let eyev = Vector::new(0., 0., -1.);
        let normalv = Vector::new(0., 0., -1.);

        let light = Light {
            position: Point::new(0., 0., -10.),
            intensity: Color::new(1., 1., 1.),
        };

        let p1 = Point::new(0.9, 0., 0.);
        let c1 = World::lighting(&obj.clone(), &light, &p1, &eyev, &normalv, false);
        assert_eq!(c1.red(), 1.);
        assert_eq!(c1.green(), 1.);
        assert_eq!(c1.blue(), 1.);

        let p2 = Point::new(1.1, 0., 0.);
        let c2 = World::lighting(&obj, &light, &p2, &eyev, &normalv, false);
        assert_eq!(c2.red(), 0.);
        assert_eq!(c2.green(), 0.);
        assert_eq!(c2.blue(), 0.);
    }

    #[test]
    fn test_stripes_with_an_obj_transformation() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, false);
        let mut material = Material::new();
        material.pattern = Some(pattern);

        let mut obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(material.clone());

        obj.scale(Vector::new(2., 2., 2.));
        let color = obj
            .material
            .pattern
            .clone()
            .unwrap()
            .stripe_at_object(&obj, &Point::new(1.5, 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);
    }

    #[test]
    fn test_stripe_with_pattern_trans() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, false);
        let mut material = Material::new();
        material.pattern = Some(pattern);
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;

        let mut obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(material.clone());

        obj.scale(Vector::new(2., 2., 2.));
        let color = obj
            .material
            .pattern
            .clone()
            .unwrap()
            .stripe_at_object(&obj, &Point::new(1.5, 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);
    }

    #[test]
    fn test_stripe_with_both_obj_pattern_trans() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, false);
        let mut material = Material::new();
        material.pattern = Some(pattern);
        material.ambient = 1.;
        material.diffuse = 0.;
        material.specular = 0.;

        let mut obj = Object::new(Shape::Sphere(Sphere::new(1.))).material(material.clone());

        obj.scale(Vector::new(2., 2., 2.));
        let color = obj
            .material
            .pattern
            .clone()
            .unwrap()
            .stripe_at_object(&obj, &Point::new(1.5, 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);
    }

    #[test]
    fn test_assigning_transformation() {
        let white = Color::new(1., 1., 1.);
        let black = Color::new(0., 0., 0.);

        let mut pattern = Pattern::new(white, black, Axis::X, false);
        pattern.translate(Vector::new(1., 2., 3.));

        assert_eq!(
            pattern.local_to_world,
            Matrix::translation(Vector::new(1., 2., 3.))
        );
    }

    #[test]
    fn test_pattern_with_obj_transformation() {
        let mut sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        sphere.scale(Vector::new(2., 2., 2.));

        let white = Color::new(1., 1.5, 2.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(black, white, Axis::X, false);
        sphere.material.pattern = Some(pattern);
        let color = sphere
            .material
            .pattern
            .clone()
            .unwrap()
            .stripe_at_object(&sphere, &Point::new(2., 3., 4.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.5);
        assert_eq!(color.b, 2.);
    }

    #[test]
    fn test_pattern_with_pattern_transformation() {
        let mut sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        let white = Color::new(1., 1.5, 2.);
        let black = Color::new(0., 0., 0.);

        let mut pattern = Pattern::new(black, white, Axis::X, false);
        pattern.scale(Vector::new(2., 2., 2.));
        sphere.material.pattern = Some(pattern);
        let color = sphere
            .material
            .pattern
            .clone()
            .unwrap()
            .stripe_at_object(&sphere, &Point::new(2., 3., 4.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.5);
        assert_eq!(color.b, 2.);
    }

    #[test]
    fn test_pattern_with_both_obj_pattern_transformation() {
        let mut sphere = Object::new(Shape::Sphere(Sphere::new(1.)));

        sphere.scale(Vector::new(2., 2., 2.));

        let white = Color::new(1., 1.5, 2.);
        let black = Color::new(0.75, 0.5, 0.25);

        let mut pattern = Pattern::new(black, white, Axis::X, false);
        pattern.translate(Vector::new(0.5, 1., 1.5));
        sphere.material.pattern = Some(pattern);
        let color = sphere
            .material
            .pattern
            .clone()
            .unwrap()
            .stripe_at_object(&sphere, &Point::new(2.5, 3., 3.5));
        assert_eq!(color.r, 0.75);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 0.25);
    }

    #[test]
    fn test_gradiant_linearly_interpolate_between_colors() {
        let white = Color::new(1., 1.0, 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::X, true);

        let mut color = pattern.pattern_at(&Point::new(0.25, 0., 0.));
        assert_eq!(color.r, 0.75);
        assert_eq!(color.g, 0.75);
        assert_eq!(color.b, 0.75);

        color = pattern.pattern_at(&Point::new(0.5, 0., 0.));
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.5);
        assert_eq!(color.b, 0.5);

        color = pattern.pattern_at(&Point::new(0.75, 0., 0.));
        assert_eq!(color.r, 0.25);
        assert_eq!(color.g, 0.25);
        assert_eq!(color.b, 0.25);
    }

    #[test]
    fn test_ring_should_extend_both_x_y() {
        let white = Color::new(1., 1.0, 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::XZ, false);

        let mut color = pattern.stripe_at(&Point::new(0., 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(0., 0., 1.));
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);

        color = pattern.stripe_at(&Point::new(1., 0., 0.));
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);
    }

    #[test]
    fn test_check_should_repeat_x() {
        let white = Color::new(1., 1.0, 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::XYZ, false);

        let mut color = pattern.stripe_at(&Point::new(0., 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(0.99, 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(1.01, 0., 0.));
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);
    }

    #[test]
    fn test_check_should_repeat_y() {
        let white = Color::new(1., 1.0, 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::XYZ, false);

        let mut color = pattern.stripe_at(&Point::new(0., 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(0., 0.99, 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(0., 1.01, 0.));
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);
    }

    #[test]
    fn test_check_should_repeat_z() {
        let white = Color::new(1., 1.0, 1.);
        let black = Color::new(0., 0., 0.);

        let pattern = Pattern::new(white, black, Axis::XYZ, false);

        let mut color = pattern.stripe_at(&Point::new(0., 0., 0.));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(0., 0., 0.99));
        assert_eq!(color.r, 1.);
        assert_eq!(color.g, 1.);
        assert_eq!(color.b, 1.);

        color = pattern.stripe_at(&Point::new(0., 0., 1.01));
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);
    }
}
