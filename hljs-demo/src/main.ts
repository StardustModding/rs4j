import "highlight.js/scss/default.scss";
import "highlight.js/scss/github-dark.scss";
import hljs from "highlight.js";
import rs4j from "@rs4j/hljs";

const text = `
class Collider {
    // =============== Normal methods ===============

    fn is_enabled() -> bool;
    fn is_sensor() -> bool;
    fn active_hooks() -> ActiveHooks;
    fn active_events() -> ActiveEvents;
    fn active_collision_types() -> ActiveCollisionTypes;
    fn friction() -> f32;
    fn friction_combine_rule() -> CoefficientCombineRule;
    fn restitution() -> f32;
    fn restitution_combine_rule() -> CoefficientCombineRule;
    fn position() -> Isometry<f32>;
    fn translation() -> Vector<f32>;
    fn rotation() -> Rotation<f32>;
    fn solver_groups() -> InteractionGroups;
    fn collision_groups() -> InteractionGroups;
    fn material() -> ColliderMaterial;
    fn volume() -> f32;
    fn density() -> f32;
    fn mass() -> f32;
    fn shape() -> Shape;
    fn shared_shape() -> SharedShape;
    fn compute_aabb() -> Aabb;
    fn compute_swept_aabb(next_pos: &Isometry<f32>) -> Aabb;
    fn mass_properties() -> MassProperties;
    fn contact_force_event_threshold() -> f32;

    // =============== Mutable methods ===============
    
    mut fn set_active_hooks(active_hooks: ActiveHooks);
    mut fn set_active_events(active_events: ActiveEvents);
    mut fn set_active_collision_types(active_collision_types: ActiveCollisionTypes);
    mut fn set_friction(coef: f32);
    mut fn set_friction_combine_rule(rule: CoefficientCombineRule);
    mut fn set_restitution(coef: f32);
    mut fn set_restitution_combine_rule(rule: CoefficientCombineRule);
    mut fn set_contact_force_event_threshold(threshold: f32);
    mut fn set_sensor(is_sensor: bool);
    mut fn set_enabled(enabled: bool);
    mut fn set_translation(translation: Vector<f32>);
    mut fn set_rotation(rotation: Rotation<f32>);
    mut fn set_position(position: Isometry<f32>);
    mut fn set_translation_wrt_parent(translation: Vector<f32>);
    mut fn set_rotation_wrt_parent(rotation: AngVector<f32>);
    mut fn set_position_wrt_parent(pos_wrt_parent: Isometry<f32>);
    mut fn set_collision_groups(groups: InteractionGroups);
    mut fn set_solver_groups(groups: InteractionGroups);
    mut fn set_density(density: f32);
    mut fn set_mass(mass: f32);
    mut fn set_mass_properties(mass_properties: MassProperties);
    mut fn shape_mut() -> Shape;
    mut fn set_shape(shape: SharedShape);

    // =============== Optional methods ===============

    fn parent() -> RigidBodyHandle;
    fn position_wrt_parent() -> Isometry<f32>;

    // =============== Trait methods ===============

    fn clone() -> Collider;
    mut fn clone_from(other: &Collider);
};
`.trim();

const el = document.createElement("pre");
const el2 = document.createElement("code");

hljs.registerLanguage("rs4j", rs4j);

el2.innerHTML = text;
el2.className = "language-rs4j";

el.appendChild(el2);
document.body.appendChild(el);

hljs.highlightAll();
