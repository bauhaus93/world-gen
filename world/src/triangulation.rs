use rand::{seq::SliceRandom, thread_rng};
use std::f32;

use core::graphics::{mesh, mesh::Vertex};
use core::{Point2f, Point3f};

pub fn triangulate(points: &[Point3f]) -> Option<Vec<mesh::Triangle>> {
    // TODO: maybe optimize search for correct 3d point / use 3d through whole algo

    let points_2d: Vec<Point2f> = points.iter().map(|p| p.as_xy()).collect();
    let triangulation = match triangulate_bowyer_watson(&points_2d) {
        Some(t) => t,
        None => return None,
    };

    let mut mesh_triangles = Vec::with_capacity(triangulation.len());
    for t in triangulation.iter() {
        let mut vertices: [Vertex; 3] = [Vertex::default(), Vertex::default(), Vertex::default()];
        for (i, tria_point) in t.get_points().iter().enumerate() {
            match points.iter().find(|p| p.as_xy() == *tria_point) {
                Some(p) => vertices[i].set_pos(*p),
                None => {
                    error!("Could not find triangle point in original list");
                    unreachable!();
                }
            }
        }
        let mut triangle = mesh::Triangle::new(vertices);
        triangle.force_ccw();
        triangle.update_triangle_normal();
        triangle.update_vertex_normals();
        mesh_triangles.push(triangle);
    }
    Some(mesh_triangles)
}

#[derive(Clone, Copy)]
struct Triangle {
    points: [Point2f; 3],
    edges: [Edge; 3],
    circumcenter: Point2f,
    radius: f32,
}

#[derive(Clone, Copy)]
struct Edge(pub Point2f, pub Point2f);

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Edge {}

impl Triangle {
    pub fn new(mut points: [Point2f; 3]) -> Option<Self> {
        let edges = [
            Edge(points[0], points[1]),
            Edge(points[1], points[2]),
            Edge(points[2], points[0]),
        ];
        let cc = match get_circumcenter(&points) {
            Some(cc) => cc,
            None => {
                return None;
            }
        };
        let radius = (points[0] - cc).length();
        Some(Self {
            points: points,
            edges: edges,
            circumcenter: cc,
            radius: radius,
        })
    }

    pub fn new_super(contained_points: &[Point2f]) -> Self {
        let mut size = 1000.;
        loop {
            let super_triangle = Triangle::new([
                contained_points[0] - Point2f::new(0., size),
                contained_points[0] + Point2f::new(-size, size),
                contained_points[0] + Point2f::new(size, size),
            ])
            .unwrap();
            if contained_points
                .iter()
                .all(|p| super_triangle.circumcircle_contains(p))
            {
                return super_triangle;
            }
            size *= 2.;
        }
    }

    pub fn has_super_point(&self, super_triangle: &Triangle) -> bool {
        self.points
            .iter()
            .any(|p| is_super_point(p, super_triangle))
    }

    pub fn circumcircle_contains(&self, point: &Point2f) -> bool {
        (*point - self.circumcenter).length() < self.radius
    }

    pub fn shares_edge(&self, other: &Triangle) -> bool {
        self.edges.iter().any(|e| other.edges.iter().any(|eo| e == eo))
    }

    pub fn contains_edge(&self, edge: &Edge) -> bool {
        self.edges.iter().any(|e| e == edge)
    }

    pub fn get_edges(&self) -> &[Edge; 3] {
        &self.edges
    }

    pub fn get_points(&self) -> &[Point2f; 3] {
        &self.points
    }
}

