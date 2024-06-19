# Roadmap

## Initial Demo

### Goals
* prove that the idea of scripting language in Lua is worth efforts.
* examine the ground before attempt to onboard Fyrox with more advanced languages like C#.

### Tasks
- Lua should be able to address Script-owned values like Vector3 to edit it in-place. The obvious way to achieve it - rely on Script impls Reflect. But there is a problem: to make Script based expression node, we need some identifier of Script, but Script is owned by Node via the Vec. Vec index is a crappy way to refer, because it could be changed by removal of another script from the same node.
  1. the obvious, but complex solution is to hold script data in the pool and make Script instance hold a handle to the data. But there are obstacles:
     - Script constructor should be supplied via Plugin::register, where Plugin is available immutably.
     - Script constructor doesn't accept any context with reference to Plugin during creation
     - on the first callbacks where Plugin is available it's too late - visit and reflect are called before them.
  2. more sound solution is to convince mrDIMAS to store scripts in pools in engine. I don't think he will object if there is any physical reason, because he likes pools.

## Alpha

### Goals
* Allow non-Rust guys create games using Fyrox.

### Tasks
- Tool to generate Lua annotations automatically
- Consider a way to automatically bind methods to Lua

## Technical debt
- unify lua types registration (to add new types in one place)
- apply Fyrox file system convention to scripts location
