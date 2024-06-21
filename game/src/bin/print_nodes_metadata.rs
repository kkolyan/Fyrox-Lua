use fyrox::core::reflect::Reflect;
use fyrox::core::visitor::Visit;
use fyrox::scene::animation::absm::AnimationBlendingStateMachine;
use fyrox::scene::animation::AnimationPlayer;
use fyrox::scene::camera::Camera;
use fyrox::scene::collider::Collider;
use fyrox::scene::decal::Decal;
use fyrox::scene::dim2::collider::Collider as Collider2D;
use fyrox::scene::dim2::joint::Joint as Joint2D;
use fyrox::scene::dim2::rectangle::Rectangle;
use fyrox::scene::dim2::rigidbody::RigidBody as RigidBody2D;
use fyrox::scene::joint::Joint;
use fyrox::scene::light::directional::DirectionalLight;
use fyrox::scene::light::point::PointLight;
use fyrox::scene::light::spot::SpotLight;
use fyrox::scene::mesh::Mesh;
use fyrox::scene::navmesh::NavigationalMesh;
use fyrox::scene::node::Node;
use fyrox::scene::node::NodeTrait;
use fyrox::scene::particle_system::ParticleSystem;
use fyrox::scene::pivot::Pivot;
use fyrox::scene::ragdoll::Ragdoll;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::scene::sound::listener::Listener;
use fyrox::scene::sound::Sound;
use fyrox::scene::sprite::Sprite;
use fyrox::scene::terrain::Terrain;

fn main() {
    let mut ctx = Context {};
    collect_info::<AnimationBlendingStateMachine>(&mut ctx, None);
    collect_info::<AnimationPlayer>(&mut ctx, None);
    collect_info::<Camera>(&mut ctx, None);
    collect_info::<Collider>(&mut ctx, None);
    collect_info::<Collider2D>(&mut ctx, Some("Collider2D"));
    collect_info::<Decal>(&mut ctx, None);
    collect_info::<DirectionalLight>(&mut ctx, None);
    collect_info::<Joint>(&mut ctx, None);
    collect_info::<Joint2D>(&mut ctx, Some("Joint2D"));
    collect_info::<Listener>(&mut ctx, None);
    collect_info::<Mesh>(&mut ctx, None);
    collect_info::<NavigationalMesh>(&mut ctx, None);
    collect_info::<ParticleSystem>(&mut ctx, None);
    collect_info::<Pivot>(&mut ctx, None);
    collect_info::<PointLight>(&mut ctx, None);
    collect_info::<Ragdoll>(&mut ctx, None);
    collect_info::<Rectangle>(&mut ctx, None);
    collect_info::<RigidBody>(&mut ctx, None);
    collect_info::<RigidBody2D>(&mut ctx, Some("RigidBody2D"));
    collect_info::<Sound>(&mut ctx, None);
    collect_info::<SpotLight>(&mut ctx, None);
    collect_info::<Sprite>(&mut ctx, None);
    collect_info::<Terrain>(&mut ctx, None);
}

struct Context {}

fn collect_info<T: Reflect + Visit + Default>(ctx: &mut Context, name_override: Option<&str>) {
    let stub = T::default();
    print_type("", name_override, &stub);
}

fn print_type<T: Reflect + ?Sized>(prefix: &str, name_override: Option<&str>, value: &T) {
    println!(
        "{}type {}",
        prefix,
        name_override.unwrap_or(value.type_name())
    );
    value.as_array(&mut |array| {
        if let Some(array) = array {}
    });
    value.fields_info(&mut |fields| {
        for field in fields {
            println!(
                "{}  field {} : {}{}",
                prefix,
                field.name,
                field.type_name,
                if field.read_only { " (readonly)" } else { "" }
            );
            print_type(format!("  {}", prefix).as_str(), None, field.reflect_value);
        }
    });
}
