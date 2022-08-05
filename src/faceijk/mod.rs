mod constants;

pub use self::constants::*;
use crate::{
    constants::{M_SQRT3_2, NUM_HEX_VERTS, NUM_PENT_VERTS},
    is_resolution_classIII,
    vec2d::Vec2d,
    CellBoundary, CoordIJK, LatLng,
};

/// For hexagonal representation
pub struct Hexagon;
/// For pentagonal representation
pub struct Pentagon;

/// Face number and ijk coordinates on that face-centered coordinate.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct FaceIJK {
    pub face: i32,
    pub coord: CoordIJK,
}

impl FaceIJK {
    pub fn new(face: i32, coord: CoordIJK) -> Self {
        Self { face, coord }
    }

    /// Determines the center point in spherical coordinates of a cell given by
    /// a [`FaceIJK`] address at a specified resolution.
    pub fn to_geo(&self, resolution: usize) -> LatLng {
        Into::<Vec2d>::into(self.coord).to_geo(self.face, resolution, 0)
    }

    /// Generates the cell boundary in spherical coordinates for a pentagonal cell
    /// given by a FaceIJK address at a specified resolution.
    pub(crate) fn pentagon_to_cell_boundary(
        &self,
        resolution: usize,
        start: usize,
        length: usize,
    ) -> CellBoundary {
        // TODO: fix the mut malarky
        let mut res = resolution;
        let verts = self.pentagon_to_verts(&mut res);

        // If we're returning the entire loop, we need one more iteration in case
        // of a distortion vertex on the last edge
        let additional_iter = if length == NUM_PENT_VERTS { 1 } else { 0 };

        // convert each vertex to lat/lng
        // adjust the face of each vertex as appropriate and introduce
        // edge-crossing vertices as needed
        let mut out = CellBoundary::new();

        let mut vert = start;
        let last_fijk: FaceIJK = Default::default();
        while vert < start + length + additional_iter {
            let v = vert % NUM_PENT_VERTS;
            let fijk = verts[v];
            fijk.adjust_pentagon_vert_overage(resolution);

            // all Class III pentagon edges cross icosa edges
            // note that Class II pentagons have vertices on the edge,
            // not edge intersections
            if is_resolution_classIII(resolution) {
                // find hex2d of the two vertexes on the last face
                let tmp_fijk = self.clone();
                let orig_2d0: Vec2d = last_fijk.coord.into();
                let current_to_last_dir =
                    ADJACENT_FACE_DIR[tmp_fijk.face as usize][last_fijk.face as usize];
                let fijk_orient =
                    &FACE_NEIGHBORS[tmp_fijk.face as usize][current_to_last_dir as usize];

                tmp_fijk.face = fijk_orient.face;
                let mut ijk = &tmp_fijk.coord;

                // rotate and translate for adjacent face
                for i in 0..fijk_orient.ccw_rot60 {
                    ijk.rotate_60ccw();
                }

                let trans_vec = fijk_orient.translate;
                trans_vec.scale(UNIT_SCALE_BY_CII_RES[resolution] * 3);
                ijk = &(ijk + &trans_vec);
                ijk.normalize();

                let orig_2d1: Vec2d = ijk.into();

                // find the appropriate icosa face edge vertexes
                let max_dim = MAX_DIM_BY_CII_RES[resolution];
                let v0 = Vec2d::new(3. * max_dim as f64, 0.);
                let v1 = Vec2d::new(-1.5 * max_dim as f64, 3. * M_SQRT3_2 * max_dim as f64);
                let v2 = Vec2d::new(-1.5 * max_dim as f64, -3.0 * M_SQRT3_2 * max_dim as f64);

                let edge0: Vec2d;
                let edge1: Vec2d;

                match ADJACENT_FACE_DIR[tmp_fijk.face as usize][fijk.face as usize] {
                    IJ => {
                        edge0 = v0;
                        edge1 = v1;
                    }
                    JK => {
                        edge0 = v1;
                        edge1 = v2;
                    }
                    KI => {
                        edge0 = v2;
                        edge1 = v0;
                    }
                    _ => {
                        // TODO: fix this behavior
                        // Src uses `assert`
                        edge0 = Default::default();
                        edge1 = Default::default();
                    }
                }

                let inter = Vec2d::intersect(&orig_2d0, &orig_2d1, &edge0, &edge1);
                out.push_vert_unchecked(inter.to_geo(fijk.face, resolution, 1));
            }

            vert += 1;
        }

        out
    }

