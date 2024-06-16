---@uuid 9596c7ea-8751-423d-839a-5f6c8364223c
---@class Bullet: Script
---@field velocity Vector3
---@field damage number
---@field ttl_sec number
---@field owner Node
local Bullet = {}
Bullet.__index = Bullet

function Bullet:on_update(dt)
	local hits = engine:raycast(self.node.local_transform.local_position, self.velocity:normalize(), self.velocity:magnitude() * dt)
	if #hits > 0 then
		engine:send_message(hits[1].node, {
			type = "Damage",
			data = {
				amount = self.damage
			},
		})
		self.node:destroy_later()
	else
		self.node.local_transform.local_position:add_assign(self.velocity * dt)
	end

	local b = Bullet:new({
		
	});

	local b = self.node:add_script("Bullet")
	local cam = Camera:new({

	})
end

return Bullet