fn triangulate_bowyer_watson(points: &[Point2f]) -> Option<Vec<Triangle>> {
    let super_triangle = Triangle::new_super(&points);
    let mut triangulation: Vec<Triangle> = Vec::new();
    triangulation.push(super_triangle);

    for new_point in points.iter() {
        //println!("new point: {}", new_point);
        let (bad_triangles, good_triangles): (Vec<Triangle>, Vec<Triangle>) = triangulation
            .into_iter()
            .partition(|t| t.circumcircle_contains(&new_point));

        triangulation = good_triangles;

        let mut new_triangles = Vec::new();
        for (i, t) in bad_triangles.iter().enumerate() {
            for e in t.get_edges().iter() {
                // Check if edge is not shared with any other bad triangle
                if !bad_triangles
                    .iter()
                    .take(i)
                    .chain(bad_triangles.iter().skip(i + 1))
                    .any(|tc| tc.contains_edge(e))
                {
                    //println!("non-sharing edge: {} -> {}, p = {}", e.0, e.1, new_point);
                    match Triangle::new([e.0, e.1, *new_point]) {
                        Some(t) => {
                            new_triangles.push(t);
                        }
                        None => {
                            continue;
                            return None;
                        }
                    }
                }
            }
        }
        triangulation.append(&mut new_triangles);
    }
    Some(
        triangulation
            .into_iter()
            .filter(|t| !t.has_super_point(&super_triangle))
            .collect(),
    )
}

fn is_super_point(point: &Point2f, super_triangle: &Triangle) -> bool {
    super_triangle.get_points().iter().any(|sp| sp == point)
}

fn get_circumcenter(triangle: &[Point2f; 3]) -> Option<Point2f> {
    let dir_1 = (triangle[0] - triangle[1]) / 2.;
    let a_1 = triangle[1] + dir_1;
    let a_2 = a_1 + dir_1.rotate_ccw_90();

    let dir_2 = (triangle[0] - triangle[2]) / 2.;
    let b_1 = triangle[2] + dir_2;
    let b_2 = b_1 + dir_2.rotate_ccw_90();
    return intersect_lines([a_1, a_2], [b_1, b_2]);
}