    /// Generates the cell boundary in spherical coordinates for a cell given by a
    /// [`FaceIJK`] of a specified resolution.
    pub fn to_cell_boundary(&self, resolution: usize, start: usize, length: usize) -> CellBoundary {
        let mut res = resolution;
        let verts = self.pentagon_to_verts(&mut res);

        // If we're returning the entire loop, we need one more iteration in case
        // of a distortion vertex on the last edge
        let additional_iter = if length == NUM_HEX_VERTS { 1 } else { 0 };

        // convert each vertex to lat/lng
        // adjust the face of each vertex as appropriate and introduce
        // edge-crossing vertices as needed
        let mut out = CellBoundary::new();
        let last_face = -1;
        let last_overage = Overage::NoOverage;
        for vert in start..start + length + additional_iter {
            let v = vert & NUM_HEX_VERTS;
            let fijk = verts[v];
            let pent_leading4 = 0;
            let overage = fijk.adjust_overage_cII(resolution, pent_leading4, 0);

            /*
            Check for edge-crossing. Each face of the underlying icosahedron is a
            different projection plane. So if an edge of the hexagon crosses an
            icosahedron edge, an additional vertex must be introduced at that
            intersection point. Then each half of the cell edge can be projected
            to geographic coordinates using the appropriate icosahedron face
            projection. Note that Class II cell edges have vertices on the face
            edge, with no edge line intersections.
            */
            if is_resolution_classIII(resolution)
                && vert > start
                && fijk.face != last_face
                && last_overage != Overage::FaceEdge
            {
                // find hex2d of the two vertexes on original face
                let last_v = (v + 5) % NUM_HEX_VERTS;
                let orig2d0: Vec2d = verts[last_v].coord.into();
                let orig2d1: Vec2d = verts[v].coord.into();

                // find the appropriate icosa face edge vertexes
                let max_dim = MAX_DIM_BY_CII_RES[resolution];
                let v0 = Vec2d::new(3. * max_dim as f64, 0.);
                let v1 = Vec2d::new(-1.5 * max_dim as f64, 3. * M_SQRT3_2 * max_dim as f64);
                let v2 = Vec2d::new(-1.5 * max_dim as f64, -3. * M_SQRT3_2 * max_dim as f64);

                let face2 = if last_face == self.face {
                    fijk.face
                } else {
                    last_face
                };

                let edge0: Vec2d;
                let edge1: Vec2d;
                // TODO: figure out the i32/usize conflict
                match ADJACENT_FACE_DIR[self.face as usize][face2 as usize] {
                    IJ => {
                        edge0 = v0;
                        edge1 = v1;
                    }
                    JK => {
                        edge0 = v1;
                        edge1 = v2;
                    }
                    KI => {
                        edge0 = v2;
                        edge1 = v0;
                    }
                }

                // find the intersection and add the lat/lng point to the result
                let inter = Vec2d::intersect(&orig2d0, &orig2d1, &edge0, &edge1);

                /*
                If a point of intersection occurs at a hexagon vertex, then each
                adjacent hexagon edge will lie completely on a single icosahedron
                face, and no additional vertex is required.
                */
                let is_intersection_at_vertex = orig2d0 == inter || orig2d1 == inter;
                if !is_intersection_at_vertex {
                    out.push_vert_unchecked(inter.to_geo(self.face, resolution, 1));
                }
            }

            if vert < start + NUM_HEX_VERTS {
                let vec: Vec2d = fijk.coord.into();
                out.push_vert_unchecked(vec.to_geo(fijk.face, resolution, 1));
            }

            last_face = fijk.face;
            last_overage = overage;
        }

        out
    }

