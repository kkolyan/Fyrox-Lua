use std::ops::Deref;
use mlua::{UserData, UserDataFields, Value};
use fyrox::core::pool::Handle;
use fyrox::core::reflect::{GetField, Reflect};
use fyrox::core::visitor::Visit;
use fyrox::scene::animation::absm::AnimationBlendingStateMachine;
use fyrox::scene::animation::AnimationPlayer;
use fyrox::scene::base::Base;
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
use fyrox::scene::node::{Node, NodeTrait};
use fyrox::scene::particle_system::ParticleSystem;
use fyrox::scene::pivot::Pivot;
use fyrox::scene::ragdoll::Ragdoll;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::scene::sound::listener::Listener;
use fyrox::scene::sound::Sound;
use fyrox::scene::sprite::Sprite;
use fyrox::scene::terrain::Terrain;
use crate::lua_utils::OptionX;
use crate::{SC_404, SCRIPT_CONTEXT};

/**
this version is bad, because it tries to apply reflection "statically", preparing things before and Fyrox Reflect doesn't fit here.
*/
pub fn demo() {
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

// we should be able to call Base::* fields by directly on this type from Lua
pub struct ScriptedNodeHandle {
    handle: Handle<Node>,
}

pub struct ScriptedNodeLazyExpression<T: Reflect + Default, const GET: fn(&mut dyn NodeTrait) -> T> {
    handle: Handle<Node>,
    get: &'static dyn Fn(&mut dyn NodeTrait) -> T,
    set: &'static dyn Fn(&mut dyn NodeTrait, T),
}

fn map_expression<A, B>(a: ScriptedNodeLazyExpression<A>, get: fn(A) -> B, set: fn(A, B)) -> ScriptedNodeLazyExpression<B>
    where A: Reflect + Default, B: Reflect + Default
{
    todo!()
}

impl UserData for ScriptedNodeHandle {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {}
}

impl<T: Reflect + Default> UserData for ScriptedNodeLazyExpression<T> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(lua_fields: &mut F) {
        let stub = T::default();
        stub.get_field()
        stub.fields_info(&mut |fields| {
            for field in fields {
                if !field.read_only {
                    lua_fields.add_field_method_set(field.name, |_lua, this, _value: Value| {
                        SCRIPT_CONTEXT.with(|ctx| {
                            let ctx = &mut ctx.borrow_mut();
                            let ctx = ctx.as_mut()
                                .lua_ok(SC_404)?;
                            let node = ctx.scene.graph.try_get_mut(this.handle)
                                .lua_ok("Node is not present on scene anymore")?;
                            field.value.

                            Ok(())
                        })
                    });
                }
                lua_fields.add_field_method_get(field.name, |_lua, this| {
                    SCRIPT_CONTEXT.with(|ctx| {
                        let ctx = &mut ctx.borrow_mut();
                        let ctx = ctx.as_mut()
                            .lua_ok(SC_404)?;
                        let node = ctx.scene.graph.try_get_mut(this.handle)
                            .lua_ok("Node is not present on scene anymore")?;
                        // this.get(node.);

                        Ok(42)
                    })
                });
            }
        });
    }
}

fn collect_info<T: Reflect + Visit + Default>(ctx: &mut Context, name_override: Option<&str>) {
    let stub = T::default();
    print_type("", name_override, &stub);
}

fn print_type<T: Reflect + ?Sized>(prefix: &str, name_override: Option<&str>, value: &T) {
    println!("{}type {}", prefix, name_override.unwrap_or(value.type_name().split("::").last().unwrap()));
    value.fields_info(&mut |fields| {
        for field in fields {
            println!("{}  field {} : {}{}", prefix, field.name, field.type_name, if field.read_only { " (readonly)" } else { "" });
            print_type(format!("  {}", prefix).as_str(), None, field.reflect_value);
        }
    });
}