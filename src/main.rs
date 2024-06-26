use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::{EnabledButtons, ExitCondition, WindowResolution};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::*;

const WINDOW_WIDTH: f32 = 512.0;
const WINDOW_HEIGHT: f32 = 768.0;

const HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
const HALF_WIDTH: f32 = WINDOW_WIDTH / 2.0;

const PADDLE_SPEED: f32 = 600.0;
const PADDLE_POSITION: Vec3 = Vec3::new(0.0, -HALF_HEIGHT * 0.86, 0.0);
const PADDLE_HEIGHT: f32 = 10.0;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    maximize: false,
                    ..default()
                },
                name: Some("BevyApp".to_string()),
                title: "BrickBreaker in Rust+Bevy".to_string(),
                ..default()
            }),
            exit_condition: ExitCondition::OnPrimaryClosed,
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // systems
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ball.after(setup))
        .add_systems(Update, handle_actions)
        .add_systems(Update, handle_collisions)
        .add_systems(Update, dev_tools_system)
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
        input_map.insert(Self::Left, GamepadButtonType::DPadLeft);

        input_map.insert_one_to_many(Self::Right, [KeyCode::ArrowRight, KeyCode::KeyD]);
        input_map.insert(Self::Right, GamepadButtonType::DPadRight);

        input_map.insert(Self::Fire, KeyCode::Space);
        input_map.insert(Self::Fire, GamepadButtonType::South);

        input_map
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

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
        .spawn(Collider::cuboid(HALF_WIDTH, 1.0))
        .insert(Name::new("Wall_Top"))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            HALF_HEIGHT,
            0.0,
        )));

    // create the left wall
    commands
        .spawn(Collider::cuboid(1.0, HALF_HEIGHT))
        .insert(Name::new("Wall_Left"))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            -HALF_WIDTH,
            0.0,
            0.0,
        )));

    // create the right wall
    commands
        .spawn(Collider::cuboid(1.0, HALF_HEIGHT))
        .insert(Name::new("Wall_Right"))
        .insert(Restitution::coefficient(1.0))
        .insert(Friction::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(
            HALF_WIDTH, 0.0, 0.0,
        )));

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
        .insert(Collider::cuboid(HALF_WIDTH / 2.5, PADDLE_HEIGHT))
        .insert(GravityScale(0.0))
        .insert(Dominance::group(5))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(LockedAxes::TRANSLATION_LOCKED_Y)
        .insert(Friction::coefficient(1.0))
        .insert(Restitution::coefficient(1.1))
        .insert(ColliderMassProperties::Mass(0.0))
        .insert(TransformBundle::from(Transform {
            translation: PADDLE_POSITION,
            ..default()
        }))
        .insert(InputManagerBundle::with_map(Action::default_input_map()))
        .insert(Player);
}

fn spawn_ball(mut commands: Commands) {
    let ball_position = PADDLE_POSITION + (Vec3::new(0.0, PADDLE_HEIGHT * 2.0, 0.0));

    // configure and spawn the ball
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Name::new("Ball"))
        .insert(Ball)
        .insert(Collider::ball(8.0))
        .insert(GravityScale(2.0))
        .insert(Ccd::enabled()) // TODO: is this needed?
        .insert(Dominance::group(0)) // default=0, but listed to be explicit
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(1.00))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(Sleeping {
            sleeping: true,
            ..default()
        })
        .insert(TransformBundle::from(Transform {
            translation: ball_position,
            ..default()
        }))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalImpulse::default());
}

fn handle_actions(
    time: Res<Time>,
    action_query: Query<&ActionState<Action>, With<Player>>,
    mut ball_query: Query<(&Sleeping, &mut ExternalImpulse), With<Ball>>,
    mut controllers: Query<&mut KinematicCharacterController>,
) {
    let action_state = action_query.single();

    if action_state.pressed(&Action::Left) {
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(Vec2::new(-PADDLE_SPEED * time.delta_seconds(), 0.0));
        }
    }

    if action_state.pressed(&Action::Right) {
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(Vec2::new(PADDLE_SPEED * time.delta_seconds(), 0.0));
        }
    }

    if action_state.just_pressed(&Action::Fire) {
        for query_result in ball_query.iter_mut() {
            let (sleeping, mut ext_impulse) = query_result;
            if sleeping.sleeping {
                ext_impulse.impulse = Vec2::new(0.0, 750.0);
            }
        }
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // ball_query: Query<&Ball>,
) {
    for collision_event in collision_events.read() {
        // TODO: this matches ANY collision, but should filter on entity type Ball
        if let CollisionEvent::Stopped(_e_collider, _e_self, _flags) = collision_event {
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/SFX_-_jump_03.ogg"),
                ..default()
            });
        }
    }
}

fn dev_tools_system(commands: Commands, mut contexts: EguiContexts) {
    egui::Window::new("Dev Tools").show(contexts.ctx_mut(), |ui| {
        if ui.button("Spawn Ball").clicked() {
            spawn_ball(commands);
        }
    });
}
