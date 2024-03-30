use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         present_mode: PresentMode::AutoVsync,
        //         ..default()
        //     }),
        //     ..default()
        // }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // systems
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ball.after(setup))
        .add_systems(Update, use_actions)
        // resources
        // start
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    // movement
    Left,
    Right,
    // abilities
    Fire,
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert_one_to_many(Self::Left, [KeyCode::ArrowLeft, KeyCode::KeyA]);
        input_map.insert_one_to_many(Self::Right, [KeyCode::ArrowRight, KeyCode::KeyD]);
        input_map.insert(Self::Fire, KeyCode::Space);

        input_map
    }
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    // mut rapier_config: ResMut<RapierConfiguration>
) {
    // increase gravity
    // rapier_config.gravity = rapier_config.gravity * Vec2::new(0.0, 2.0);

    // configure and spawn the camera
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    // create the top
    commands
        .spawn(Collider::cuboid(640.0, 10.0))
        .insert(Name::new("Wall_Top"))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 350.0, 0.0)));

    // create the left wall
    commands
        .spawn(Collider::cuboid(10.0, 500.0))
        .insert(Name::new("Wall_Left"))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(-630.0, 0.0, 0.0)));

    // create the right wall
    commands
        .spawn(Collider::cuboid(10.0, 500.0))
        .insert(Name::new("Wall_Right"))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(630.0, 0.0, 0.0)));

    // // create the bottom
    // commands
    //     .spawn(Collider::cuboid(640.0, 10.0))
    //     .insert(Restitution::coefficient(1.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -350.0, 0.0)));

    // configure and spawn the player
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Name::new("Player"))
        .insert(KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            ..default()
        })
        .insert(Collider::cuboid(250.0, 10.0))
        .insert(GravityScale(0.0))
        .insert(Dominance::group(5))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(LockedAxes::TRANSLATION_LOCKED_Y)
        .insert(Friction::coefficient(1.0))
        .insert(Restitution::coefficient(1.1))
        .insert(ColliderMassProperties::Mass(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -300.0, 0.0)))
        .insert(InputManagerBundle::with_map(
            Action::default_input_map(),
        ))
        .insert(Player);
}

fn spawn_ball(mut commands: Commands) {
    // configure and spawn the ball
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Name::new("Ball"))
        .insert(Collider::ball(10.0))
        .insert(GravityScale(2.0))
        .insert(Ccd::enabled()) // TODO: is this needed?
        .insert(Dominance::group(0)) // default=0, but listed to be explicit
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(1.00))
        .insert(ColliderMassProperties::Mass(1000.0))        
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 300.0, 0.0)))
        .insert(ExternalForce {
            torque: 1.0,
            ..default()
        });
}

fn use_actions(
    commands: Commands,
    action_query: Query<&ActionState<Action>, With<Player>>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    let action_state = action_query.single();

    if action_state.pressed(&Action::Left) {
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(Vec2::new(-7.0, 0.0));
        }
    }

    if action_state.pressed(&Action::Right) {
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(Vec2::new(7.0, 0.0));
        }
    }

    if action_state.just_pressed(&Action::Fire) {
        spawn_ball(commands);
    }
}