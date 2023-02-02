#[doc(hidden)]
#[macro_export]
macro_rules! __impl_planar_ops {
    ($s:ident, $([$f:ident, $i:tt, $t:tt]),*) => {
        impl<F: $crate::Float> $s<F> {
            /// Creates new from individual components.
            #[inline]
            pub const fn new($($f: $t),*) -> Self {
                Self {
                    $($f),*
                }
            }

            /// Creates new with all components equal to `val`.
            #[inline]
            pub const fn same(val: F) -> Self {
                Self {
                    $($f: val),*
                }
            }

            /// Splits into components.
            #[inline]
            pub const fn into_parts(self) -> ($($t),*) {
                ($(self.$f),*)
            }

            /// Creates new from components.
            #[inline]
            pub const fn from_parts(parts: ($($t),*)) -> Self {
                Self {
                    $(
                        $f: parts.$i
                    ),*
                }
            }

            /// Multiplies all components by `factor`.
            #[inline]
            pub fn scale(&mut self, factor: F) {
                $(
                    self.$f *= factor;
                )*
            }

            /// Returns the scaled copy. See [`Self::scale`].
            #[inline]
            pub fn scaled(self, factor: F) -> Self {
                Self {
                    $(
                        $f: self.$f * factor
                    ),*
                }
            }

            /// Returns squared magnitude.
            #[inline]
            pub fn sqr_magnitude(&self) -> F {
                $(self.$f * self.$f +)* F::ZERO
            }

            /// Returns magnitude.
            #[inline]
            pub fn magnitude(&self) -> F {
                self.sqr_magnitude().sqrt()
            }

            /// Returns maximum component.
            #[inline]
            pub fn max(&self, other: Self) -> Self {
                Self {
                    $(
                        $f: self.$f.max(other.$f)
                    ),*
                }
            }

            /// Returns minimum component.
            #[inline]
            pub fn min(&self, other: Self) -> Self {
                Self {
                    $(
                        $f: self.$f.min(other.$f)
                    ),*
                }
            }

            // Returns a copy where all components are clamped between `from` and `to`.
            // #[inline]
            // pub fn clamp(&self, from: F, to: F) -> Self {
            //     Self {
            //         $(
            //             $f: self.$f.clamp(from, to)
            //         ),*
            //     }
            // }

            /// Returns a copy where all components are posivive.
            #[inline]
            pub fn abs(self) -> Self {
                Self {
                    $(
                        $f: self.$f.abs()
                    ),*
                }
            }

            /// Checks is zero with regard to `EPSILON`.
            #[inline]
            pub fn is_zero(&self) -> bool {
                $(
                    self.$f.abs() < F::EPSILON &&
                )* true
            }
        }

        impl<F: Float> core::convert::From<($($t),*)> for $s<F> {
            #[inline]
            fn from(val: ($($t),*)) -> Self {
                Self {
                    $(
                        $f: val.$i
                    ),*
                }
            }
        }

        impl<F: Float> core::convert::From<$s<F>> for ($($t),*) {
            #[inline]
            fn from(val: $s<F>) -> Self {
                ($(
                    val.$f
                ),*)
            }
        }

        impl<F: Float> core::ops::Add for $s<F> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self {
                    $(
                        $f: self.$f + rhs.$f
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::AddAssign for $s<F> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$f = self.$f + rhs.$f
                );*
            }
        }

        impl<F: Float> core::ops::Sub for $s<F> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                Self {
                    $(
                        $f: self.$f - rhs.$f
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::SubAssign for $s<F> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$f = self.$f - rhs.$f
                );*
            }
        }

        impl<F: Float> core::ops::Mul<F> for $s<F> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: F) -> Self {
                Self {
                    $(
                        $f: self.$f * rhs
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::MulAssign<F> for $s<F> {
            #[inline]
            fn mul_assign(&mut self, rhs: F) {
                $(
                    self.$f = self.$f * rhs
                );*
            }
        }

        impl<F: Float> core::ops::Div<F> for $s<F> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: F) -> Self {
                Self {
                    $(
                        $f: self.$f / rhs
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::DivAssign<F> for $s<F> {
            #[inline]
            fn div_assign(&mut self, rhs: F) {
                $(
                    self.$f = self.$f / rhs
                );*
            }
        }

        impl<F: Float> core::ops::Neg for $s<F> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    $(
                        $f: -self.$f
                    ),*
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_mat_ops {
    ($mat:ident, $rowtype:ident, $dim:expr, $($r:ident),*) => {
        impl<F: Float> $mat<F> {
            /// Inverses matrix in place.
            /// # Panics
            /// If the determinant is `0`.
            pub fn inverse(&mut self) {
                assert!(self.det().abs() > F::EPSILON, "Determinant is 0");

                let mut i = Self::IDENTITY;
                self.row_echelon_reduced(&mut i);
                *self = i;
            }

            /// Returns inversed matrix.
            /// # Panics
            /// If the determinant is `0`.
            pub fn inversed(&self) -> Self {
                assert!(self.det().abs() > F::EPSILON, "Determinant is 0");

                let mut i = Self::IDENTITY;
                self.clone().row_echelon_reduced(&mut i);
                i
            }

            /// Inverses matrix in place without checking if the determinant is `0`.
            pub fn inverse_unchecked(&mut self) {
                let mut i = Self::IDENTITY;
                self.row_echelon_reduced(&mut i);
                *self = i;
            }

            /// Returns inversed matrix without checking if the determinant is `0`.
            pub fn inversed_unchecked(&self) -> Self {
                let mut i = Self::IDENTITY;
                self.clone().row_echelon_reduced(&mut i);
                i
            }

            /// Computes the rank of the matrix using gaussian elimination.
            pub fn rank(&self) -> usize {
                let mut copy = self.clone();
                copy.to_row_echelon();

                let mut rank = 0;

                for d in 0..$dim {
                    if !copy[d].is_zero() {
                        rank += 1;
                    }
                }

                rank
            }


            /// Swaps two rows in place.
            pub fn swap_rows(&mut self, i: usize, j: usize) {
                assert!(
                    (1..=$dim).contains(&i) && (1..=$dim).contains(&j) && i != j,
                    "Invalid row index specified or i == j. I: {i}, J: {j}"
                );

                std::mem::swap(
                    unsafe { &mut *(self as *mut _ as *mut $rowtype<F>).add(i - 1) },
                    unsafe { &mut *(self as *mut _ as *mut $rowtype<F>).add(j - 1) },
                );
            }

            /// Turns matrix to its row echelon form using gaussian elimination.
            pub fn to_row_echelon(&mut self) {
                let (mut h, mut k) = (0, 0);

                while h < $dim && k < $dim {
                    let i_max = self.column(k + 1).abs().max_index().max(h);

                    if self[i_max][k].abs() < F::EPSILON {
                        k += 1;
                    } else {
                        if h != i_max {
                            self.swap_rows(h + 1, i_max + 1);
                        }

                        for i in (h + 1)..$dim {
                            let f = self[i][k] / self[h][k];

                            self[i][k] = F::ZERO;

                            for j in (k + 1)..$dim {
                                let delta = self[h][j] * f;
                                self[i][j] -= delta;
                            }
                        }

                        h += 1;
                        k += 1;
                    }
                }
            }

            fn row_echelon_reduced(&mut self, adjacent: &mut Self) {
                let mut lead = 0;

                for r in 0..$dim {
                    if $dim <= lead {
                        return;
                    }

                    let mut i = r;
                    while self[i][lead] == F::ZERO {
                        i += 1;

                        if $dim == i {
                            i = r;
                            lead += 1;
                            if $dim == lead {
                                return;
                            }
                        }
                    }

                    if i != r {
                        self.swap_rows(i + 1, r + 1);
                        adjacent.swap_rows(i + 1, r + 1);
                    }

                    let f = self[r][lead];
                    self[r] /= f;

                    adjacent[r] /= f;

                    for j in 0..$dim {
                        if j != r {
                            let f = self[j][lead];

                            let row = self[r];
                            self[j] -= row * f;

                            let row = adjacent[r];
                            adjacent[j] -= row * f;
                        }
                    }

                    lead += 1;
                }
            }
        }

        #[allow(clippy::int_plus_one)]
        impl<F: Float> core::ops::Index<usize> for $mat<F> {
            type Output = $rowtype<F>;

            #[inline]
            fn index(&self, idx: usize) -> &Self::Output {
                assert!(idx <= $dim - 1);

                unsafe { &*(self as *const _ as *const $crate::$rowtype<F>).add(idx) }
            }
        }

        #[allow(clippy::int_plus_one)]
        impl<F: Float> core::ops::IndexMut<usize> for $mat<F> {
            #[inline]
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                assert!(idx <= $dim - 1);

                unsafe { &mut *(self as *mut _ as *mut $crate::$rowtype<F>).add(idx) }
            }
        }

        impl<F: Float> core::ops::Add for $mat<F> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                $mat::<F> {
                    $(
                        $r: self.$r + rhs.$r
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::AddAssign for $mat<F> {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                $(
                    self.$r += rhs.$r
                );*
            }
        }

        impl<F: Float> core::ops::Sub for $mat<F> {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                $mat::<F> {
                    $(
                        $r: self.$r - rhs.$r
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::SubAssign for $mat<F> {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                $(
                    self.$r -= rhs.$r
                );*
            }
        }

        impl<F: Float> core::ops::Mul<F> for $mat<F> {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: F) -> Self::Output {
                $mat::<F> {
                    $(
                        $r: self.$r * rhs
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::MulAssign<F> for $mat<F> {
            #[inline]
            fn mul_assign(&mut self, rhs: F) {
                $(
                    self.$r *= rhs
                );*
            }
        }

        impl<F: Float> core::ops::Div<F> for $mat<F> {
            type Output = Self;

            #[inline]
            fn div(self, rhs: F) -> Self::Output {
                $mat::<F> {
                    $(
                        $r: self.$r / rhs
                    ),*
                }
            }
        }

        impl<F: Float> core::ops::DivAssign<F> for $mat<F> {
            #[inline]
            fn div_assign(&mut self, rhs: F) {
                $(
                    self.$r /= rhs
                );*
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_vec_ops {
    ($vec:ident, $dim:expr, $($c:ident),*) => {
        impl<F: Float> core::ops::Index<usize> for $vec<F> {
            type Output = F;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                assert!(index <= $dim);

                unsafe { &*(self as *const _ as *const F).add(index) }
            }
        }

        impl<F: Float> core::ops::IndexMut<usize> for $vec<F> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                assert!(index <= $dim);

                unsafe { &mut *(self as *mut _ as *mut F).add(index) }
            }
        }

        impl<F: Float> $vec<F> {
            /// Normalizes vector, preserving directing and making its magnitude equal to `1`.
            #[inline]
            pub fn normalize(&mut self) {
                let mag = self.magnitude();

                $(
                    self.$c /= mag
                );*
            }

            /// Returns normalized copy of the vector. See [`Self::normalize`].
            #[inline]
            pub fn normalized(&self) -> Self {
                let mag = self.magnitude();

                Self {
                    $(
                        $c: self.$c / mag
                    ),*
                }
            }

            /// Converts the vector to an array.
            #[inline]
            pub fn to_array(&self) -> [F; $dim + 1] {
                unsafe { std::mem::transmute_copy(self) }
            }

            /// Converts array to a vector.
            #[inline]
            pub fn from_array(array: [F; $dim + 1]) -> Self {
                unsafe { std::mem::transmute_copy(&array) }
            }

            /// Converts the vector to an array slice.
            #[inline]
            pub fn as_array(&self) -> &[F; $dim + 1] {
                unsafe { std::mem::transmute_copy(&self) }
            }

            /// Converts the vector to a mutable array slice.
            #[inline]
            pub fn as_array_mut<'a>(&'a mut self) -> &'a [F; $dim + 1] {
                unsafe { std::mem::transmute_copy(&self) }
            }

            /// Computes the dot(scalar) product between two vectors.
            /// Dot product for two normalized vector is equal to the cosine of the angle between
            /// them.
            #[inline]
            pub fn dot(&self, other: Self) -> F {
                $(self.$c * other.$c +)* F::ZERO
            }

            /// Computes the dot product between two vectors normalizing them beforehand.
            #[inline]
            pub fn dot_normalized(&self, other: Self) -> F {
                self.normalized().dot(other.normalized())
            }

            /// Returns angle in radians between two vectors. Output range is: `[0, pi]`.
            #[inline]
            pub fn angle_to(&self, other: Self) -> F {
                self.dot_normalized(other).acos()
            }

            /// Returns angle in radians between two vectors that goes along circle arc
            /// counter-clockwise. Output range is: `[0, 2pi]`.
            /// ```
            /// # use math::vector;
            /// let a = vector!(1, 1);
            /// let b = vector!(-1, 1);
            /// assert_eq!(a.arc_angle_to(b).to_degrees(), 90.0);
            /// assert_eq!(b.arc_angle_to(a).to_degrees(), 270.0);
            /// /*  (B). -- ~~~ -- .(A)
            ///     .-~\    90.0   / ~-.
            ///    /     \        /     \
            ///   /        \    /        \
            ///  |          \  /          |
            ///  |           ()           |
            ///  |                        |
            ///   \         270.0        /
            ///    \                    /
            ///     `-.              .-'
            ///         ~- . ___ . -~    */
            /// ```
            #[inline]
            pub fn arc_angle_to(&self, other: Self) -> F {
                let (v1, v2) = (self.normalized(), other.normalized());

                let dot = v1.dot(v2);
                let det = v1.x * v2.y - v1.y * v2.x;

                let mut ang = det.atan2(dot);
                if ang < F::ZERO {
                    ang += F::PI * F::TWO
                }

                ang
            }

            /// Projects a vector onto another vector. Axis and the resulting vector are collinear.
            #[inline]
            pub fn project_onto(&mut self, axis: Self) {
                let an = axis.normalized();

                *self = an * self.dot(an);
            }

            /// Returns the projected copy of the vector onto another vector. See
            /// [`Self::project_onto`].
            #[inline]
            pub fn projected_onto(&self, axis: Self) -> Self {
                let an = axis.normalized();

                an * self.dot(an)
            }

            /// Computes the distance between two vectors.
            #[inline]
            pub fn distance_to(&self, other: Self) -> F {
                (other - *self).magnitude()
            }

            /// Computes the squared distance between two vectors.
            #[inline]
            pub fn sqr_distance_to(&self, other: Self) -> F {
                (other - *self).sqr_magnitude()
            }

            /// Linearly interpolates between two vectors.
            #[inline]
            pub fn lerp(self, end: Self, t: F) -> Self {
                self + (end - self) * t
            }

            /// Inverse linear interpolation between two vectors.
            #[inline]
            pub fn inv_lerp(self, end: Self, v: Self) -> F {
                let ab = end - self;
                let av = v - self;

                av.dot(ab)
            }

            /// Returns normalized copy of the linear interpolation between vectors.
            #[inline]
            pub fn nlerp(self, end: Self, t: F) -> Self {
                self.lerp(end, t).normalized()
            }

            /// Spherically interpolates between two vectors.
            #[inline]
            pub fn slerp(self, end: Self, t: F) -> Self {
                let omega = self.dot_normalized(end).acos();

                self * (((F::ONE - t) * omega).sin() / omega.sin())
                    + end * ((t * omega).sin() / omega.sin())
            }

            /// Computes the product of all elements in the vector.
            #[inline]
            pub fn product(&self) -> F {
                $(
                    self.$c *
                )* F::ONE
            }

            /// Computes the sum of all elements in the vector.
            #[inline]
            pub fn sum(&self) -> F {
                $(
                    self.$c +
                )* F::ZERO
            }
        }
    };
}
