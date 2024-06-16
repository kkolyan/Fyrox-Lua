-- uuid = "9596c7ea-8751-423d-839a-5f6c8364223c"

local node: Node
local velocity: Vector3
local damage: number
local ttl_sec: number

function on_update(dt)
	local hits = engine:raycast(node.position, node.velocity:normalize(), velocity:magnitude() * dt)
	if #hits > 0 then
		engine:send_message(hits[1], {
			type = "Damage",
			amount = damage,
		})
		engine:destroy_later(node)
	else
		node.position += velocity * dt
	end
end