    /// Get the vertices of a pentagon cell as substrate FaceIJK
    pub fn pentagon_to_verts(&self, resolution: &mut usize) -> [Self; NUM_PENT_VERTS] {
        let class3 = is_resolution_classIII(*resolution);

        // get the correct set of substrate vertices for this resolution
        let verts = if class3 {
            // the vertexes of an origin-centered pentagon in a Class III resolution on
            // a substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
            // vertices, and the 3r7r gets us to Class II. vertices listed ccw from the
            // i-axes
            [
                CoordIJK { i: 5, j: 4, k: 0 }, // 0
                CoordIJK { i: 1, j: 5, k: 0 }, // 1
                CoordIJK { i: 0, j: 5, k: 4 }, // 2
                CoordIJK { i: 0, j: 1, k: 5 }, // 3
                CoordIJK { i: 4, j: 0, k: 5 }, // 4
            ] // CIII
        } else {
            // the vertexes of an origin-centered pentagon in a Class II resolution on a
            // substrate grid with aperture sequence 33r. The aperture 3 gets us the
            // vertices, and the 3r gets us back to Class II.
            // vertices listed ccw from the i-axes
            [
                CoordIJK { i: 2, j: 1, k: 0 }, // 0
                CoordIJK { i: 1, j: 2, k: 0 }, // 1
                CoordIJK { i: 0, j: 2, k: 1 }, // 2
                CoordIJK { i: 0, j: 1, k: 2 }, // 3
                CoordIJK { i: 1, j: 0, k: 2 }, // 4
            ] // CII
        };

        // adjust the center point to be in an aperture 33r substrate grid
        // these should be composed for speed
        self.coord.down_aperture_3();
        self.coord.down_aperture_3r();

        // if res is Class III we need to add a cw aperture 7 to get to
        // icosahedral Class II
        if class3 {
            self.coord.down_aperture_7r();
            *resolution += 1;
        }

        let out: [FaceIJK; NUM_PENT_VERTS] = [
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];

        // The center point is now in the same substrate grid as the origin
        // cell vertices. Add the center point substate coordinates
        // to each vertex to translate the vertices to that cell.
        for i in 0..NUM_PENT_VERTS {
            out[i].face = self.face;
            out[i].coord = self.coord + verts[i];
            out[i].coord.normalize();
        }

        out
    }

    /// Get the vertices of a cell as substrate FaceIJK.
    pub fn to_verts(&self, resolution: &mut usize) -> [Self; NUM_HEX_VERTS] {
        let class3 = is_resolution_classIII(*resolution);

        // get the correct set of substrate vertices for this resolution
        let verts = if class3 {
            // the vertexes of an origin-centered pentagon in a Class III resolution on
            // a substrate grid with aperture sequence 33r7r. The aperture 3 gets us the
            // vertices, and the 3r7r gets us to Class II. vertices listed ccw from the
            // i-axes
            [
                CoordIJK { i: 5, j: 4, k: 0 }, // 0
                CoordIJK { i: 1, j: 5, k: 0 }, // 1
                CoordIJK { i: 0, j: 5, k: 4 }, // 2
                CoordIJK { i: 0, j: 1, k: 5 }, // 3
                CoordIJK { i: 4, j: 0, k: 5 }, // 4
                CoordIJK { i: 5, j: 0, k: 1 }, // 5
            ] // CIII
        } else {
            // the vertexes of an origin-centered pentagon in a Class II resolution on a
            // substrate grid with aperture sequence 33r. The aperture 3 gets us the
            // vertices, and the 3r gets us back to Class II.
            // vertices listed ccw from the i-axes
            [
                CoordIJK { i: 2, j: 1, k: 0 }, // 0
                CoordIJK { i: 1, j: 2, k: 0 }, // 1
                CoordIJK { i: 0, j: 2, k: 1 }, // 2
                CoordIJK { i: 0, j: 1, k: 2 }, // 3
                CoordIJK { i: 1, j: 0, k: 2 }, // 4
                CoordIJK { i: 2, j: 0, k: 1 }, // 5
            ] // CII
        };

        // adjust the center point to be in an aperture 33r substrate grid
        // these should be composed for speed
        self.coord.down_aperture_3();
        self.coord.down_aperture_3r();

        // if res is Class III we need to add a cw aperture 7 to get to
        // icosahedral Class II
        if class3 {
            self.coord.down_aperture_7r();
            *resolution += 1;
        }

        let out: [FaceIJK; NUM_HEX_VERTS] = [
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];

        // The center point is now in the same substrate grid as the origin
        // cell vertices. Add the center point substate coordinates
        // to each vertex to translate the vertices to that cell.
        for i in 0..NUM_PENT_VERTS {
            out[i].face = self.face;
            out[i].coord = self.coord + verts[i];
            out[i].coord.normalize();
        }

        out
    }

