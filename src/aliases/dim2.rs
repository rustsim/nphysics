use nalgebra::mat::{Mat2, Mat1};
use nalgebra::vec::{Vec2, Vec1};
use nalgebra::adaptors::transform::Transform;
use nalgebra::adaptors::rotmat::Rotmat;
use nalgebra::vec::AlgebraicVec;
use ncollide::geom::{Ball, Plane, Box, Cylinder, Cone};
use integration::{BodyForceGenerator, RigidBodySmpEulerIntegrator};
use detection::collision::bodies_bodies::{Dispatcher, PairwiseDetector};
use detection::joint::joint_manager::JointManager;
use detection::joint::ball_in_socket::BallInSocket;
use detection::constraint::Constraint;
use detection::IslandActivationManager;
use resolution::AccumulatedImpulseSolver;
use object::{DefaultGeom, RigidBody, Body};
use world::World;
use object::volumetric::InertiaTensor;

type LV<N> = Vec2<N>;
type AV<N> = Vec1<N>;
type II<N> = Mat1<N>;
type M<N>  = Transform<Rotmat<Mat2<N>>, Vec2<N>>;

// fancier names
pub type Transform2d<N>       = M<N>;
pub type LinearVelocity2d<N>  = LV<N>;
pub type AngularVelocity2d<N> = AV<N>;
pub type InertiaTensor2d<N>   = II<N>;

pub type Ball2d<N>     = Ball<N>;
pub type Box2d<N>      = Box<N, Vec2<N>>;
pub type Cylinder2d<N> = Cylinder<N>;
pub type Cone2d<N>     = Cone<N>;
pub type Plane2d<N>    = Plane<N, Vec2<N>>;
pub type Geom2d<N>     = DefaultGeom<N, LV<N>, M<N>, II<N>>;

pub type ForceGenerator2d<N> = BodyForceGenerator<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type RigidBodyIntegrator2d<N> = RigidBodySmpEulerIntegrator<N, LV<N>, AV<N>, M<N>, II<N>>;

pub type Dispatcher2d<N> = Dispatcher<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type PairwiseDetector2d<N> = PairwiseDetector<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type JointManager2d<N> = JointManager<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type IslandActivationManager2d<N> = IslandActivationManager<N, LV<N>, AV<N>, M<N>, II<N>>; 

pub type ContactSolver2d<N> = AccumulatedImpulseSolver<N, LV<N>, AV<N>, M<N>, II<N>, Vec2<N>>;

pub type Constraint2d<N> = Constraint<N, LV<N>, AV<N>, M<N>, II<N>>;
pub type RigidBody2d<N> = RigidBody<N, LV<N>, AV<N>, M<N>, II<N>>; 
pub type Body2d<N> = Body<N, LV<N>, AV<N>, M<N>, II<N>>; 
pub type World2d<N> = World<N, Body2d<N>, Constraint2d<N>>;

/*
 * Joints
 */
pub type BallInSocket2d<N> = BallInSocket<N, LV<N>, AV<N>, M<N>, II<N>>;

/// NOTE: it is a bit unfortunate to have to specialize that for the raw types.
impl<N: Clone + Num + Algebraic, Any>
InertiaTensor<N, LV<N>, Any> for InertiaTensor2d<N> {
    #[inline]
    fn to_world_space(&self, _: &Any) -> InertiaTensor2d<N> {
        self.clone()
    }

    #[inline]
    fn to_relative_wrt_point(&self, mass: &N, pt: &LV<N>) -> InertiaTensor2d<N> {
        *self + Mat1::new(mass * pt.sqnorm())
    }
}
