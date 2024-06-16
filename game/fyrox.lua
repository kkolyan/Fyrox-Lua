---@meta

---@class Vector3
---@field x number
---@field y number
---@field z number
Vector3 = {}

--- The node script is attached to
---@class Script
---@field node Node
Script = {}

---@class RaycastHit
---@field node Node
RaycastHit = {}

---@class Message
---@field type string
---@field data table
Message = {}

---@class Node
---@field base Base
Node = {}

function Node:destroy_later() end

---@generic T
---@param class `T`
---@return T 
function Node:add_script(class) end

---@class Base
---@field local_transform Transform
Base = {}

---@class Transform
---@field local_position Vector3
Transform = {}

---@class Engine
Engine = {}

---@param position Vector3
---@param direction Vector3
---@return RaycastHit[]
function Engine:cast_ray(position, direction) end

---@param to Node
---@param message Message
function Engine:send_message(to, message) end

---@type Engine
engine = {}

---@class Camera
---@field enabled boolean?
---@field position Vector3?
Camera = {}

---@param params Camera
---@return Camera
function Camera:new(params) end