    /// Adjusts the FaceIJK for a pentagon vertex in a substrate grid
    /// so that the resulting cell is relative to the correct
    /// icosahedral face.
    fn adjust_pentagon_vert_overage(&mut self, resolution: usize) -> Overage {
        let pent_leading_4 = 0;
        let mut overage = self.adjust_overage_cII(resolution, 0, 1);
        while overage == Overage::NewFace {
            overage = self.adjust_overage_cII(resolution, pent_leading_4, 0);
        }

        overage
    }

    /// Adjusts a FaceIJK so that the resulting cell is
    /// relative to the correct icosahedral face.
    fn adjust_overage_cII(
        &mut self,
        resolution: usize,
        pent_leading_4: i32,
        substrate: i32,
    ) -> Overage {
        let mut overage = Overage::NoOverage;
        let mut coord = &self.coord;

        // get the maximum dimension value; scale if a substrate grid
        let mut max_dim = MAX_DIM_BY_CII_RES[resolution];
        if substrate > 0 {
            max_dim *= 3;
        }

        if substrate > 0 && coord.i + coord.j + coord.k == max_dim {
            overage = Overage::FaceEdge;
        } else if coord.i + coord.j + coord.k > max_dim {
            overage = Overage::NewFace;
            let fijk_orient: &FaceOrientIJK;

            if coord.k > 0 {
                if coord.j > 0 {
                    // jk "quadrant"
                    fijk_orient = &FACE_NEIGHBORS[self.face as usize][JK as usize];
                } else {
                    // ik "quadrant"
                    fijk_orient = &FACE_NEIGHBORS[self.face as usize][KI as usize];

                    // adjust for the pentagonal missing sequence
                    if pent_leading_4 > 0 {
                        let origin = &CoordIJK::new(max_dim, 0, 0);
                        // translate origin to center of pentagon
                        let tmp = coord - origin;
                        // rotate to adjust for the missing sequence
                        tmp.rotate_60cw();
                        // translate the origin back to the center of the triangle
                        let CoordIJK { i, j, k } = &tmp + coord;
                        coord = &(&tmp + coord);
                    }
                }
            } else {
                // ij "quadrant"
                fijk_orient = &FACE_NEIGHBORS[self.face as usize][IJ as usize];
            }

            self.face = fijk_orient.face;
            // rotate and translate for adjacent face
            for i in 0..fijk_orient.ccw_rot60 {
                coord.rotate_60ccw();
            }

            let trans = fijk_orient.translate;
            let unit_scale = UNIT_SCALE_BY_CII_RES[resolution];
            if substrate > 0 {
                unit_scale *= 3
            }

            trans.scale(unit_scale);
            coord = &(coord + &trans);
            coord.normalize();

            if substrate > 0 && coord.i + coord.j + coord.k == max_dim {
                overage = Overage::FaceEdge;
            }
        }

        overage
    }
}

/// Information to transform into an adjacent face IJK system.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FaceOrientIJK {
    pub face: i32,
    pub translate: CoordIJK,
    pub ccw_rot60: i32,
}

/// Digit representing overage type.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Overage {
    NoOverage,
    FaceEdge,
    NewFace,
}
