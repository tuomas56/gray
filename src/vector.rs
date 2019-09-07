use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct V4(pub f32, pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct V3(pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct V2(pub f32, pub f32);
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct V1(pub f32);

impl ops::Add for V4 {
    type Output = V4;

    fn add(self, rhs: V4) -> V4 {
        V4(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl ops::AddAssign for V4 {
    fn add_assign(&mut self, rhs: V4) {
        *self = *self + rhs;
    }
}

impl ops::Sub for V4 {
    type Output = V4;

    fn sub(self, rhs: V4) -> V4 {
        V4(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3)
    }
}

impl ops::Neg for V4 {
    type Output = V4;

    fn neg(self) -> V4 {
        V4(-self.0, -self.1, -self.2, -self.3)
    }
}

impl ops::SubAssign for V4 {
    fn sub_assign(&mut self, rhs: V4) {
        *self = *self - rhs;
    }
}

impl ops::Mul for V4 {
    type Output = V4;

    fn mul(self, rhs: V4) -> V4 {
        V4(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, self.3 * rhs.3)
    }
}

impl ops::MulAssign for V4 {
    fn mul_assign(&mut self, rhs: V4) {
        *self = *self * rhs;
    }
}

impl ops::Div for V4 {
    type Output = V4;
    fn div(self, rhs: V4) -> V4 {
        V4(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2, self.3 / rhs.3)
    }
}

impl ops::DivAssign for V4 {
    fn div_assign(&mut self, rhs: V4) {
        *self = *self / rhs;
    }
}

impl ops::Add for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> V3 {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign for V3 {
    fn add_assign(&mut self, rhs: V3) {
        *self = *self + rhs;
    }
}

impl ops::Sub for V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> V3 {
        V3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign for V3 {
    fn sub_assign(&mut self, rhs: V3) {
        *self = *self - rhs;
    }
}

impl ops::Neg for V3 {
    type Output = V3;

    fn neg(self) -> V3 {
        V3(-self.0, -self.1, -self.2)
    }
}

impl ops::Mul for V3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> V3 {
        V3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign for V3 {
    fn mul_assign(&mut self, rhs: V3) {
        *self = *self * rhs;
    }
}

impl ops::Div for V3 {
    type Output = V3;
    fn div(self, rhs: V3) -> V3 {
        V3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl ops::DivAssign for V3 {
    fn div_assign(&mut self, rhs: V3) {
        *self = *self / rhs;
    }
}

impl ops::Add for V2 {
    type Output = V2;

    fn add(self, rhs: V2) -> V2 {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for V2 {
    fn add_assign(&mut self, rhs: V2) {
        *self = *self + rhs;
    }
}

impl ops::Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: V2) -> V2 {
        V2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::SubAssign for V2 {
    fn sub_assign(&mut self, rhs: V2) {
        *self = *self - rhs;
    }
}

impl ops::Neg for V2 {
    type Output = V2;

    fn neg(self) -> V2 {
        V2(-self.0, -self.1)
    }
}

impl ops::Mul for V2 {
    type Output = V2;

    fn mul(self, rhs: V2) -> V2 {
        V2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::MulAssign for V2 {
    fn mul_assign(&mut self, rhs: V2) {
        *self = *self * rhs;
    }
}

impl ops::Div for V2 {
    type Output = V2;
    fn div(self, rhs: V2) -> V2 {
        V2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl ops::DivAssign for V2 {
    fn div_assign(&mut self, rhs: V2) {
        *self = *self / rhs;
    }
}

impl ops::Add for V1 {
    type Output = V1;

    fn add(self, rhs: V1) -> V1 {
        V1(self.0 + rhs.0)
    }
}

impl ops::AddAssign for V1 {
    fn add_assign(&mut self, rhs: V1) {
        *self = *self + rhs;
    }
}

impl ops::Sub for V1 {
    type Output = V1;

    fn sub(self, rhs: V1) -> V1 {
        V1(self.0 - rhs.0)
    }
}

impl ops::SubAssign for V1 {
    fn sub_assign(&mut self, rhs: V1) {
        *self = *self - rhs;
    }
}

impl ops::Neg for V1 {
    type Output = V1;

    fn neg(self) -> V1 {
        V1(-self.0)
    }
}

impl ops::Mul for V1 {
    type Output = V1;

    fn mul(self, rhs: V1) -> V1 {
        V1(self.0 * rhs.0)
    }
}

impl ops::MulAssign for V1 {
    fn mul_assign(&mut self, rhs: V1) {
        *self = *self * rhs;
    }
}

impl ops::Div for V1 {
    type Output = V1;
    fn div(self, rhs: V1) -> V1 {
        V1(self.0 / rhs.0)
    }
}

impl ops::DivAssign for V1 {
    fn div_assign(&mut self, rhs: V1) {
        *self = *self / rhs;
    }
}

impl V4 {
    pub fn x(&self) -> V1 { V1(self.0) }
    pub fn y(&self) -> V1 { V1(self.1) }
    pub fn z(&self) -> V1 { V1(self.2) }
    pub fn w(&self) -> V1 { V1(self.3) }
    pub fn xx(&self) -> V2 { V2(self.0, self.0) }
    pub fn xy(&self) -> V2 { V2(self.0, self.1) }
    pub fn xz(&self) -> V2 { V2(self.0, self.2) }
    pub fn xw(&self) -> V2 { V2(self.0, self.3) }
    pub fn yx(&self) -> V2 { V2(self.1, self.0) }
    pub fn yy(&self) -> V2 { V2(self.1, self.1) }
    pub fn yz(&self) -> V2 { V2(self.1, self.2) }
    pub fn yw(&self) -> V2 { V2(self.1, self.3) }
    pub fn zx(&self) -> V2 { V2(self.2, self.0) }
    pub fn zy(&self) -> V2 { V2(self.2, self.1) }
    pub fn zz(&self) -> V2 { V2(self.2, self.2) }
    pub fn zw(&self) -> V2 { V2(self.2, self.3) }
    pub fn wx(&self) -> V2 { V2(self.3, self.0) }
    pub fn wy(&self) -> V2 { V2(self.3, self.1) }
    pub fn wz(&self) -> V2 { V2(self.3, self.2) }
    pub fn ww(&self) -> V2 { V2(self.3, self.3) }
    pub fn xxx(&self) -> V3 { V3(self.0, self.0, self.0) }
    pub fn xxy(&self) -> V3 { V3(self.0, self.0, self.1) }
    pub fn xxz(&self) -> V3 { V3(self.0, self.0, self.2) }
    pub fn xxw(&self) -> V3 { V3(self.0, self.0, self.3) }
    pub fn xyx(&self) -> V3 { V3(self.0, self.1, self.0) }
    pub fn xyy(&self) -> V3 { V3(self.0, self.1, self.1) }
    pub fn xyz(&self) -> V3 { V3(self.0, self.1, self.2) }
    pub fn xyw(&self) -> V3 { V3(self.0, self.1, self.3) }
    pub fn xzx(&self) -> V3 { V3(self.0, self.2, self.0) }
    pub fn xzy(&self) -> V3 { V3(self.0, self.2, self.1) }
    pub fn xzz(&self) -> V3 { V3(self.0, self.2, self.2) }
    pub fn xzw(&self) -> V3 { V3(self.0, self.2, self.3) }
    pub fn xwx(&self) -> V3 { V3(self.0, self.3, self.0) }
    pub fn xwy(&self) -> V3 { V3(self.0, self.3, self.1) }
    pub fn xwz(&self) -> V3 { V3(self.0, self.3, self.2) }
    pub fn xww(&self) -> V3 { V3(self.0, self.3, self.3) }
    pub fn yxx(&self) -> V3 { V3(self.1, self.0, self.0) }
    pub fn yxy(&self) -> V3 { V3(self.1, self.0, self.1) }
    pub fn yxz(&self) -> V3 { V3(self.1, self.0, self.2) }
    pub fn yxw(&self) -> V3 { V3(self.1, self.0, self.3) }
    pub fn yyx(&self) -> V3 { V3(self.1, self.1, self.0) }
    pub fn yyy(&self) -> V3 { V3(self.1, self.1, self.1) }
    pub fn yyz(&self) -> V3 { V3(self.1, self.1, self.2) }
    pub fn yyw(&self) -> V3 { V3(self.1, self.1, self.3) }
    pub fn yzx(&self) -> V3 { V3(self.1, self.2, self.0) }
    pub fn yzy(&self) -> V3 { V3(self.1, self.2, self.1) }
    pub fn yzz(&self) -> V3 { V3(self.1, self.2, self.2) }
    pub fn yzw(&self) -> V3 { V3(self.1, self.2, self.3) }
    pub fn ywx(&self) -> V3 { V3(self.1, self.3, self.0) }
    pub fn ywy(&self) -> V3 { V3(self.1, self.3, self.1) }
    pub fn ywz(&self) -> V3 { V3(self.1, self.3, self.2) }
    pub fn yww(&self) -> V3 { V3(self.1, self.3, self.3) }
    pub fn zxx(&self) -> V3 { V3(self.2, self.0, self.0) }
    pub fn zxy(&self) -> V3 { V3(self.2, self.0, self.1) }
    pub fn zxz(&self) -> V3 { V3(self.2, self.0, self.2) }
    pub fn zxw(&self) -> V3 { V3(self.2, self.0, self.3) }
    pub fn zyx(&self) -> V3 { V3(self.2, self.1, self.0) }
    pub fn zyy(&self) -> V3 { V3(self.2, self.1, self.1) }
    pub fn zyz(&self) -> V3 { V3(self.2, self.1, self.2) }
    pub fn zyw(&self) -> V3 { V3(self.2, self.1, self.3) }
    pub fn zzx(&self) -> V3 { V3(self.2, self.2, self.0) }
    pub fn zzy(&self) -> V3 { V3(self.2, self.2, self.1) }
    pub fn zzz(&self) -> V3 { V3(self.2, self.2, self.2) }
    pub fn zzw(&self) -> V3 { V3(self.2, self.2, self.3) }
    pub fn zwx(&self) -> V3 { V3(self.2, self.3, self.0) }
    pub fn zwy(&self) -> V3 { V3(self.2, self.3, self.1) }
    pub fn zwz(&self) -> V3 { V3(self.2, self.3, self.2) }
    pub fn zww(&self) -> V3 { V3(self.2, self.3, self.3) }
    pub fn wxx(&self) -> V3 { V3(self.3, self.0, self.0) }
    pub fn wxy(&self) -> V3 { V3(self.3, self.0, self.1) }
    pub fn wxz(&self) -> V3 { V3(self.3, self.0, self.2) }
    pub fn wxw(&self) -> V3 { V3(self.3, self.0, self.3) }
    pub fn wyx(&self) -> V3 { V3(self.3, self.1, self.0) }
    pub fn wyy(&self) -> V3 { V3(self.3, self.1, self.1) }
    pub fn wyz(&self) -> V3 { V3(self.3, self.1, self.2) }
    pub fn wyw(&self) -> V3 { V3(self.3, self.1, self.3) }
    pub fn wzx(&self) -> V3 { V3(self.3, self.2, self.0) }
    pub fn wzy(&self) -> V3 { V3(self.3, self.2, self.1) }
    pub fn wzz(&self) -> V3 { V3(self.3, self.2, self.2) }
    pub fn wzw(&self) -> V3 { V3(self.3, self.2, self.3) }
    pub fn wwx(&self) -> V3 { V3(self.3, self.3, self.0) }
    pub fn wwy(&self) -> V3 { V3(self.3, self.3, self.1) }
    pub fn wwz(&self) -> V3 { V3(self.3, self.3, self.2) }
    pub fn www(&self) -> V3 { V3(self.3, self.3, self.3) }
    pub fn xxxx(&self) -> V4 { V4(self.0, self.0, self.0, self.0) }
    pub fn xxxy(&self) -> V4 { V4(self.0, self.0, self.0, self.1) }
    pub fn xxxz(&self) -> V4 { V4(self.0, self.0, self.0, self.2) }
    pub fn xxxw(&self) -> V4 { V4(self.0, self.0, self.0, self.3) }
    pub fn xxyx(&self) -> V4 { V4(self.0, self.0, self.1, self.0) }
    pub fn xxyy(&self) -> V4 { V4(self.0, self.0, self.1, self.1) }
    pub fn xxyz(&self) -> V4 { V4(self.0, self.0, self.1, self.2) }
    pub fn xxyw(&self) -> V4 { V4(self.0, self.0, self.1, self.3) }
    pub fn xxzx(&self) -> V4 { V4(self.0, self.0, self.2, self.0) }
    pub fn xxzy(&self) -> V4 { V4(self.0, self.0, self.2, self.1) }
    pub fn xxzz(&self) -> V4 { V4(self.0, self.0, self.2, self.2) }
    pub fn xxzw(&self) -> V4 { V4(self.0, self.0, self.2, self.3) }
    pub fn xxwx(&self) -> V4 { V4(self.0, self.0, self.3, self.0) }
    pub fn xxwy(&self) -> V4 { V4(self.0, self.0, self.3, self.1) }
    pub fn xxwz(&self) -> V4 { V4(self.0, self.0, self.3, self.2) }
    pub fn xxww(&self) -> V4 { V4(self.0, self.0, self.3, self.3) }
    pub fn xyxx(&self) -> V4 { V4(self.0, self.1, self.0, self.0) }
    pub fn xyxy(&self) -> V4 { V4(self.0, self.1, self.0, self.1) }
    pub fn xyxz(&self) -> V4 { V4(self.0, self.1, self.0, self.2) }
    pub fn xyxw(&self) -> V4 { V4(self.0, self.1, self.0, self.3) }
    pub fn xyyx(&self) -> V4 { V4(self.0, self.1, self.1, self.0) }
    pub fn xyyy(&self) -> V4 { V4(self.0, self.1, self.1, self.1) }
    pub fn xyyz(&self) -> V4 { V4(self.0, self.1, self.1, self.2) }
    pub fn xyyw(&self) -> V4 { V4(self.0, self.1, self.1, self.3) }
    pub fn xyzx(&self) -> V4 { V4(self.0, self.1, self.2, self.0) }
    pub fn xyzy(&self) -> V4 { V4(self.0, self.1, self.2, self.1) }
    pub fn xyzz(&self) -> V4 { V4(self.0, self.1, self.2, self.2) }
    pub fn xyzw(&self) -> V4 { V4(self.0, self.1, self.2, self.3) }
    pub fn xywx(&self) -> V4 { V4(self.0, self.1, self.3, self.0) }
    pub fn xywy(&self) -> V4 { V4(self.0, self.1, self.3, self.1) }
    pub fn xywz(&self) -> V4 { V4(self.0, self.1, self.3, self.2) }
    pub fn xyww(&self) -> V4 { V4(self.0, self.1, self.3, self.3) }
    pub fn xzxx(&self) -> V4 { V4(self.0, self.2, self.0, self.0) }
    pub fn xzxy(&self) -> V4 { V4(self.0, self.2, self.0, self.1) }
    pub fn xzxz(&self) -> V4 { V4(self.0, self.2, self.0, self.2) }
    pub fn xzxw(&self) -> V4 { V4(self.0, self.2, self.0, self.3) }
    pub fn xzyx(&self) -> V4 { V4(self.0, self.2, self.1, self.0) }
    pub fn xzyy(&self) -> V4 { V4(self.0, self.2, self.1, self.1) }
    pub fn xzyz(&self) -> V4 { V4(self.0, self.2, self.1, self.2) }
    pub fn xzyw(&self) -> V4 { V4(self.0, self.2, self.1, self.3) }
    pub fn xzzx(&self) -> V4 { V4(self.0, self.2, self.2, self.0) }
    pub fn xzzy(&self) -> V4 { V4(self.0, self.2, self.2, self.1) }
    pub fn xzzz(&self) -> V4 { V4(self.0, self.2, self.2, self.2) }
    pub fn xzzw(&self) -> V4 { V4(self.0, self.2, self.2, self.3) }
    pub fn xzwx(&self) -> V4 { V4(self.0, self.2, self.3, self.0) }
    pub fn xzwy(&self) -> V4 { V4(self.0, self.2, self.3, self.1) }
    pub fn xzwz(&self) -> V4 { V4(self.0, self.2, self.3, self.2) }
    pub fn xzww(&self) -> V4 { V4(self.0, self.2, self.3, self.3) }
    pub fn xwxx(&self) -> V4 { V4(self.0, self.3, self.0, self.0) }
    pub fn xwxy(&self) -> V4 { V4(self.0, self.3, self.0, self.1) }
    pub fn xwxz(&self) -> V4 { V4(self.0, self.3, self.0, self.2) }
    pub fn xwxw(&self) -> V4 { V4(self.0, self.3, self.0, self.3) }
    pub fn xwyx(&self) -> V4 { V4(self.0, self.3, self.1, self.0) }
    pub fn xwyy(&self) -> V4 { V4(self.0, self.3, self.1, self.1) }
    pub fn xwyz(&self) -> V4 { V4(self.0, self.3, self.1, self.2) }
    pub fn xwyw(&self) -> V4 { V4(self.0, self.3, self.1, self.3) }
    pub fn xwzx(&self) -> V4 { V4(self.0, self.3, self.2, self.0) }
    pub fn xwzy(&self) -> V4 { V4(self.0, self.3, self.2, self.1) }
    pub fn xwzz(&self) -> V4 { V4(self.0, self.3, self.2, self.2) }
    pub fn xwzw(&self) -> V4 { V4(self.0, self.3, self.2, self.3) }
    pub fn xwwx(&self) -> V4 { V4(self.0, self.3, self.3, self.0) }
    pub fn xwwy(&self) -> V4 { V4(self.0, self.3, self.3, self.1) }
    pub fn xwwz(&self) -> V4 { V4(self.0, self.3, self.3, self.2) }
    pub fn xwww(&self) -> V4 { V4(self.0, self.3, self.3, self.3) }
    pub fn yxxx(&self) -> V4 { V4(self.1, self.0, self.0, self.0) }
    pub fn yxxy(&self) -> V4 { V4(self.1, self.0, self.0, self.1) }
    pub fn yxxz(&self) -> V4 { V4(self.1, self.0, self.0, self.2) }
    pub fn yxxw(&self) -> V4 { V4(self.1, self.0, self.0, self.3) }
    pub fn yxyx(&self) -> V4 { V4(self.1, self.0, self.1, self.0) }
    pub fn yxyy(&self) -> V4 { V4(self.1, self.0, self.1, self.1) }
    pub fn yxyz(&self) -> V4 { V4(self.1, self.0, self.1, self.2) }
    pub fn yxyw(&self) -> V4 { V4(self.1, self.0, self.1, self.3) }
    pub fn yxzx(&self) -> V4 { V4(self.1, self.0, self.2, self.0) }
    pub fn yxzy(&self) -> V4 { V4(self.1, self.0, self.2, self.1) }
    pub fn yxzz(&self) -> V4 { V4(self.1, self.0, self.2, self.2) }
    pub fn yxzw(&self) -> V4 { V4(self.1, self.0, self.2, self.3) }
    pub fn yxwx(&self) -> V4 { V4(self.1, self.0, self.3, self.0) }
    pub fn yxwy(&self) -> V4 { V4(self.1, self.0, self.3, self.1) }
    pub fn yxwz(&self) -> V4 { V4(self.1, self.0, self.3, self.2) }
    pub fn yxww(&self) -> V4 { V4(self.1, self.0, self.3, self.3) }
    pub fn yyxx(&self) -> V4 { V4(self.1, self.1, self.0, self.0) }
    pub fn yyxy(&self) -> V4 { V4(self.1, self.1, self.0, self.1) }
    pub fn yyxz(&self) -> V4 { V4(self.1, self.1, self.0, self.2) }
    pub fn yyxw(&self) -> V4 { V4(self.1, self.1, self.0, self.3) }
    pub fn yyyx(&self) -> V4 { V4(self.1, self.1, self.1, self.0) }
    pub fn yyyy(&self) -> V4 { V4(self.1, self.1, self.1, self.1) }
    pub fn yyyz(&self) -> V4 { V4(self.1, self.1, self.1, self.2) }
    pub fn yyyw(&self) -> V4 { V4(self.1, self.1, self.1, self.3) }
    pub fn yyzx(&self) -> V4 { V4(self.1, self.1, self.2, self.0) }
    pub fn yyzy(&self) -> V4 { V4(self.1, self.1, self.2, self.1) }
    pub fn yyzz(&self) -> V4 { V4(self.1, self.1, self.2, self.2) }
    pub fn yyzw(&self) -> V4 { V4(self.1, self.1, self.2, self.3) }
    pub fn yywx(&self) -> V4 { V4(self.1, self.1, self.3, self.0) }
    pub fn yywy(&self) -> V4 { V4(self.1, self.1, self.3, self.1) }
    pub fn yywz(&self) -> V4 { V4(self.1, self.1, self.3, self.2) }
    pub fn yyww(&self) -> V4 { V4(self.1, self.1, self.3, self.3) }
    pub fn yzxx(&self) -> V4 { V4(self.1, self.2, self.0, self.0) }
    pub fn yzxy(&self) -> V4 { V4(self.1, self.2, self.0, self.1) }
    pub fn yzxz(&self) -> V4 { V4(self.1, self.2, self.0, self.2) }
    pub fn yzxw(&self) -> V4 { V4(self.1, self.2, self.0, self.3) }
    pub fn yzyx(&self) -> V4 { V4(self.1, self.2, self.1, self.0) }
    pub fn yzyy(&self) -> V4 { V4(self.1, self.2, self.1, self.1) }
    pub fn yzyz(&self) -> V4 { V4(self.1, self.2, self.1, self.2) }
    pub fn yzyw(&self) -> V4 { V4(self.1, self.2, self.1, self.3) }
    pub fn yzzx(&self) -> V4 { V4(self.1, self.2, self.2, self.0) }
    pub fn yzzy(&self) -> V4 { V4(self.1, self.2, self.2, self.1) }
    pub fn yzzz(&self) -> V4 { V4(self.1, self.2, self.2, self.2) }
    pub fn yzzw(&self) -> V4 { V4(self.1, self.2, self.2, self.3) }
    pub fn yzwx(&self) -> V4 { V4(self.1, self.2, self.3, self.0) }
    pub fn yzwy(&self) -> V4 { V4(self.1, self.2, self.3, self.1) }
    pub fn yzwz(&self) -> V4 { V4(self.1, self.2, self.3, self.2) }
    pub fn yzww(&self) -> V4 { V4(self.1, self.2, self.3, self.3) }
    pub fn ywxx(&self) -> V4 { V4(self.1, self.3, self.0, self.0) }
    pub fn ywxy(&self) -> V4 { V4(self.1, self.3, self.0, self.1) }
    pub fn ywxz(&self) -> V4 { V4(self.1, self.3, self.0, self.2) }
    pub fn ywxw(&self) -> V4 { V4(self.1, self.3, self.0, self.3) }
    pub fn ywyx(&self) -> V4 { V4(self.1, self.3, self.1, self.0) }
    pub fn ywyy(&self) -> V4 { V4(self.1, self.3, self.1, self.1) }
    pub fn ywyz(&self) -> V4 { V4(self.1, self.3, self.1, self.2) }
    pub fn ywyw(&self) -> V4 { V4(self.1, self.3, self.1, self.3) }
    pub fn ywzx(&self) -> V4 { V4(self.1, self.3, self.2, self.0) }
    pub fn ywzy(&self) -> V4 { V4(self.1, self.3, self.2, self.1) }
    pub fn ywzz(&self) -> V4 { V4(self.1, self.3, self.2, self.2) }
    pub fn ywzw(&self) -> V4 { V4(self.1, self.3, self.2, self.3) }
    pub fn ywwx(&self) -> V4 { V4(self.1, self.3, self.3, self.0) }
    pub fn ywwy(&self) -> V4 { V4(self.1, self.3, self.3, self.1) }
    pub fn ywwz(&self) -> V4 { V4(self.1, self.3, self.3, self.2) }
    pub fn ywww(&self) -> V4 { V4(self.1, self.3, self.3, self.3) }
    pub fn zxxx(&self) -> V4 { V4(self.2, self.0, self.0, self.0) }
    pub fn zxxy(&self) -> V4 { V4(self.2, self.0, self.0, self.1) }
    pub fn zxxz(&self) -> V4 { V4(self.2, self.0, self.0, self.2) }
    pub fn zxxw(&self) -> V4 { V4(self.2, self.0, self.0, self.3) }
    pub fn zxyx(&self) -> V4 { V4(self.2, self.0, self.1, self.0) }
    pub fn zxyy(&self) -> V4 { V4(self.2, self.0, self.1, self.1) }
    pub fn zxyz(&self) -> V4 { V4(self.2, self.0, self.1, self.2) }
    pub fn zxyw(&self) -> V4 { V4(self.2, self.0, self.1, self.3) }
    pub fn zxzx(&self) -> V4 { V4(self.2, self.0, self.2, self.0) }
    pub fn zxzy(&self) -> V4 { V4(self.2, self.0, self.2, self.1) }
    pub fn zxzz(&self) -> V4 { V4(self.2, self.0, self.2, self.2) }
    pub fn zxzw(&self) -> V4 { V4(self.2, self.0, self.2, self.3) }
    pub fn zxwx(&self) -> V4 { V4(self.2, self.0, self.3, self.0) }
    pub fn zxwy(&self) -> V4 { V4(self.2, self.0, self.3, self.1) }
    pub fn zxwz(&self) -> V4 { V4(self.2, self.0, self.3, self.2) }
    pub fn zxww(&self) -> V4 { V4(self.2, self.0, self.3, self.3) }
    pub fn zyxx(&self) -> V4 { V4(self.2, self.1, self.0, self.0) }
    pub fn zyxy(&self) -> V4 { V4(self.2, self.1, self.0, self.1) }
    pub fn zyxz(&self) -> V4 { V4(self.2, self.1, self.0, self.2) }
    pub fn zyxw(&self) -> V4 { V4(self.2, self.1, self.0, self.3) }
    pub fn zyyx(&self) -> V4 { V4(self.2, self.1, self.1, self.0) }
    pub fn zyyy(&self) -> V4 { V4(self.2, self.1, self.1, self.1) }
    pub fn zyyz(&self) -> V4 { V4(self.2, self.1, self.1, self.2) }
    pub fn zyyw(&self) -> V4 { V4(self.2, self.1, self.1, self.3) }
    pub fn zyzx(&self) -> V4 { V4(self.2, self.1, self.2, self.0) }
    pub fn zyzy(&self) -> V4 { V4(self.2, self.1, self.2, self.1) }
    pub fn zyzz(&self) -> V4 { V4(self.2, self.1, self.2, self.2) }
    pub fn zyzw(&self) -> V4 { V4(self.2, self.1, self.2, self.3) }
    pub fn zywx(&self) -> V4 { V4(self.2, self.1, self.3, self.0) }
    pub fn zywy(&self) -> V4 { V4(self.2, self.1, self.3, self.1) }
    pub fn zywz(&self) -> V4 { V4(self.2, self.1, self.3, self.2) }
    pub fn zyww(&self) -> V4 { V4(self.2, self.1, self.3, self.3) }
    pub fn zzxx(&self) -> V4 { V4(self.2, self.2, self.0, self.0) }
    pub fn zzxy(&self) -> V4 { V4(self.2, self.2, self.0, self.1) }
    pub fn zzxz(&self) -> V4 { V4(self.2, self.2, self.0, self.2) }
    pub fn zzxw(&self) -> V4 { V4(self.2, self.2, self.0, self.3) }
    pub fn zzyx(&self) -> V4 { V4(self.2, self.2, self.1, self.0) }
    pub fn zzyy(&self) -> V4 { V4(self.2, self.2, self.1, self.1) }
    pub fn zzyz(&self) -> V4 { V4(self.2, self.2, self.1, self.2) }
    pub fn zzyw(&self) -> V4 { V4(self.2, self.2, self.1, self.3) }
    pub fn zzzx(&self) -> V4 { V4(self.2, self.2, self.2, self.0) }
    pub fn zzzy(&self) -> V4 { V4(self.2, self.2, self.2, self.1) }
    pub fn zzzz(&self) -> V4 { V4(self.2, self.2, self.2, self.2) }
    pub fn zzzw(&self) -> V4 { V4(self.2, self.2, self.2, self.3) }
    pub fn zzwx(&self) -> V4 { V4(self.2, self.2, self.3, self.0) }
    pub fn zzwy(&self) -> V4 { V4(self.2, self.2, self.3, self.1) }
    pub fn zzwz(&self) -> V4 { V4(self.2, self.2, self.3, self.2) }
    pub fn zzww(&self) -> V4 { V4(self.2, self.2, self.3, self.3) }
    pub fn zwxx(&self) -> V4 { V4(self.2, self.3, self.0, self.0) }
    pub fn zwxy(&self) -> V4 { V4(self.2, self.3, self.0, self.1) }
    pub fn zwxz(&self) -> V4 { V4(self.2, self.3, self.0, self.2) }
    pub fn zwxw(&self) -> V4 { V4(self.2, self.3, self.0, self.3) }
    pub fn zwyx(&self) -> V4 { V4(self.2, self.3, self.1, self.0) }
    pub fn zwyy(&self) -> V4 { V4(self.2, self.3, self.1, self.1) }
    pub fn zwyz(&self) -> V4 { V4(self.2, self.3, self.1, self.2) }
    pub fn zwyw(&self) -> V4 { V4(self.2, self.3, self.1, self.3) }
    pub fn zwzx(&self) -> V4 { V4(self.2, self.3, self.2, self.0) }
    pub fn zwzy(&self) -> V4 { V4(self.2, self.3, self.2, self.1) }
    pub fn zwzz(&self) -> V4 { V4(self.2, self.3, self.2, self.2) }
    pub fn zwzw(&self) -> V4 { V4(self.2, self.3, self.2, self.3) }
    pub fn zwwx(&self) -> V4 { V4(self.2, self.3, self.3, self.0) }
    pub fn zwwy(&self) -> V4 { V4(self.2, self.3, self.3, self.1) }
    pub fn zwwz(&self) -> V4 { V4(self.2, self.3, self.3, self.2) }
    pub fn zwww(&self) -> V4 { V4(self.2, self.3, self.3, self.3) }
    pub fn wxxx(&self) -> V4 { V4(self.3, self.0, self.0, self.0) }
    pub fn wxxy(&self) -> V4 { V4(self.3, self.0, self.0, self.1) }
    pub fn wxxz(&self) -> V4 { V4(self.3, self.0, self.0, self.2) }
    pub fn wxxw(&self) -> V4 { V4(self.3, self.0, self.0, self.3) }
    pub fn wxyx(&self) -> V4 { V4(self.3, self.0, self.1, self.0) }
    pub fn wxyy(&self) -> V4 { V4(self.3, self.0, self.1, self.1) }
    pub fn wxyz(&self) -> V4 { V4(self.3, self.0, self.1, self.2) }
    pub fn wxyw(&self) -> V4 { V4(self.3, self.0, self.1, self.3) }
    pub fn wxzx(&self) -> V4 { V4(self.3, self.0, self.2, self.0) }
    pub fn wxzy(&self) -> V4 { V4(self.3, self.0, self.2, self.1) }
    pub fn wxzz(&self) -> V4 { V4(self.3, self.0, self.2, self.2) }
    pub fn wxzw(&self) -> V4 { V4(self.3, self.0, self.2, self.3) }
    pub fn wxwx(&self) -> V4 { V4(self.3, self.0, self.3, self.0) }
    pub fn wxwy(&self) -> V4 { V4(self.3, self.0, self.3, self.1) }
    pub fn wxwz(&self) -> V4 { V4(self.3, self.0, self.3, self.2) }
    pub fn wxww(&self) -> V4 { V4(self.3, self.0, self.3, self.3) }
    pub fn wyxx(&self) -> V4 { V4(self.3, self.1, self.0, self.0) }
    pub fn wyxy(&self) -> V4 { V4(self.3, self.1, self.0, self.1) }
    pub fn wyxz(&self) -> V4 { V4(self.3, self.1, self.0, self.2) }
    pub fn wyxw(&self) -> V4 { V4(self.3, self.1, self.0, self.3) }
    pub fn wyyx(&self) -> V4 { V4(self.3, self.1, self.1, self.0) }
    pub fn wyyy(&self) -> V4 { V4(self.3, self.1, self.1, self.1) }
    pub fn wyyz(&self) -> V4 { V4(self.3, self.1, self.1, self.2) }
    pub fn wyyw(&self) -> V4 { V4(self.3, self.1, self.1, self.3) }
    pub fn wyzx(&self) -> V4 { V4(self.3, self.1, self.2, self.0) }
    pub fn wyzy(&self) -> V4 { V4(self.3, self.1, self.2, self.1) }
    pub fn wyzz(&self) -> V4 { V4(self.3, self.1, self.2, self.2) }
    pub fn wyzw(&self) -> V4 { V4(self.3, self.1, self.2, self.3) }
    pub fn wywx(&self) -> V4 { V4(self.3, self.1, self.3, self.0) }
    pub fn wywy(&self) -> V4 { V4(self.3, self.1, self.3, self.1) }
    pub fn wywz(&self) -> V4 { V4(self.3, self.1, self.3, self.2) }
    pub fn wyww(&self) -> V4 { V4(self.3, self.1, self.3, self.3) }
    pub fn wzxx(&self) -> V4 { V4(self.3, self.2, self.0, self.0) }
    pub fn wzxy(&self) -> V4 { V4(self.3, self.2, self.0, self.1) }
    pub fn wzxz(&self) -> V4 { V4(self.3, self.2, self.0, self.2) }
    pub fn wzxw(&self) -> V4 { V4(self.3, self.2, self.0, self.3) }
    pub fn wzyx(&self) -> V4 { V4(self.3, self.2, self.1, self.0) }
    pub fn wzyy(&self) -> V4 { V4(self.3, self.2, self.1, self.1) }
    pub fn wzyz(&self) -> V4 { V4(self.3, self.2, self.1, self.2) }
    pub fn wzyw(&self) -> V4 { V4(self.3, self.2, self.1, self.3) }
    pub fn wzzx(&self) -> V4 { V4(self.3, self.2, self.2, self.0) }
    pub fn wzzy(&self) -> V4 { V4(self.3, self.2, self.2, self.1) }
    pub fn wzzz(&self) -> V4 { V4(self.3, self.2, self.2, self.2) }
    pub fn wzzw(&self) -> V4 { V4(self.3, self.2, self.2, self.3) }
    pub fn wzwx(&self) -> V4 { V4(self.3, self.2, self.3, self.0) }
    pub fn wzwy(&self) -> V4 { V4(self.3, self.2, self.3, self.1) }
    pub fn wzwz(&self) -> V4 { V4(self.3, self.2, self.3, self.2) }
    pub fn wzww(&self) -> V4 { V4(self.3, self.2, self.3, self.3) }
    pub fn wwxx(&self) -> V4 { V4(self.3, self.3, self.0, self.0) }
    pub fn wwxy(&self) -> V4 { V4(self.3, self.3, self.0, self.1) }
    pub fn wwxz(&self) -> V4 { V4(self.3, self.3, self.0, self.2) }
    pub fn wwxw(&self) -> V4 { V4(self.3, self.3, self.0, self.3) }
    pub fn wwyx(&self) -> V4 { V4(self.3, self.3, self.1, self.0) }
    pub fn wwyy(&self) -> V4 { V4(self.3, self.3, self.1, self.1) }
    pub fn wwyz(&self) -> V4 { V4(self.3, self.3, self.1, self.2) }
    pub fn wwyw(&self) -> V4 { V4(self.3, self.3, self.1, self.3) }
    pub fn wwzx(&self) -> V4 { V4(self.3, self.3, self.2, self.0) }
    pub fn wwzy(&self) -> V4 { V4(self.3, self.3, self.2, self.1) }
    pub fn wwzz(&self) -> V4 { V4(self.3, self.3, self.2, self.2) }
    pub fn wwzw(&self) -> V4 { V4(self.3, self.3, self.2, self.3) }
    pub fn wwwx(&self) -> V4 { V4(self.3, self.3, self.3, self.0) }
    pub fn wwwy(&self) -> V4 { V4(self.3, self.3, self.3, self.1) }
    pub fn wwwz(&self) -> V4 { V4(self.3, self.3, self.3, self.2) }
    pub fn wwww(&self) -> V4 { V4(self.3, self.3, self.3, self.3) }

    pub fn set_x(&self, x: f32) -> V4 {
        V4(x, self.1, self.2, self.3)
    }

    pub fn set_y(&self, y: f32) -> V4 {
        V4(self.0, y, self.2, self.3)
    }

    pub fn set_z(&self, z: f32) -> V4 {
        V4(self.0, self.1, z, self.3)
    }

    pub fn set_w(&self, w: f32) -> V4 {
        V4(self.0, self.1, self.2, w)
    }

    pub fn sum(&self) -> V1 {
        V1(self.0 + self.1 + self.2 + self.3)
    }

    pub fn dot(self, other: V4) -> V1 {
        (self * other).sum()
    }

    pub fn mag2(self) -> V1 {
        self.dot(self)
    }

    pub fn mag(self) -> V1 {
        V1(self.mag2().val().sqrt())
    }

    pub fn norm(self) -> V4 {
        let mag = self.mag();
        if mag.val() == 0.0 {
            self
        } else {
            self / mag.xxxx()
        }
    }

    pub fn fmap<F: Fn(f32) -> f32>(self, f: F) -> V4 {
        V4(f(self.0), f(self.1), f(self.2), f(self.3))
    }
}

impl V3 {
    pub fn x(&self) -> V1 { V1(self.0) }
    pub fn y(&self) -> V1 { V1(self.1) }
    pub fn z(&self) -> V1 { V1(self.2) }
    pub fn xx(&self) -> V2 { V2(self.0, self.0) }
    pub fn xy(&self) -> V2 { V2(self.0, self.1) }
    pub fn xz(&self) -> V2 { V2(self.0, self.2) }
    pub fn yx(&self) -> V2 { V2(self.1, self.0) }
    pub fn yy(&self) -> V2 { V2(self.1, self.1) }
    pub fn yz(&self) -> V2 { V2(self.1, self.2) }
    pub fn zx(&self) -> V2 { V2(self.2, self.0) }
    pub fn zy(&self) -> V2 { V2(self.2, self.1) }
    pub fn zz(&self) -> V2 { V2(self.2, self.2) }
    pub fn xxx(&self) -> V3 { V3(self.0, self.0, self.0) }
    pub fn xxy(&self) -> V3 { V3(self.0, self.0, self.1) }
    pub fn xxz(&self) -> V3 { V3(self.0, self.0, self.2) }
    pub fn xyx(&self) -> V3 { V3(self.0, self.1, self.0) }
    pub fn xyy(&self) -> V3 { V3(self.0, self.1, self.1) }
    pub fn xyz(&self) -> V3 { V3(self.0, self.1, self.2) }
    pub fn xzx(&self) -> V3 { V3(self.0, self.2, self.0) }
    pub fn xzy(&self) -> V3 { V3(self.0, self.2, self.1) }
    pub fn xzz(&self) -> V3 { V3(self.0, self.2, self.2) }
    pub fn yxx(&self) -> V3 { V3(self.1, self.0, self.0) }
    pub fn yxy(&self) -> V3 { V3(self.1, self.0, self.1) }
    pub fn yxz(&self) -> V3 { V3(self.1, self.0, self.2) }
    pub fn yyx(&self) -> V3 { V3(self.1, self.1, self.0) }
    pub fn yyy(&self) -> V3 { V3(self.1, self.1, self.1) }
    pub fn yyz(&self) -> V3 { V3(self.1, self.1, self.2) }
    pub fn yzx(&self) -> V3 { V3(self.1, self.2, self.0) }
    pub fn yzy(&self) -> V3 { V3(self.1, self.2, self.1) }
    pub fn yzz(&self) -> V3 { V3(self.1, self.2, self.2) }
    pub fn zxx(&self) -> V3 { V3(self.2, self.0, self.0) }
    pub fn zxy(&self) -> V3 { V3(self.2, self.0, self.1) }
    pub fn zxz(&self) -> V3 { V3(self.2, self.0, self.2) }
    pub fn zyx(&self) -> V3 { V3(self.2, self.1, self.0) }
    pub fn zyy(&self) -> V3 { V3(self.2, self.1, self.1) }
    pub fn zyz(&self) -> V3 { V3(self.2, self.1, self.2) }
    pub fn zzx(&self) -> V3 { V3(self.2, self.2, self.0) }
    pub fn zzy(&self) -> V3 { V3(self.2, self.2, self.1) }
    pub fn zzz(&self) -> V3 { V3(self.2, self.2, self.2) }
    pub fn xxxx(&self) -> V4 { V4(self.0, self.0, self.0, self.0) }
    pub fn xxxy(&self) -> V4 { V4(self.0, self.0, self.0, self.1) }
    pub fn xxxz(&self) -> V4 { V4(self.0, self.0, self.0, self.2) }
    pub fn xxyx(&self) -> V4 { V4(self.0, self.0, self.1, self.0) }
    pub fn xxyy(&self) -> V4 { V4(self.0, self.0, self.1, self.1) }
    pub fn xxyz(&self) -> V4 { V4(self.0, self.0, self.1, self.2) }
    pub fn xxzx(&self) -> V4 { V4(self.0, self.0, self.2, self.0) }
    pub fn xxzy(&self) -> V4 { V4(self.0, self.0, self.2, self.1) }
    pub fn xxzz(&self) -> V4 { V4(self.0, self.0, self.2, self.2) }
    pub fn xyxx(&self) -> V4 { V4(self.0, self.1, self.0, self.0) }
    pub fn xyxy(&self) -> V4 { V4(self.0, self.1, self.0, self.1) }
    pub fn xyxz(&self) -> V4 { V4(self.0, self.1, self.0, self.2) }
    pub fn xyyx(&self) -> V4 { V4(self.0, self.1, self.1, self.0) }
    pub fn xyyy(&self) -> V4 { V4(self.0, self.1, self.1, self.1) }
    pub fn xyyz(&self) -> V4 { V4(self.0, self.1, self.1, self.2) }
    pub fn xyzx(&self) -> V4 { V4(self.0, self.1, self.2, self.0) }
    pub fn xyzy(&self) -> V4 { V4(self.0, self.1, self.2, self.1) }
    pub fn xyzz(&self) -> V4 { V4(self.0, self.1, self.2, self.2) }
    pub fn xzxx(&self) -> V4 { V4(self.0, self.2, self.0, self.0) }
    pub fn xzxy(&self) -> V4 { V4(self.0, self.2, self.0, self.1) }
    pub fn xzxz(&self) -> V4 { V4(self.0, self.2, self.0, self.2) }
    pub fn xzyx(&self) -> V4 { V4(self.0, self.2, self.1, self.0) }
    pub fn xzyy(&self) -> V4 { V4(self.0, self.2, self.1, self.1) }
    pub fn xzyz(&self) -> V4 { V4(self.0, self.2, self.1, self.2) }
    pub fn xzzx(&self) -> V4 { V4(self.0, self.2, self.2, self.0) }
    pub fn xzzy(&self) -> V4 { V4(self.0, self.2, self.2, self.1) }
    pub fn xzzz(&self) -> V4 { V4(self.0, self.2, self.2, self.2) }
    pub fn yxxx(&self) -> V4 { V4(self.1, self.0, self.0, self.0) }
    pub fn yxxy(&self) -> V4 { V4(self.1, self.0, self.0, self.1) }
    pub fn yxxz(&self) -> V4 { V4(self.1, self.0, self.0, self.2) }
    pub fn yxyx(&self) -> V4 { V4(self.1, self.0, self.1, self.0) }
    pub fn yxyy(&self) -> V4 { V4(self.1, self.0, self.1, self.1) }
    pub fn yxyz(&self) -> V4 { V4(self.1, self.0, self.1, self.2) }
    pub fn yxzx(&self) -> V4 { V4(self.1, self.0, self.2, self.0) }
    pub fn yxzy(&self) -> V4 { V4(self.1, self.0, self.2, self.1) }
    pub fn yxzz(&self) -> V4 { V4(self.1, self.0, self.2, self.2) }
    pub fn yyxx(&self) -> V4 { V4(self.1, self.1, self.0, self.0) }
    pub fn yyxy(&self) -> V4 { V4(self.1, self.1, self.0, self.1) }
    pub fn yyxz(&self) -> V4 { V4(self.1, self.1, self.0, self.2) }
    pub fn yyyx(&self) -> V4 { V4(self.1, self.1, self.1, self.0) }
    pub fn yyyy(&self) -> V4 { V4(self.1, self.1, self.1, self.1) }
    pub fn yyyz(&self) -> V4 { V4(self.1, self.1, self.1, self.2) }
    pub fn yyzx(&self) -> V4 { V4(self.1, self.1, self.2, self.0) }
    pub fn yyzy(&self) -> V4 { V4(self.1, self.1, self.2, self.1) }
    pub fn yyzz(&self) -> V4 { V4(self.1, self.1, self.2, self.2) }
    pub fn yzxx(&self) -> V4 { V4(self.1, self.2, self.0, self.0) }
    pub fn yzxy(&self) -> V4 { V4(self.1, self.2, self.0, self.1) }
    pub fn yzxz(&self) -> V4 { V4(self.1, self.2, self.0, self.2) }
    pub fn yzyx(&self) -> V4 { V4(self.1, self.2, self.1, self.0) }
    pub fn yzyy(&self) -> V4 { V4(self.1, self.2, self.1, self.1) }
    pub fn yzyz(&self) -> V4 { V4(self.1, self.2, self.1, self.2) }
    pub fn yzzx(&self) -> V4 { V4(self.1, self.2, self.2, self.0) }
    pub fn yzzy(&self) -> V4 { V4(self.1, self.2, self.2, self.1) }
    pub fn yzzz(&self) -> V4 { V4(self.1, self.2, self.2, self.2) }
    pub fn zxxx(&self) -> V4 { V4(self.2, self.0, self.0, self.0) }
    pub fn zxxy(&self) -> V4 { V4(self.2, self.0, self.0, self.1) }
    pub fn zxxz(&self) -> V4 { V4(self.2, self.0, self.0, self.2) }
    pub fn zxyx(&self) -> V4 { V4(self.2, self.0, self.1, self.0) }
    pub fn zxyy(&self) -> V4 { V4(self.2, self.0, self.1, self.1) }
    pub fn zxyz(&self) -> V4 { V4(self.2, self.0, self.1, self.2) }
    pub fn zxzx(&self) -> V4 { V4(self.2, self.0, self.2, self.0) }
    pub fn zxzy(&self) -> V4 { V4(self.2, self.0, self.2, self.1) }
    pub fn zxzz(&self) -> V4 { V4(self.2, self.0, self.2, self.2) }
    pub fn zyxx(&self) -> V4 { V4(self.2, self.1, self.0, self.0) }
    pub fn zyxy(&self) -> V4 { V4(self.2, self.1, self.0, self.1) }
    pub fn zyxz(&self) -> V4 { V4(self.2, self.1, self.0, self.2) }
    pub fn zyyx(&self) -> V4 { V4(self.2, self.1, self.1, self.0) }
    pub fn zyyy(&self) -> V4 { V4(self.2, self.1, self.1, self.1) }
    pub fn zyyz(&self) -> V4 { V4(self.2, self.1, self.1, self.2) }
    pub fn zyzx(&self) -> V4 { V4(self.2, self.1, self.2, self.0) }
    pub fn zyzy(&self) -> V4 { V4(self.2, self.1, self.2, self.1) }
    pub fn zyzz(&self) -> V4 { V4(self.2, self.1, self.2, self.2) }
    pub fn zzxx(&self) -> V4 { V4(self.2, self.2, self.0, self.0) }
    pub fn zzxy(&self) -> V4 { V4(self.2, self.2, self.0, self.1) }
    pub fn zzxz(&self) -> V4 { V4(self.2, self.2, self.0, self.2) }
    pub fn zzyx(&self) -> V4 { V4(self.2, self.2, self.1, self.0) }
    pub fn zzyy(&self) -> V4 { V4(self.2, self.2, self.1, self.1) }
    pub fn zzyz(&self) -> V4 { V4(self.2, self.2, self.1, self.2) }
    pub fn zzzx(&self) -> V4 { V4(self.2, self.2, self.2, self.0) }
    pub fn zzzy(&self) -> V4 { V4(self.2, self.2, self.2, self.1) }
    pub fn zzzz(&self) -> V4 { V4(self.2, self.2, self.2, self.2) }

    pub fn set_x(&self, x: f32) -> V3 {
        V3(x, self.1, self.2)
    }

    pub fn set_y(&self, y: f32) -> V3 {
        V3(self.0, y, self.2)
    }

    pub fn set_z(&self, z: f32) -> V3 {
        V3(self.0, self.1, z)
    }

    pub fn sum(&self) -> V1 {
        V1(self.0 + self.1 + self.2)
    }

    pub fn dot(self, other: V3) -> V1 {
        (self * other).sum()
    }

    pub fn mag2(self) -> V1 {
        self.dot(self)
    }

    pub fn mag(self) -> V1 {
        V1(self.mag2().val().sqrt())
    }

    pub fn norm(self) -> V3 {
        self / self.mag().xxx()
    }

    pub fn cross(self, other: V3) -> V3 {
        V3(self.1 * other.2 - self.2 * other.1, self.2 * other.0 - self.0 * other.2, self.0 * other.1 - self.1 * other.0)
    }

    pub fn fmap<F: Fn(f32) -> f32>(self, f: F) -> V3 {
        V3(f(self.0), f(self.1), f(self.2))
    }
}

impl V2 {
    pub fn x(&self) -> V1 { V1(self.0) }
    pub fn y(&self) -> V1 { V1(self.1) }
    pub fn xx(&self) -> V2 { V2(self.0, self.0) }
    pub fn xy(&self) -> V2 { V2(self.0, self.1) }
    pub fn yx(&self) -> V2 { V2(self.1, self.0) }
    pub fn yy(&self) -> V2 { V2(self.1, self.1) }
    pub fn xxx(&self) -> V3 { V3(self.0, self.0, self.0) }
    pub fn xxy(&self) -> V3 { V3(self.0, self.0, self.1) }
    pub fn xyx(&self) -> V3 { V3(self.0, self.1, self.0) }
    pub fn xyy(&self) -> V3 { V3(self.0, self.1, self.1) }
    pub fn yxx(&self) -> V3 { V3(self.1, self.0, self.0) }
    pub fn yxy(&self) -> V3 { V3(self.1, self.0, self.1) }
    pub fn yyx(&self) -> V3 { V3(self.1, self.1, self.0) }
    pub fn yyy(&self) -> V3 { V3(self.1, self.1, self.1) }
    pub fn xxxx(&self) -> V4 { V4(self.0, self.0, self.0, self.0) }
    pub fn xxxy(&self) -> V4 { V4(self.0, self.0, self.0, self.1) }
    pub fn xxyx(&self) -> V4 { V4(self.0, self.0, self.1, self.0) }
    pub fn xxyy(&self) -> V4 { V4(self.0, self.0, self.1, self.1) }
    pub fn xyxx(&self) -> V4 { V4(self.0, self.1, self.0, self.0) }
    pub fn xyxy(&self) -> V4 { V4(self.0, self.1, self.0, self.1) }
    pub fn xyyx(&self) -> V4 { V4(self.0, self.1, self.1, self.0) }
    pub fn xyyy(&self) -> V4 { V4(self.0, self.1, self.1, self.1) }
    pub fn yxxx(&self) -> V4 { V4(self.1, self.0, self.0, self.0) }
    pub fn yxxy(&self) -> V4 { V4(self.1, self.0, self.0, self.1) }
    pub fn yxyx(&self) -> V4 { V4(self.1, self.0, self.1, self.0) }
    pub fn yxyy(&self) -> V4 { V4(self.1, self.0, self.1, self.1) }
    pub fn yyxx(&self) -> V4 { V4(self.1, self.1, self.0, self.0) }
    pub fn yyxy(&self) -> V4 { V4(self.1, self.1, self.0, self.1) }
    pub fn yyyx(&self) -> V4 { V4(self.1, self.1, self.1, self.0) }
    pub fn yyyy(&self) -> V4 { V4(self.1, self.1, self.1, self.1) }

    pub fn set_x(&self, x: f32) -> V2 {
        V2(x, self.1)
    }

    pub fn set_y(&self, y: f32) -> V2 {
        V2(self.0, y)
    }

    pub fn sum(&self) -> V1 {
        V1(self.0 + self.1)
    }

    pub fn dot(self, other: V2) -> V1 {
        (self * other).sum()
    }

    pub fn mag2(self) -> V1 {
        self.dot(self)
    }

    pub fn mag(self) -> V1 {
        V1(self.mag2().val().sqrt())
    }

    pub fn norm(self) -> V2 {
        self / self.mag().xx()
    }

    pub fn fmap<F: Fn(f32) -> f32>(self, f: F) -> V2 {
        V2(f(self.0), f(self.1))
    }
}

impl V1 {
    pub fn x(&self) -> V1 { V1(self.0) }
    pub fn xx(&self) -> V2 { V2(self.0, self.0) }
    pub fn xxx(&self) -> V3 { V3(self.0, self.0, self.0) }
    pub fn xxxx(&self) -> V4 { V4(self.0, self.0, self.0, self.0) }

    pub fn set_x(&self, x: f32) -> V1 {
        V1(x)
    }

    pub fn sum(&self) -> V1 {
        V1(self.0)
    }

    pub fn dot(self, other: V1) -> V1 {
        (self * other).sum()
    }

    pub fn val(&self) -> f32 {
        self.0
    }

    pub fn mag2(self) -> V1 {
        self.dot(self)
    }

    pub fn mag(self) -> V1 {
        V1(self.mag2().val().sqrt())
    }

    pub fn norm(self) -> V1 {
        self / self.mag().x()
    }

    pub fn fmap<F: Fn(f32) -> f32>(self, f: F) -> V1 {
        V1(f(self.0))
    }
}

impl ops::Deref for V1 {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl ops::DerefMut for V1 {
    fn deref_mut(&mut self) -> &mut f32 {
        &mut self.0
    }
}

pub trait ToVector {
    fn v(self) -> V1;
}

impl ToVector for f32 {
    fn v(self) -> V1 {
        V1(self)
    }
}