// intesect lines given 2 points on each line
fn intersect_lines(a: [Point2f; 2], b: [Point2f; 2]) -> Option<Point2f> {
    let denom =
        (a[0][0] - a[1][0]) * (b[0][1] - b[1][1]) - (a[0][1] - a[1][1]) * (b[0][0] - b[1][0]);
    if denom.abs() >= f32::EPSILON {
        let fac_1 = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        let fac_2 = b[0][0] * b[1][1] - b[0][1] * b[1][0];
        let x = fac_1 * (b[0][0] - b[1][0]) - (a[0][0] - a[1][0]) * fac_2;
        let y = fac_1 * (b[0][1] - b[1][1]) - (a[0][1] - a[1][1]) * fac_2;
        Some(Point2f::new(x, y) / denom)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use test::{black_box, Bencher};

    const RANDOM_SEED: u64 = 9001;
    const BENCH_GRID_SIZE: usize = 16;

    #[test]
    fn test_line_intersection_ortho_lines() {
        let p = intersect_lines(
            [Point2f::new(0., 0.), Point2f::new(0., 1.)],
            [Point2f::new(1., 0.5), Point2f::new(2., 0.5)],
        );
        assert!(p.is_some());
        assert_eq!(Point2f::new(0., 0.5), p.unwrap());
    }

    #[test]
    fn test_line_intersection_same_center() {
        let p = intersect_lines(
            [Point2f::new(0., 0.), Point2f::new(1., 1.)],
            [Point2f::new(0., 0.), Point2f::new(1., -1.)],
        );
        assert!(p.is_some());
        assert_eq!(Point2f::new(0., 0.), p.unwrap());
    }

    #[test]
    fn test_line_intersection_parallel() {
        let p = intersect_lines(
            [Point2f::new(10., 10.), Point2f::new(10., 11.)],
            [Point2f::new(-20., -20.), Point2f::new(-20., -21.)],
        );
        assert!(p.is_none());
    }

    #[test]
    fn test_line_intersection_ident() {
        let p = intersect_lines(
            [Point2f::new(10., 2.), Point2f::new(15., 2.)],
            [Point2f::new(100., 2.), Point2f::new(-50., 2.)],
        );
        assert!(p.is_none());
    }

    #[test]
    fn test_line_intersection_one_point() {
        let p = intersect_lines(
            [Point2f::new(5., 5.), Point2f::new(5., 5.)],
            [Point2f::new(-20., -20.), Point2f::new(-20., -21.)],
        );
        assert!(p.is_none());
    }

    fn all_same_distance(points: &[Point2f], cc: Point2f) -> bool {
        if points.len() > 0 {
            let dist = (points[0] - cc).length();
            for p in points[1..].iter() {
                let diff = (*p - cc).length() - dist;
                if diff.abs() > 1e-3 {
                    println!("diff = {}", diff);
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_triangle_circumcenter_right_angled() {
        let a = Point2f::new(0., 0.);
        let b = Point2f::new(1., 0.);
        let c = Point2f::new(0., 2.);
        let cc = get_circumcenter(&[a, b, c]);
        assert!(cc.is_some());
        assert_eq!(Point2f::new(0.5, 1.), cc.unwrap());
        assert!(all_same_distance(&[a, b, c], cc.unwrap()));
    }

    #[test]
    fn test_triangle_circumcenter_same_sides() {
        let a = Point2f::new(0., 0.);
        let b = Point2f::new(1., 0.);
        let c = Point2f::new(0.5, 1.);
        let cc = get_circumcenter(&[a, b, c]);
        assert!(cc.is_some());
        assert_eq!(Point2f::new(0.5, 0.375), cc.unwrap());
        assert!(all_same_distance(&[a, b, c], cc.unwrap()));
    }

    #[test]
    fn test_triangle_circumcenter_random() {
        const RUN_COUNT: usize = 10000;
        let mut rng = SmallRng::seed_from_u64(RANDOM_SEED);
        for _ in 0..RUN_COUNT {
            let a = Point2f::new(rng.gen(), rng.gen());
            let b = Point2f::new(rng.gen(), rng.gen());
            let c = Point2f::new(rng.gen(), rng.gen());
            if intersect_lines([a, b], [a, c]).is_some() {
                let cc = get_circumcenter(&[a, b, c]);
                assert!(cc.is_some());
                assert!(all_same_distance(&[a, b, c], cc.unwrap()));
            }
        }
    }

    #[test]
    fn test_triangulation_quad() {
        let points = vec![
            Point2f::new(0., 0.),
            Point2f::new(1., 0.),
            Point2f::new(0., 1.),
            Point2f::new(1., 1.),
        ];
        let triangulation = triangulate_bowyer_watson(&points);
        assert!(triangulation.is_some());
        assert_eq!(2, triangulation.unwrap().len());
    }

    // May fail if points are too densely packed
    #[test]
    fn test_triangulation_random_check_points_in_triangles() {
        const RUN_COUNT: usize = 16;
        const VERTEX_MAX: usize = 128;
        let mut rng = SmallRng::seed_from_u64(RANDOM_SEED);
        for _ in 0..RUN_COUNT {
            let vertex_count: usize = rng.gen_range(1, VERTEX_MAX);
            let mut points = Vec::with_capacity(vertex_count);
            for _ in 0..vertex_count {
                points.push(Point2f::new(rng.gen_range(0., 64.), rng.gen_range(0., 64.)));
            }
            let triangulation = triangulate_bowyer_watson(&points);
            assert!(triangulation.is_some());
            for t in triangulation.unwrap() {
                points
                    .iter()
                    .filter(|p| !t.get_points().iter().any(|tp| tp == *p))
                    .filter(|p| t.circumcircle_contains(&p))
                    .for_each(|p| {
                        println!(
                            "Triangle [({}), ({}), ({})] contains other point: {}",
                            t.get_points()[0],
                            t.get_points()[1],
                            t.get_points()[2],
                            p
                        )
                    });

                assert!(!points
                    .iter()
                    .filter(|p| !t.get_points().iter().any(|tp| tp == *p))
                    .any(|p| t.circumcircle_contains(p)));
            }
        }
    }

    #[bench]
    fn triangulation_with_mesh_vertices(b: &mut Bencher) {
        let mut points = Vec::new();
        for y in 0..BENCH_GRID_SIZE {
            for x in 0..BENCH_GRID_SIZE {
                points.push(Point3f::new(x as f32, y as f32, 0.));
            }
        }

        b.iter(|| triangulate(&points));
    }

    #[bench]
    fn triangulation_bw_only(b: &mut Bencher) {
        let mut points = Vec::new();
        for y in 0..BENCH_GRID_SIZE {
            for x in 0..BENCH_GRID_SIZE {
                points.push(Point2f::new(x as f32, y as f32));
            }
        }
        b.iter(|| triangulate_bowyer_watson(&points));
    }
}
